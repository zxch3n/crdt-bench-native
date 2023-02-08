import __ from "https://deno.land/x/dirname@1.1.2/mod.ts";
import { existsSync } from "https://deno.land/std@0.105.0/fs/exists.ts";
const { __dirname } = __(import.meta);
import { resolve } from "https://deno.land/std@0.105.0/path/mod.ts";

const CRITERION_DIR = resolve(__dirname, "../target/criterion");

interface Task {
  task: string;
  crate: string;
  time: number;
  timeStdErr: number;
}

interface EstimateReport {
  mean: {
    /**
     * nano second
     */
    point_estimate: number;
    /**
     * nano second
     */
    standard_error: number;
  };
}

async function readCrate(crate: string): Promise<Task[]> {
  const crateResultPath = resolve(CRITERION_DIR, crate);
  if (!existsSync(crateResultPath)) {
    throw new Error(
      "crate not found: " + crate +
        ". Run cargo bench first before generating report.",
    );
  }

  const tasks: Task[] = [];
  for await (const dirEntry of Deno.readDir(crateResultPath)) {
    if (dirEntry.isDirectory) {
      tasks.push(
        await readTask(resolve(crateResultPath, dirEntry.name), crate),
      );
    }
  }

  return tasks;
}

async function readTask(path: string, crate: string): Promise<Task> {
  const taskName: string =
    JSON.parse(await Deno.readTextFile(resolve(path, "new", "benchmark.json")))
      .full_id;

  const estimate: EstimateReport = JSON.parse(
    await Deno.readTextFile(resolve(path, "new", "estimates.json")),
  );

  return {
    crate,
    task: taskName.replace(/^.*?\//, ""),
    time: estimate.mean.point_estimate / 1000 / 1000,
    timeStdErr: estimate.mean.standard_error / 1000 / 1000,
  };
}

const CRATES = [
  "automerge",
  "loro",
  "diamond-type",
  "yrs",
] as const;

const results = await Promise.all(CRATES.map((x) => readCrate(x)));

interface Transported {
  [task: string]: {
    [crate: string]: {
      time: number;
      timeStdErr: number;
    };
  };
}

const report: Transported = {};
for (let i = 0; i < CRATES.length; i++) {
  for (const task of results[i]) {
    report[task.task] = report[task.task] || {};
    report[task.task][CRATES[i]] = report[task.task][CRATES[i]] || {};
    report[task.task][CRATES[i]].time = task.time;
    report[task.task][CRATES[i]].timeStdErr = task.timeStdErr;
  }
}

console.log(`| Tasks | ${CRATES.join(" | ")} |`);
console.log(`| :---- | ${CRATES.map(() => ":----").join(" | ")} |`);
const tasks = Object.keys(report);
tasks.sort();
for (const task of tasks) {
  console.log(`| ${task} | ${
    CRATES.map((x) => {
      const time = report[task][x].time;
      const timeStdErr = report[task][x].timeStdErr;
      if (time > 1) {
        return `${time.toFixed(2)} ± ${timeStdErr.toFixed(2)} ms`;
      } else {
        return `${(time * 1000).toFixed(2)} ± ${
          (timeStdErr * 1000).toFixed(2)
        } us`;
      }
    }).join(" | ")
  } |`);
}

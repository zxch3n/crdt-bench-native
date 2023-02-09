# Native CRDT benchmark

| Tasks                   | automerge        | loro            | diamond-type    | yrs                |
| :---------------------- | :--------------- | :-------------- | :-------------- | :----------------- |
| automerge - apply       | 343.93 ± 0.89 ms | 71.38 ± 0.09 ms | 19.67 ± 0.90 ms | 4239.71 ± 65.33 ms |
| automerge - decode time | 389.08 ± 1.17 ms | 1.10 ± 0.01 ms  | 1.92 ± 0.01 ms  | 4.42 ± 0.01 ms     |
| automerge - encode time | 17.65 ± 0.05 ms  | 3.28 ± 0.01 ms  | 1.17 ± 0.01 ms  | 558.97 ± 2.05 us   |
| concurrent list inserts | 61.07 ± 0.55 ms  | 81.77 ± 0.32 ms | Unknown         | 15.21 ± 0.04 ms    |
| list_random_insert_1k   | 304.50 ± 5.59 ms | 51.52 ± 0.29 ms | Unknown         | 5.78 ± 0.01 ms     |

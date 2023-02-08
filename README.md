# Native CRDT benchmark

| Tasks                   | automerge          | loro            | diamond-type    | yrs                |
| :---------------------- | :----------------- | :-------------- | :-------------- | :----------------- |
| automerge - apply       | 318.05 ± 3.35 ms   | 70.04 ± 0.13 ms | 19.20 ± 0.07 ms | 4019.50 ± 10.60 ms |
| automerge - decode time | 407.47 ± 1.73 ms   | 6.52 ± 0.01 ms  | 1.87 ± 0.00 ms  | 0.71 ± 0.00 us     |
| automerge - encode time | 18.19 ± 0.06 ms    | 6.71 ± 0.02 ms  | 1.13 ± 0.00 ms  | 0.29 ± 0.00 us     |
| concurrent list inserts | 107.29 ± 0.43 ms   | 55.84 ± 0.30 ms | 44.03 ± 0.17 ms | 8.98 ± 0.99 ms     |
| list_random_insert_1k   | 1432.86 ± 10.14 ms | 11.58 ± 0.15 ms | 27.24 ± 0.03 ms | 3.99 ± 0.01 ms     |

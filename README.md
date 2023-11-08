# Native CRDT benchmark

| Tasks                   | automerge        | loro             | diamond-type    | yrs                |
|:------------------------|:-----------------|:-----------------|:----------------|:-------------------|
| automerge - apply       | 344.73 ± 0.84 ms | 62.59 ± 0.20 ms  | 19.67 ± 0.90 ms | 4239.71 ± 65.33 ms |
| automerge - decode time | 394.37 ± 3.36 ms | 1.10 ± 0.006 ms  | 1.92 ± 0.01 ms  | 4.42 ± 0.01 ms     |
| automerge - encode time | 9.37 ± 0.02 ms   | 1.34 ± 0.005 ms  | 1.17 ± 0.01 ms  | 558.97 ± 2.05 us   |
| concurrent list inserts | 58.55 ± 0.23 ms  | 109.04 ± 0.49 ms | Unknown         | 15.21 ± 0.04 ms    |
| list_random_insert_1k   | 265.08 ± 0.77 ms | 8.78 ± 0.019 ms  | Unknown         | 5.78 ± 0.01 ms     |

# Symbolic GKAT Equivalence 
This repository implements symbolic
[GKAT](https://dl.acm.org/doi/10.1145/3371129) equivalence algorithms.

## Building
Requires the `cargo` build tool for the [Rust](https://www.rust-lang.org/)
programming language.

To build the equivalence checker:
``` sh
cargo build --release
```
The resulting executable can be found at `target/release/rust-gkat`.

## Usage
`rust-gkat` offers 2 kernels and 2 solver backends for checking equivalence of boolean expressions.

- kernel `k1`: symbolic derivative method (default)
``` sh
rust-gkat -k k1 ./input/test00.txt
```

- kernel `k2`: symbolic thompson's construction
``` sh
rust-gkat -k k2 ./input/test00.txt
```

- solver `bdd`: use Binary Decision Diagrams (CUDD) for boolean satisfiability checking (default)
``` sh
rust-gkat -s bdd ./input/test00.txt
```

- solver `sat`: use SAT solver (MiniSat2) for boolean satisfiability checking
``` sh
rust-gkat -s sat ./input/test00.txt
```

Kernels and solvers can be mixed freely.

## Input Format
Each input file consists of 3 s-expressions. The first 2 s-expressions are the
GKAT expressions for equivalence testing. The final `(equiv ...)` marks whether
these 2 expressions are expected to be equivalent or not.

Sample from `input/test10.txt`:
```
(seq
  (seq
    (seq
      (seq (seq p7 p10) p9) (if b66 (if (or b79 b78 b68) p8 p1) (seq p6 p1)))
    (seq (if (not b25) p5 p22) p3 p0 p7) (seq p6 p2)
    (if (and b60 b18 b99) (seq p3 p98) p6))
  (seq (seq p9 p8) p2) (seq p25 p5)
  (if (and (or b82 b42) (or b42 b82)) (if (and b52 b75) p6 p1) p20))

(seq
  (seq
    (seq
      (seq (seq p7 p10) p9)
      (if b66 (if (or (or b79 b78) b68) p8 p1) (seq p6 p1)))
    (seq (seq (if (not b25) p5 p22) (seq p3 p0) p7) p6 p2)
    (if (and (and b60 b18) b99) (seq p3 p98) p6))
  (seq (seq p9 p8) p2) (seq p25 p5)
  (if (and (and b52 b75) (or b82 b42)) p6
    (if (and (or b82 b42) (or b42 b82)) p1 p20)))

(equiv 1)
```

For n-ary syntax such as `(seq A B C)`, it is parsed right-associatively into
binary form as `(seq A (seq B C))`.

## Performance and Evaluation
### Benchmarks
We provide a set of benchmarks for evaluating the performance of `rust-gkat`.
These benchmarks follow a simple naming scheme describing the expression pairs
inside. For example, the benchmark `e250b5p10eq` contains expressions which have
approximately 250 primitive actions (`e250`), a maximum boolean expression size
of 5 (`b5`), 10 possible boolean variables (`p10`) and are known to be
equivalent (`eq`). Benchmarks with the suffix `ne` have expression pairs which
are known to be non-equivalent. 

One can use `make [dataset] kernel=[k1|k2] solver=[bdd|sat]` to run `rust-gkat`
on a particular dataset. For example, `make e250b5p10eq kernel=k1 solver=bdd`
runs `rust-gkat` on all expression pairs contained in dataset `e250b5p10eq`
using kernel `k1` and solver `bdd`.

### Results
We evaluate the performance of `rust-gkat` in terms of time and memory usage. We
also compare `rust-gkat` with a modified version of
[SymKAT](https://perso.ens-lyon.fr/damien.pous/symbolickat/) (sk) that allows
for checking larger expressions. The following table lists the total time and
peak memory used for each benchmark.

#### Benchmark Total Time Usage
| Benchmark      | Time (k1-bdd) | Time (k2-bdd) | Time (k1-sat) | Time (k2-sat) | Time (sk) |
| -------------- | ------------- | ------------- | ------------- | ------------- | --------- |
| e250b5p10ne    | 0.19s         | 0.18s         | 0.20s         | 0.23s         | 5.82s     |
| e250b5p10eq    | 0.21s         | 0.18s         | 0.18s         | 0.29s         | 2.83s     |
| e500b5p50ne    | 0.23s         | 0.22s         | 0.27s         | 0.39s         | 37.28s    |
| e500b5p50eq    | 0.26s         | 0.21s         | 0.28s         | 0.57s         | 14.06s    |
| e1000b10p100ne | 0.31s         | 0.37s         | 0.48s         | 0.85s         | timeout   |
| e1000b10p100eq | 0.37s         | 0.28s         | 0.44s         | 1.04s         | 77.83s    |
| e2000b20p200ne | 1.50s         | 2.75s         | 0.99s         | 2.01s         | timeout   |
| e2000b20p200eq | 3.34s         | 3.03s         | 0.67s         | 3.14s         | timeout   |
| e3000b30p200ne | 1.75s         | 26.38s        | 1.91s         | 4.46s         | timeout   |
| e3000b30p200eq | 17.86s        | 22.59s        | 1.09s         | 5.25s         | timeout   |
| degenerate     | 220.53s       | 232.63s       | 0.42s         | 0.93s         | timeout   |

#### Benchmark Peak Memory Usage
| Benchmark      | Mem (k1-bdd) | Mem (k2-bdd) | Mem (k1-sat) | Mem (k2-sat) | Memory (sk) |
| -------------- | ------------ | ------------ | ------------ | ------------ | ----------- |
| e250b5p10ne    | 15.36MB      | 14.76MB      | 6.81MB       | 7.02MB       | 114.06MB    |
| e250b5p10eq    | 15.56MB      | 14.95MB      | 7.06MB       | 7.25MB       | 100.48MB    |
| e500b5p50ne    | 16.26MB      | 15.49MB      | 7.02MB       | 7.25MB       | 524.89MB    |
| e500b5p50eq    | 17.97MB      | 15.54MB      | 7.07MB       | 7.27MB       | 546.914MB   |
| e1000b10p100ne | 18.21MB      | 21.41MB      | 8.22MB       | 7.99MB       | timeout     |
| e1000b10p100eq | 20.41MB      | 17.66MB      | 8.69MB       | 7.63MB       | 5822.66MB   |
| e2000b20p200ne | 239.71MB     | 283.45MB     | 13.33MB      | 11.92MB      | timeout     |
| e2000b20p200eq | 107.92MB     | 102.46MB     | 13.20MB      | 14.02MB      | timeout     |
| e3000b30p200ne | 112.59MB     | 1235.46MB    | 21.44MB      | 20.10MB      | timeout     |
| e3000b30p200eq | 245.92MB     | 228.85MB     | 21.41MB      | 18.15MB      | timeout     |
| degenerate     | 631.47MB     | 1229.30MB    | 19.62MB      | 18.43MB      | timeout     |
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
`rust-gkat` offers 2 equivalence checking kernels backed by 2 satisfiability solver backends.

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

- solver `sat`: use a SAT solver (MiniSat2) for boolean satisfiability checking
``` sh
rust-gkat -s sat ./input/test00.txt
```

Kernels and solvers can be mixed freely.

## Input Format
Each input file consists of 3 s-expressions. The first 2 s-expressions are the
GKAT expressions for equivalence testing. The final `(equiv ...)` marks whether
these 2 expressions are expected to be equivalent or not.

```
<const> ::= 0 | 1

<bexp> ::= <const> | identifier
         | ( and <bexp> {<bexp>}+ )
         | ( or  <bexp> {<bexp>}+ )
         | ( not <bexp> )

 <exp> ::= identifier 
         | ( test <bexp> )
         | ( seq <exp> {<exp>}+ )
         | ( if <bexp> <exp> <exp> )
         | ( while <bexp> <exp> )

<format> ::= <exp> <exp> ( equiv <const> )
```

For n-ary syntax such as `(and A B C)`, it is parsed right-associatively into
binary form as `(and A (and B C))`.

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

#### Benchmark Total Time Usage (seconds)
| Benchmark      | Time (k1-bdd) | Time (k2-bdd) | Time (k1-sat) | Time (k2-sat) | Time (sk) |
| -------------- | ------------- | ------------- | ------------- | ------------- | --------- |
| e250b5p10ne    | 0.18          | 0.18          | 0.16          | 0.17          | 5.82s     |
| e250b5p10eq    | 0.19          | 0.18          | 0.15          | 0.18          | 2.83s     |
| e500b5p50ne    | 0.21          | 0.21          | 0.21          | 0.22          | 37.28s    |
| e500b5p50eq    | 0.22          | 0.21          | 0.20          | 0.27          | 14.06s    |
| e1000b10p100ne | 0.26          | 0.28          | 0.34          | 0.38          | timeout   |
| e1000b10p100eq | 0.28          | 0.26          | 0.28          | 0.41          | 77.83s    |
| e2000b20p200ne | 1.32          | 2.60          | 0.68          | 0.79          | timeout   |
| e2000b20p200eq | 1.92          | 2.95          | 0.41          | 1.00          | timeout   |
| e3000b30p200ne | 1.57          | 24.86         | 1.30          | 1.60          | timeout   |
| e3000b30p200eq | 10.82         | 22.83         | 0.66          | 1.54          | timeout   |
| degenerate     | 99.48         | 228.16        | 0.24          | 0.30          | timeout   |

#### Benchmark Peak Memory Usage (megabytes)
| Benchmark      | Mem (k1-bdd) | Mem (k2-bdd) | Mem (k1-sat) | Mem (k2-sat) | Memory (sk) |
| -------------- | ------------ | ------------ | ------------ | ------------ | ----------- |
| e250b5p10ne    | 15.17        | 14.54        | 7.07         | 7.01         | 114.06MB    |
| e250b5p10eq    | 15.69        | 14.69        | 7.06         | 7.29         | 100.48MB    |
| e500b5p50ne    | 15.99        | 15.33        | 7.02         | 7.29         | 524.89MB    |
| e500b5p50eq    | 16.85        | 15.26        | 6.99         | 7.01         | 546.914MB   |
| e1000b10p100ne | 17.51        | 21.31        | 7.78         | 7.50         | timeout     |
| e1000b10p100eq | 19.52        | 17.48        | 8.17         | 7.04         | 5822.66MB   |
| e2000b20p200ne | 237.17       | 280.70       | 12.95        | 11.43        | timeout     |
| e2000b20p200eq | 107.61       | 102.18       | 12.76        | 13.25        | timeout     |
| e3000b30p200ne | 112.06       | 1232.24      | 21.00        | 19.30        | timeout     |
| e3000b30p200eq | 245.23       | 228.86       | 21.18        | 17.64        | timeout     |
| degenerate     | 632.70       | 1225.24      | 19.13        | 17.94        | timeout     |
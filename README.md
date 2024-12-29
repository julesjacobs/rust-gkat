# Symbolic GKAT Equivalence 
This repository implements Zhang's symbolic
[GKAT](https://dl.acm.org/doi/10.1145/3371129) equivalence algorithm.

## Building
Requires the `cargo` build tool for the [Rust](https://www.rust-lang.org/)
programming language.

To build the equivalence checker:
``` sh
cargo build --release
```
The resulting executable can be found at `target/release/rust-gkat`.

## Usage
`rust-gkat` offers 2 solver kernels for checking equivalence of boolean expressions.
- `k1`: symbolic derivative method (default mode)
``` sh
rust-gkat -k k1 ./input/test00.txt
```
- `k2`: symbolic thompson's construction
``` sh
rust-gkat -k k2 ./input/test00.txt
```

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

For n-ary syntax such `(seq A B C)`, it is parsed right-associatively into
binary form as `(seq A (seq B C))`.

## Performance and Evaluation
Currently, we have tested `rust-gkat` on all large GKAT pairs contained in
`dataset.zip`. One can also use `make k1` or `make k2` to run `rust-gkat` on
all examples in the dataset.

### Symbolic Derivative Method
Even for difficult examples such as `exp9000.txt`, we achieve a very competitive
runtime of `0.04s` and peak memory consumption of `15MB` with the symbolic derivative 
method.
```
➞  /usr/bin/time -l ./target/release/rust-gkat -k k1 dataset/exp9000.txt
equiv_expected = true
equiv_result   = true
        0.04 real         0.04 user         0.00 sys
            14532608  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                1004  page reclaims
                   3  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                  19  involuntary context switches
           626808277  instructions retired
           123665227  cycles elapsed
            13386016  peak memory footprint
```

### Symbolic Thompson's Construction
The algorithm for symbolic thompson's construction performs much better than
the derivative method due to algorithmic improvements. Examples provided in `dataset.zip`
are no longer useful as runtime benchmarks as each test solves in `0.00s`.
```
➞  /usr/bin/time -l ./target/release/rust-gkat -k k2 dataset/exp9000.txt
equiv_expected = true
equiv_result   = true
        0.00 real         0.00 user         0.00 sys
             6979584  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                 557  page reclaims
                   3  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                   3  involuntary context switches
            49674701  instructions retired
            14603881  cycles elapsed
             5751096  peak memory footprint
```

We provide a dataset `dataset_big.zip` containing even larger examples with expression size
of 1,000,000. These examples are far more challenging to solve. Use `make big` to run 
`rust-gkat` on all big examples.
```
➞  /usr/bin/time -l ./target/release/rust-gkat -k k2 dataset_big/big02.txt
equiv_expected = true
equiv_result   = true
      166.44 real       165.24 user         1.05 sys
          1316077568  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
               86351  page reclaims
                   3  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   1  voluntary context switches
               14087  involuntary context switches
       3236003182277  instructions retired
        685796237324  cycles elapsed
           859768944  peak memory footprint
```
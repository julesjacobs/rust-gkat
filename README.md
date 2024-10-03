# Symbolic GKAT Equivalence 
This repository implements Zhang's symbolic [GKAT](https://dl.acm.org/doi/10.1145/3371129) equivalence algorithm.

## Building
Requires the `cargo` build tool for the [Rust](https://www.rust-lang.org/) programming language.

To build the equivalence checker:
``` sh
cargo build --release
```
The resulting executable can be found at `target/release/rust-gkat`.

## Usage
`rust-gkat` can operate under 2 modes for checking equivalence of boolean expressions:
- `bdd`: binary decision diagrams 
``` sh
rust-gkat -m bdd ./input/test00.txt
```
- `sdd`: sentential decision diagrams
``` sh
rust-gkat -m sdd ./input/test00.txt
```

One can also run `make bdd` or `make sdd` to test `rust-gkat` on all sample inputs.

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

For n-ary syntax such `(seq A B C)`, it is parsed right-associatively into binary form as `(seq A (seq B C))`.

## Performance and Evaluation
Currently, we have tested `rust-gkat` on a large dataset of over 10000 GKAT pairs.
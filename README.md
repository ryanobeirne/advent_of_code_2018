# Advent of Code 2018

## <https://www.adventofcode.com>

I did Advent of Code this year while learning Rust. Each day is it's own crate. Most of the crates read puzzle input from stdin. To try it out:

```sh
cd day01
cargo run --release < input/input.txt
```

Some crates read puzzle input as arguments:

```sh
cd day11
cargo run --release -- 7347
```
# Advent of Code 2018

## <https://www.adventofcode.com>

I did Advent of Code this year while learning Rust. Each day is it's own bin. Most of the crates read puzzle input from stdin. To try it out:

```sh
cargo run --release --bin=day01 < input/day01.txt
```

Some bins read puzzle input as arguments:

```sh
cargo run --release --bin=day11 -- 7347
```
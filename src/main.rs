extern crate advent_of_code_2018;
use advent_of_code_2018::input;

fn day_1() {
    println!("Day 1");

    let lines = input::read(1);
    let vec_isize: Vec<isize> = lines.iter().map(|line|
        line.parse().unwrap()
    ).collect();

    let sum: isize = vec_isize.iter().sum();

    println!("Part 1: {}", sum);

    let mut new_vec: Vec<isize> = Vec::new();
    let mut val: isize = 0;

    while !new_vec.contains(&val) {
        for i in &vec_isize {
            if new_vec.contains(&val) {
                println!("Part 2: {}", val);
                break;
            }
            new_vec.push(val);
            val += i;
        }
    }
}

fn main() {
    day_1();
}
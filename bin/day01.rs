use std::io::{Read, stdin};

pub fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Problem reading from stdin!");

    let vec_isize = input.lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let part1 = part_1(&vec_isize);
    println!("Part 1: {}", part1);

    let part2 = part_2(&vec_isize);
    println!("Part 2: {}", part2);
}

pub fn part_1(vec_isize: &Vec<isize>) -> isize {
    vec_isize.iter().sum()
}

pub fn part_2(vec_isize: &Vec<isize>) -> isize {
    let mut new_vec: Vec<isize> = Vec::new();
    let mut val: isize = 0;

    while !new_vec.contains(&val) {
        for i in vec_isize {
            if new_vec.contains(&val) {
                break;
            }
            new_vec.push(val);
            val += i;
        }
    }

    val
}

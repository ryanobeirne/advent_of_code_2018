pub fn go() {
    part_1();
    part_2();
}

fn line_vec() -> Vec<isize> {
    let lines = super::input::read(1);
    let vec_isize: Vec<isize> = lines.iter().map(|line| line.parse().unwrap()).collect();
    vec_isize
}

fn part_1() {
    println!("Day 1");

    let vec_isize = line_vec();

    let sum: isize = vec_isize.iter().sum();

    println!("\tPart 1: {}", sum);
}

pub fn part_2() {
    let vec_isize = line_vec();

    let mut new_vec: Vec<isize> = Vec::new();
    let mut val: isize = 0;

    while !new_vec.contains(&val) {
        for i in &vec_isize {
            if new_vec.contains(&val) {
                println!("\tPart 2: {}", val);
                break;
            }
            new_vec.push(val);
            val += i;
        }
    }
}

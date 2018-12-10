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

fn day_2() {
    println!("Day 2");

    let lines = input::read(2);

    let mut count_2: usize = 0;
    let mut count_3: usize = 0;

    // Does the string contain exactly n characters
    fn count_chars(s: &String, n: isize) -> bool {
        let vec_char: Vec<char> = s.chars().collect();
        if vec_char.len() == 0 || n == 0 { return false }

        for character in &vec_char {
            let mut char_count = 0;

            for (index, character2) in vec_char.iter().enumerate() {
                if character == character2 { char_count += 1 }
                
                if index + 1  == vec_char.len() {
                    if char_count == n {
                        return true
                    } else {
                        char_count = 0
                    }
                }
            }

        }
        
        false
    }

    for id in lines {
        if count_chars(&id, 2) {
            count_2 += 1
        }
        if count_chars(&id, 3) {
            count_3 += 1
        }
    }

    let checksum = count_2 * count_3;

    println!("Checksum: {}", checksum);
}

fn main() {
    // day_1();
    day_2();
}
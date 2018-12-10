pub fn go() {
    part_1();
    part_2();
}

fn characterize(s: &String) -> Vec<char> {
    s.chars().collect()
}

fn part_1() {
    println!("Day 2");

    let lines = super::input::read(2);

    let mut count_2: usize = 0;
    let mut count_3: usize = 0;

    // Does the string contain exactly n characters
    fn count_chars(s: &String, n: isize) -> bool {
        let vec_char = characterize(s);
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

    println!("\tPart 1: {}", checksum);
}

fn part_2() {
    use std::fmt::Write;

    let lines = super::input::read(2);
    let mut diff_vec: Vec<&String> = Vec::new();

    for id in lines.iter() {
        let chars = characterize(&id);
        for id2 in lines.iter() {
            let chars2 = characterize(&id2);
            if chars == chars2 { continue }
            let mut diff_count = 0;

            for (index, c) in chars.iter().enumerate() {
                if c != &chars2[index] {
                    diff_count += 1;
                }
            }

            if diff_count == 1 {
                diff_vec.push(id);
            }
        }
    }

    let mut s = String::new();

    let char_vec1 = characterize(diff_vec[0]);
    let char_vec2 = characterize(diff_vec[1]);

    for (i, c) in char_vec1.iter().enumerate() {
        if c == &char_vec2[i] {
            write!(&mut s, "{}", c);
        }
    }

    println!("\tPart 2: {}", s);
}
pub fn go() {
    part_1();
    // part_2();
}

fn part_1() {
    println!("Day 2");

    let lines = super::input::read(2);

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

    println!("\tPart 1: {}", checksum);
}
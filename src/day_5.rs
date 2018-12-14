use super::*;

pub fn go() {
    println!("Day 5");
    part_1();
    part_2();
}

pub const UNITS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

fn react(string: &String) -> String {
    let mut new_string = String::new();

    let mut skip_next = false;
    for (index, byte) in string.bytes().enumerate() {
        if !skip_next {
            if index + 2 > string.len() { 
                new_string.push(byte as char);
                continue;
            } else {
                // eprintln!("{}: {}", index, &file_as_string[index..index + 2]);
                match &string[index .. index + 2] {
                    "Aa" | "Bb" | "Cc" | "Dd" | "Ee" | "Ff" | "Gg" | "Hh" |
                    "Ii" | "Jj" | "Kk" | "Ll" | "Mm" | "Nn" | "Oo" | "Pp" |
                    "Qq" | "Rr" | "Ss" | "Tt" | "Uu" | "Vv" | "Ww" | "Xx" |
                    "Yy" | "Zz" | "aA" | "bB" | "cC" | "dD" | "eE" | "fF" |
                    "gG" | "hH" | "iI" | "jJ" | "kK" | "lL" | "mM" | "nN" |
                    "oO" | "pP" | "qQ" | "rR" | "sS" | "tT" | "uU" | "vV" |
                    "wW" | "xX" | "yY" | "zZ" => skip_next = true,
                    _ => {
                        new_string.push(byte as char);
                        skip_next = false;
                    }
                }
            }
        } else {
            skip_next = false;
        }
    }

    new_string
}

pub fn react_loop(file_as_string: &String) -> String {
    let mut new_string = react(&file_as_string);
    let mut cmp_string = file_as_string.clone();

    while new_string != cmp_string {
        cmp_string = react(&new_string);
        new_string = react(&cmp_string);
    }

    new_string
}

pub fn remove_unit(string: &String, unit: char) -> String {
    let mut new_string = String::new();

    for character in string.chars() {
        if character.to_ascii_uppercase() != unit {
            new_string.push(character);
        }
    }

    new_string
}

pub fn part_1() -> usize {
    let file_as_string = input::read_to_string(5);

    let new_string = react_loop(&file_as_string);

    let answer = new_string.len();
    println!("\tPart 1: {}", answer);
    answer
}

pub fn part_2() -> usize {
    let file_as_string = input::read_to_string(5);

    let mut shortest_polymer: (char, usize) = ('a', std::usize::MAX);
    for unit in UNITS {
        let new_string = remove_unit(&file_as_string, *unit);
        let reacted_string = react_loop(&new_string);
        if reacted_string.len() < shortest_polymer.1 {
            shortest_polymer = (*unit, reacted_string.len())
        }
    }

    let answer = shortest_polymer.1;
    println!("\tPart 2: {}", answer);
    
    answer
}
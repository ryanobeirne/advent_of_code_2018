use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Problem reading from stdin!");

    let part1 = part_1(&input);
    println!("Part 1: {}", part1);

    let part2 = part_2(&input);
    println!("Part 2: {}", part2);
}

fn part_1(input: &String) -> usize {
    react_loop(input).len()
}

fn part_2(input: &String) -> usize {
    let mut shortest_polymer: (char, usize) = (char::default(), std::usize::MAX);
    for unit in UNITS {
        let new_string = remove_unit(input, *unit);
        let reacted_string = react_loop(&new_string);
        if reacted_string.len() < shortest_polymer.1 {
            shortest_polymer = (*unit, reacted_string.len())
        }
    }

    shortest_polymer.1
}

const UNITS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

fn react(string: &String) -> String {
    let len = string.len();
    let mut new_string = String::new();

    let mut skip_next = false;
    for (index, ch) in string.chars().enumerate() {
        if !skip_next {
            if index + 2 > len { 
                new_string.push(ch);
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
                        new_string.push(ch);
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

fn react_loop(file_as_string: &String) -> String {
    let mut new_string = react(&file_as_string);
    let mut cmp_string = file_as_string.clone();

    while new_string != cmp_string {
        cmp_string = react(&new_string);
        new_string = react(&cmp_string);
    }

    new_string
}

fn remove_unit(string: &String, unit: char) -> String {
    let mut new_string = String::new();

    for character in string.chars() {
        if character.to_ascii_uppercase() != unit {
            new_string.push(character);
        }
    }

    new_string
}

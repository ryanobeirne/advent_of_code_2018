use super::*;

pub fn go() {
    println!("Day 5");
    part_1();
}

fn react(string: &String) -> String {
    let mut newstring = String::new();

    let mut skip_next = false;
    for (index, byte) in string.bytes().enumerate() {
        if !skip_next {
            if index + 2 > string.len() { 
                newstring.push(byte as char);
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
                        newstring.push(byte as char);
                        skip_next = false;
                    }
                }
            }
        } else {
            skip_next = false;
        }
    }

    newstring
}

pub fn part_1() -> usize {
    let file_as_string = input::read_to_string(5);

    let mut new_string = react(&file_as_string);
    let mut cmp_string = file_as_string;

    while new_string != cmp_string {
        cmp_string = react(&new_string);
        new_string = react(&cmp_string);
    }

    let answer = new_string.len();
    println!("\tPart_1: {}", answer);
    answer
}
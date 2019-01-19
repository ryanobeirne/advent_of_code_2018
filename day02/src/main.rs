use std::io::{Read, stdin};
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Problem reading from stdin!");

    let lines = input.lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let part1 = part1(&lines);
    println!("Part 1: {}", part1);

    let part2 = part2(&lines);
    println!("Part 2: {}", part2);
}

pub fn part1(lines: &Vec<String>) -> usize {
    let mut count_2: usize = 0;
    let mut count_3: usize = 0;

    for id in lines {
        let mut tracker = (false, false);
        for c in id.chars() {
            let c_count = id.chars()
                .filter(|c2| c == *c2)
                .count();

            if c_count == 2 {
                tracker.0 = true;
            }

            if c_count == 3 {
                tracker.1 = true;
            }
        }

        if tracker.0 == true {
            count_2 += 1;
        }

        if tracker.1 == true {
            count_3 += 1;
        }
    }

    count_2 * count_3
}

fn diff_count(s1: &str, s2: &str) -> usize {
    let btree1 = BTreeMap::from_iter(s1.char_indices());
    let btree2 = BTreeMap::from_iter(s2.char_indices());
    
    btree1.into_iter()
        .filter(|(i, c)|
            c != btree2.get(i).expect("&str's have different lengths")
        ).count()
}

#[test]
fn diff() {
    let s1 = "abcde";
    let s2 = "axcye";
    assert_eq!(diff_count(s1, s2), 2);
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut diff_vec: Vec<&str> = Vec::new();

    for id1 in lines {
        for id2 in lines {
            if diff_count(id1, id2) == 1 {
                diff_vec.push(id1);
            }
        }
    }

    common_chars(diff_vec[0], diff_vec[1])
}

fn common_chars(s1: &str, s2: &str) -> String {
    let btree1 = BTreeMap::from_iter(s1.char_indices());
    let btree2 = BTreeMap::from_iter(s2.char_indices());

    btree1.into_iter()
        .filter(|(i, c)| c == btree2.get(i).expect("&str's have diffrent lengths!"))
        .map(|(_, c)| c)
        .collect()
}

#[test]
fn common() {
    let s1 = "abcde";
    let s2 = "axcye";
    assert_eq!(common_chars(s1, s2), "ace");
}
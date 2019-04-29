use std::collections::BTreeMap;
use std::io::Read; 
use std::result;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

type Result<T> = result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let instructions: Vec<Instruction> = input.lines()
        .map(|line| Instruction::from_str(line))
        .filter_map(|o| o.ok())
        .collect();


    let answer1 = part1(instructions);
    println!("Day 7, Part 1:\t{}", answer1);
    if answer1 != "LAPFCRGHVZOTKWENBXIMSUDJQY" {
        println!("WRONG! Answer:\tLAPFCRGHVZOTKWENBXIMSUDJQY");
        std::process::exit(1);
    }

    Ok(())
}

fn part1(instructions: Vec<Instruction>) -> String {
    let mut order = Vec::<Name>::new();
    // let mut order_queue =  Vec::<Vec<Name>>::new();

    let prereq_map = Instruction::prereq_map(instructions);
    let mut status_map = StatusMap::new(&prereq_map);

    // Get initial instructions without prerequisites
    for prereqs in prereq_map.values() {
        for p in prereqs {
            if ! prereq_map.contains_key(&p) && ! order.contains(&p)  {
                status_map.mark_done(p);
                order.push(*p);
            }
        }
    }

    for (key, value) in &prereq_map {
        println!("{}:\n{:?}\n", key, value);
    }

    assert!( ! prereq_map.values().all(|v| v.is_empty()) );

    while status_map.is_not_done() {
        for (name, prereqs) in &prereq_map {
            if status_map.satisfied(prereqs) && !order.contains(name){
                status_map.mark_done(name);
                order.push(*name);
            }
        }
    }

    order.iter().collect()
}

type Name = char;
type PreReq = char;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Instruction {
    name: Name,
    prereq: PreReq,
}

type PrereqMap = BTreeMap<Name, Vec<PreReq>>;

impl Instruction {
    fn prereq_map(instructions: Vec<Instruction>) -> PrereqMap {
        let mut prereq_map = PrereqMap::new();

        for i in instructions {
            prereq_map
                .entry(i.name)
                .or_insert(Vec::new())
                .push(i.prereq);
        }

        for prereqs in prereq_map.values_mut() {
            prereqs.sort();
        }

        prereq_map
    }
}

impl FromStr for Instruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Instruction> {
        let split = s.split_whitespace().collect::<Vec<&str>>();
        let name = split[7].chars().collect::<Vec<char>>()[0];
        let prereq = split[1].chars().collect::<Vec<char>>()[0];
        
        Ok(
            Instruction { name, prereq }
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Progress {
    NotStarted,
    // InProgress,
    Done,
}

#[derive(Debug)]
struct StatusMap(BTreeMap<Name, Progress>);

impl StatusMap {
    fn new(prereq_map: &PrereqMap) -> StatusMap {
        let mut status_map = BTreeMap::new();
        
        for (name, prereqs) in prereq_map {
            status_map.insert(*name, Progress::NotStarted);
            for p in prereqs {
                status_map.insert(*p, Progress::NotStarted);
            }
        }

        StatusMap(status_map)
    }

    fn mark_done(&mut self, name: &Name) {
        if self.0.contains_key(name) {
            self.0.insert(*name, Progress::Done);
        } else {
            panic!("Status map does not contain key '{}'", name);
        }
    }

    fn is_done(&self) -> bool {
        self.0.values().all(|s| *s == Progress::Done)
    }

    fn is_not_done(&self) -> bool {
        !self.is_done()
    }

    fn did(&self, name: &Name) -> bool {
        self.0.contains_key(name) &&
        *self.0.get(name)
            .expect("StatusMap does not contain key. Cannot check if done")
            == Progress::Done
    }

    #[allow(dead_code)]
    fn did_not_do(&self, name: &Name) -> bool {
        !self.did(name)
    }

    fn satisfied(&self, names: &Vec<PreReq>) -> bool {
        names.iter().all(|n| self.did(n))
    }

    #[allow(dead_code)]
    fn unsatisfied(&self, names: &Vec<PreReq>) -> bool {
        !self.satisfied(names)
    }
}

impl fmt::Display for StatusMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut format = String::new();
        for (key, value) in &self.0 {
            format.push_str(format!("\t{}: {:?}\n", key, value).as_str());
        }
        write!(f, "\nStatusMap{{\n{}}}\n", format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_done() {
        let prereqs = &vec!['A', 'B', 'C'];
        let mut status_map = StatusMap(BTreeMap::new());
        status_map.0.insert('A', Progress::Done);
        status_map.0.insert('B', Progress::Done);
        status_map.0.insert('C', Progress::Done);

        assert!(status_map.is_done());
        assert!(status_map.satisfied(prereqs));

        let prereqs = &vec!['1', '2', '3'];
        let mut status_map = StatusMap(BTreeMap::new());
        status_map.0.insert('1', Progress::Done);
        status_map.0.insert('2', Progress::NotStarted);
        status_map.0.insert('3', Progress::Done);

        assert!(status_map.unsatisfied(prereqs));
        assert!(status_map.is_not_done());

        status_map.mark_done(&'2');

        assert!(status_map.satisfied(prereqs));
        assert!(status_map.is_done());
    }
}
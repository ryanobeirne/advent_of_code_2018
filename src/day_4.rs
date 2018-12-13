pub use super::*;
use std::collections::BTreeMap;

pub fn go() {
    println!("Day 4");
    part_1();
}

const DATE_FMT: &str = "%Y-%m-%d %H:%M";

fn parse_date_time(s: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, DATE_FMT).unwrap()
}

fn parse_dt_string(line: &String) -> NaiveDateTime {
    parse_date_time(&parse_btw_brackets(line))
}

#[derive(Debug, Clone)]
pub struct ShiftLog(Vec<String>);

impl ShiftLog {
    pub fn to_shift(&self) -> Shift {
        let guard_split: Vec<&str> = self.0[0].split_whitespace().collect();
        let guard = *&guard_split[3]
            .trim_left_matches('#')
            .parse::<usize>()
            .unwrap_or(0);

        let start = parse_dt_string(&self.0[0]);
        let end = parse_dt_string(&self.0.last().unwrap());

        let nap_vec: Vec<&String> = self.0.iter().filter(|line| 
            line.contains("asleep") || line.contains("wake")
        ).collect();

        let mut naps: Vec<Nap> = Vec::new();
        for (index, nap_time) in nap_vec.iter().enumerate() {
            if index % 2 == 0 {
                let start = parse_dt_string(&nap_time);
                let end = parse_dt_string(&nap_vec[index + 1]);
                naps.push(Nap { start, end });
            }
        }

        // println!("GUARD:\t{}", guard);
        // println!("START:\t{}", start);
        // for nap in &naps {
        //     println!("NAP:\t{:?}", nap);
        //     println!("\tDURATION:\t{} min", nap.duration());
        // }
        // println!("END:\t{}\n", end);

        Shift {
            guard, start, end, naps
        }
    }
}

#[derive(Debug, Clone)]
pub struct Log(Vec<ShiftLog>);

named!(in_brackets, delimited!(char!('['), is_not!("]"), char!(']')));

fn parse_btw_brackets(line: &String) -> String {
    std::str::from_utf8(
        in_brackets(line.as_bytes())
        .unwrap_or((&[0u8], &[0u8])).1
    ).unwrap_or("")
        .to_string()
}

impl Log {
    pub fn build() -> Self {
        let mut log = Log(Vec::new());
        let mut lines = input::read(4);
        lines.sort();

        named!(in_brackets, delimited!(char!('['), is_not!("]"), char!(']')));

        let mut shift_log = ShiftLog(Vec::new());
        for line in lines.iter() {
            if line == &lines[0] {
                shift_log.0.push(line.clone());
            } else if !line.contains("begins shift") { 
                shift_log.0.push(line.clone());
            } else {
                shift_log.0.push(line.clone());
                log.0.push(shift_log.clone());
                shift_log.0.clear();
                shift_log.0.push(line.clone());
            }
        }

        log
    }
}

#[derive(Debug)]
pub struct GuardDuty(Vec<Shift>);

impl GuardDuty {
    pub fn from_log() -> Self {
        let log = Log::build();
        GuardDuty(log.0.iter().map(|l| l.to_shift()).collect())
    }

    pub fn sleepiest(&self) -> usize {
        let mut guard_hash: BTreeMap<usize, i64> = BTreeMap::new();

        for shift in &self.0 {
            let guard = shift.guard;
            let mut nap_sum = 0;
            for nap in &shift.naps {
                nap_sum += nap.duration();
            }
            nap_sum += guard_hash.get(&guard).unwrap_or(&0_i64);
            guard_hash.insert(guard, nap_sum);
        }

        let mut biggest = (0, 0);
        for (guard, naptime) in guard_hash {
            if naptime > biggest.1 {
                biggest = (guard, naptime);
            }
        }

        println!("Sleepiest Guard #{}: {} minutes", biggest.0, biggest.1);

        biggest.0
    }
}

#[derive(Debug, Clone)]
pub struct Shift {
    pub guard: usize,
    pub start: NaiveDateTime,
    pub end:   NaiveDateTime,
    pub naps:  Vec<Nap>,
}

impl Shift {
    pub fn new() -> Self {
        Self {
            guard: 0,
            start: NaiveDateTime::from_timestamp(0, 0),
            end:   NaiveDateTime::from_timestamp(0, 0),
            naps:  Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Nap {
    pub start: NaiveDateTime,
    pub end:   NaiveDateTime,
}

impl Nap {
    pub fn duration(&self) -> i64 {
        let duration = self.end - self.start;
        duration.num_minutes()
    }
}

pub fn part_1() {
    let mut lines = super::input::read(4);
    lines.sort();

    let sleepy_guard = GuardDuty::from_log().sleepiest();

}
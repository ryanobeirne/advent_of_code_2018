pub use super::*;
use std::collections::BTreeMap;

pub fn go() {
    println!("Day 4");
    part_1();
    part_2();
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

    pub fn sleepiest_guard(&self) -> usize {
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

        // println!("Sleepiest Guard #{}: {} minutes", biggest.0, biggest.1);
        biggest.0
    }

    pub fn sleepiest_minute(&self) -> u32 {
        let mut stack: BTreeMap<u32, usize> = BTreeMap::new();

        for shift in &self.0 {
            for nap in &shift.naps {
                let start = nap.start.minute();
                let end = nap.end.minute();
                for minute in 0..60 {
                    if minute >= start && minute < end {
                        let count = stack.get(&minute).unwrap_or(&0).clone();
                        stack.insert(minute, count + 1);
                    }
                }
            }
        }

        let mut biggest = (0_u32, 0_usize);
        for (minute, count) in &stack {
            if *count > biggest.1 {
                biggest = (*minute, *count);
            }
        }

        // println!("Sleepiest Minute: {}", biggest.0);
        biggest.0
    }

    pub fn guard_most_asleep_on_minute(&self, minute: u32) -> usize {
        let mut guards: BTreeMap<usize, usize> = BTreeMap::new();

        for shift in &self.0 {
            for nap in &shift.naps {
                if nap.contains_minute(minute) {
                    let count = guards.get(&shift.guard).unwrap_or(&0_usize) + 1;
                    guards.insert(shift.guard, count);
                }
            }
        }

        let mut biggest: (usize, usize) = (0, 0);
        for (guard, count) in guards {
            if count > biggest.1 {
                biggest = (guard, count);
            }
        }

        // println!("GUARD #{}: {}", biggest.0, biggest.1);
        
        biggest.0
    }

    pub fn by_guard(&self, guard: usize) -> Self {
        let mut guard_duty = GuardDuty(Vec::new());

        for shift in &self.0 {
            if shift.guard == guard {
                guard_duty.0.push(shift.clone());
            }
        }

        guard_duty
    }

    pub fn guard_map_minutes(&self) -> BTreeMap<usize, Vec<u32>> {
        let mut guard_map: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
        let dummy: Vec<u32> = Vec::new();

        for shift in &self.0 {
            let mut minute_vec = guard_map.get(&shift.guard).unwrap_or(&dummy).clone();
            for nap in &shift.naps {
                for minute in &nap.minutes(){
                    minute_vec.push(*minute);
                }
            }
            guard_map.insert(shift.guard, minute_vec);
        }

        guard_map
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

    pub fn minutes(&self) -> Vec<u32> {
        let mut minutes: Vec<u32> = Vec::new();        

        for m in 0..60 {
            if self.contains_minute(m) {
                minutes.push(m);
            }
        }

        minutes
    }

    pub fn contains_minute(&self, minute: u32) -> bool {
        minute >= self.start.minute() && minute < self.end.minute()
    }

    pub fn print_timeline(&self) {
        let mut timeline = String::new();

        for minute in 0..60 {
            if minute >= self.start.minute() && minute < self.end.minute(){
                timeline.push('#');
            } else {
                timeline.push('.');
            }
        }

        println!("{}", timeline);
    }
}

fn mode_minute(nums: Vec<u32>) -> (u32, usize) {
    let mut minute_count: BTreeMap<u32, usize> = BTreeMap::new();

    for minute in nums {
        if minute_count.contains_key(&minute) {
            let count = minute_count.get(&minute).unwrap() + 1;
            minute_count.insert(minute, count);
        } else {
            minute_count.insert(minute, 1);
        }
    }

    let mut most = (0, 0);
    for (minute, count) in minute_count {
        if count > most.1 {
            most = (minute, count);
        }
    }

    most
}

pub fn part_1() -> usize {
    let guard_duty = GuardDuty::from_log();
    let sleepy_guard = guard_duty.sleepiest_guard();

    let sleepy_guard_duty = guard_duty.by_guard(sleepy_guard);
    let sleepy_minute = sleepy_guard_duty.sleepiest_minute();

    let answer = sleepy_guard * sleepy_minute as usize;
    println!("\tPart 1: {}", answer);
    answer    
}

pub fn part_2() -> usize {
    let guard_duty = GuardDuty::from_log();
    let guard_map = guard_duty.guard_map_minutes();

    let mut guard_minute: Vec<(usize, (u32, usize))> = Vec::new();
    for (guard, minutes) in &guard_map {
        guard_minute.push( (*guard, mode_minute(minutes.clone())) );
    }

    let mut most = (0, (0, 0));
    for (guard, (minute, count)) in guard_minute {
        if count > (most.1).1 {
            most = (guard, (minute, count));
        }
    }

    let (guard, (minute, _)) = most;

    let answer = guard * minute as usize;
    println!("\tPart 2: {}", answer);
    answer
}
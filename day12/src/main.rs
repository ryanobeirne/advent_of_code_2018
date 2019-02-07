use std::str::FromStr;
use std::fmt;
use std::io::{self, Read, stdin};
use std::collections::BTreeMap;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let state = State::from_input(&input)?;
    let rules = Rule::rules_from_input(&input)?;

    let part1 = part_1(&mut state.clone(), &rules, 20);
    println!("Part 1: {}", part1);

    let part2 = part_1(&mut state.clone(), &rules, 50_000_000_000);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part_1(state: &mut State, rules: &Rules, generations: usize) -> i32 {
    for gen in 0..generations {
        if gen % 1000 == 0 && gen != 0 {
            println!(
                "{} Generations\tMin: {}\tMax: {}\tSum: {}",
                gen, state.min(), state.max(), state.sum()
            );
        }
        state.advance(rules);
    }

    state.sum()
}
 
#[derive(Debug, Clone, PartialOrd, Ord, Eq, PartialEq)]
struct Plant;

#[derive(Debug, Clone, PartialOrd, Ord, Eq, PartialEq)]
struct Pot(Option<Plant>);

#[derive(Clone)]
struct State {
    pots: BTreeMap<i32, Pot>,
}

impl State {
    fn new() -> State {
        State { pots: BTreeMap::new() }
    }

    fn from_input(input: &str) -> io::Result<State> {
        if let Some(line) = input.lines().nth(0) {
            Ok(State::from_str( line.trim_start_matches("initial state: "))?)
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    fn advance(&mut self, rules: &Rules) {
        let mut advanced_state = State::new();

        let min = self.min();
        let max = self.max();

        self.bump(&min, &max);

        for (index, _pot) in self.pots.iter() {
            let config = self.get_config(index);
            let new_pot = config.transform(&rules);
            advanced_state.pots.insert(*index, new_pot);
        }

        self.pots = advanced_state.pots;
    }

    fn bump(&mut self, min: &i32, max: &i32) {
        let first_two = self.first_two();
        let last_two  = self.last_two();

        match first_two {
            [Some(_), Some(_)] | [Some(_), None] => {
                self.pots.insert(min - 1, Pot(None));
                self.pots.insert(min - 2, Pot(None));
            },
            [None, Some(_)] => {
                self.pots.insert(min - 1, Pot(None));
            },
            [None, None] => {
                // self.pots.remove(min);
                // self.pots.remove(&(min + 1));
            },
        }

        match last_two {
            [Some(_), Some(_)] | [None, Some(_)] => {
                self.pots.insert(max + 1, Pot(None));
                self.pots.insert(max + 2, Pot(None));
            },
            [Some(_), None] => {
                self.pots.insert(max + 1, Pot(None));
            },
            [None, None] => {
                // self.pots.remove(max);
                // self.pots.remove(&(max - 1));
            },
        }
    }

    fn first_two(&self) -> [Option<Plant>; 2] {
        let config = self.get_config(&self.min()).0;

        [config[2].0.clone(), config[3].0.clone()]
    }

    fn last_two(&self) -> [Option<Plant>; 2] {
        let config = self.get_config(&self.max()).0;

        [config[1].0.clone(), config[2].0.clone()]
    }

    fn sum(&self) -> i32 {
        self.pots.iter()
            .filter(|(_index, pot)| pot.0.is_some())
            .map(|(index, _pot)| index)
            .sum()
    }

    fn max(&self) -> i32 {
        *self.pots.keys().max().expect("State is empty!")
    }

    fn min(&self) -> i32 {
        *self.pots.keys().min().expect("State is empty!")
    }

    fn get_config(&self, index: &i32) -> Config {
        let center = self.pots.get(index)
            .expect("Don't call `State::get_config()` unless you know the index exists!")
            .clone();

        let left2 = self.edge_or_create(index - 2);
        let left1 = self.edge_or_create(index - 1);
        let right1 = self.edge_or_create(index + 1);
        let right2 = self.edge_or_create(index + 2);

        Config([left2, left1, center, right1, right2])
    }

    fn edge_or_create(&self, index: i32) -> Pot {
        if let Some(pot) = self.pots.get(&index) {
            pot.clone()
        } else {
            Pot(None)
        }
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
struct Config([Pot; 5]);

impl Config {
    fn transform(&self, rules: &Rules) -> Pot {
        if rules.is_empty() {
            panic!("No rules! Anarchy!");
        }
        rules.get(self).unwrap_or(&Pot(None)).clone()
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
struct Rule {
    config: Config,
    result: Pot,
}

type Rules = BTreeMap<Config, Pot>;

impl Rule {
    fn rules_from_input(input: &str) -> Result<Rules> {
        let mut ruleset = Rules::new();
        let lines = input.lines().skip(2).collect::<Vec<&str>>();

        for line in &lines {
            let rule = Rule::from_str(line)?;
            ruleset.insert(rule.config, rule.result);
        }

        Ok(ruleset)
    }
}

impl FromStr for Pot {
    type Err = io::Error;
    fn from_str(s: &str) -> io::Result<Pot> {
        match s {
            "#" => Ok(Pot(Some(Plant))),
            "." => Ok(Pot(None)),
            _   => Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}

impl fmt::Display for Pot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match self.0 {
                Some(_) => "#",
                None    => ".",
            }
        )
    }
}

impl FromStr for State {
    type Err = io::Error;
    fn from_str(s: &str) -> io::Result<State> {
        let mut pots = BTreeMap::new();

        for (index, c) in s.chars().enumerate() {
            let pot = Pot::from_str(&c.to_string())?;
            pots.insert(index as i32, pot);
        }

        Ok(State { pots })
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State{{\t{}}}",
            self.pots.iter()
                .map(|(index, pot)| format!("{}[{}]\t", index, pot))
                .collect::<String>()
        )
    }
}

impl FromStr for Config {
    type Err = io::Error;

    fn from_str(s: &str) -> io::Result<Config> {
        match s.len() {
            5 => Ok(
               Config([
                   Pot::from_str(&s.chars().nth(0).unwrap().to_string())?,
                   Pot::from_str(&s.chars().nth(1).unwrap().to_string())?,
                   Pot::from_str(&s.chars().nth(2).unwrap().to_string())?,
                   Pot::from_str(&s.chars().nth(3).unwrap().to_string())?,
                   Pot::from_str(&s.chars().nth(4).unwrap().to_string())?,
               ])
            ),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}

impl FromStr for Rule {
    type Err = io::Error;

    fn from_str(s: &str) -> io::Result<Rule> {
        let split = s.split(" => ").collect::<Vec<&str>>();
        match split.len() {
            2 => Ok(Rule {
                config: Config::from_str(split[0])?,
                result: Pot::from_str(split[1])?,
            }),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}",
            self.config.0.iter()
                .map(|p| p.to_string())
                .collect::<String>(),
            self.result
        )
    }
}
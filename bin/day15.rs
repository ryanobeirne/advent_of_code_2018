use std::collections::BTreeMap;
use std::str::FromStr;
use std::fmt;
use std::io::{Read, Write, stdout, stdin};

use CreatureType::{Elf, Goblin};
use Feature::{Unit, Wall};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let input = &mut String::new();
    stdin().read_to_string(input)?;

    let map = Map::from_str(&input)?;

    writeln!(stdout(), "{}", map)?;

    Ok(())
}

#[derive(Debug)]
struct Map {
    layout: BTreeMap<Point, Option<Feature>>
}

#[derive(Debug)]
enum CreatureType {
    Elf,
    Goblin,
}

impl CreatureType {
    fn enemy(&self) -> CreatureType {
        match self {
            Elf => Goblin,
            Goblin => Elf,
        }
    }
}

#[derive(Debug)]
struct Creature {
    team: CreatureType,
    damage: usize,
}

#[derive(Debug)]
enum Feature {
    Unit(Creature),
    Wall,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 
            if let Unit(creature) = self {
                match creature.team {
                    Elf => 'E',
                    Goblin => 'G',
                }
            } else {
                '#'
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: u8,
    x: u8,
}

impl Default for Map {
    fn default() -> Map {
        Map { layout: BTreeMap::new()}
    }
}

impl Map {
    fn width(&self) -> u8 {
        self.layout.keys()
            .map(|p| p.x)
            .max_by(|a, b| a.cmp(b))
            .unwrap_or_default()
    }
}

impl FromStr for Map {
    type Err = std::io::Error;

    fn from_str(s: &str) -> std::result::Result<Map, Self::Err> {
        use std::io::{Error, ErrorKind};

        let mut map = Map::default();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point { x: x as u8, y: y as u8 };
                let feature = match c {
                    'E' => Some(Unit(Creature {team: Elf, damage: 0})),
                    'G' => Some(Unit(Creature {team: Goblin, damage: 0})),
                    '#' => Some(Wall),
                    '.' => None,
                    _   => return Err(Error::from(ErrorKind::InvalidInput)),
                };

                map.layout.insert(point, feature);
            }
        }

        Ok(map)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = self.width();
        for (point, feature) in self.layout.iter() {
            match feature {
                Some(feat) => write!(f, "{}", feat)?,
                None => write!(f, ".")?,
            }

            if point.x == width {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
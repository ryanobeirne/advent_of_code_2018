use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::io::{self, Read, stdin};
use std::collections::BTreeMap;

macro_rules! inputerr {
    () => {
        Err(io::Error::from(io::ErrorKind::InvalidInput))
    };
}

use Feature::*;
use Heading::*;
use Turn::*;
use Orientation::*;
use Rotation::*;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut map = Map::from_input(&input)?;
    println!("{}", map);

    Ok(())
}

struct Map(BTreeMap<Location, Position>);

impl Map {
    fn new() -> Map {
        Map(BTreeMap::new())
    }

    fn width(&self) -> u8 {
        self.0.keys()
            .map(|loc| loc.x)
            .max().expect("Empty map!")
        -
        self.0.keys()
            .map(|loc| loc.x)
            .min().expect("Empty map!")
    }

    fn height(&self) -> u8 {
        self.0.keys()
            .map(|loc| loc.y)
            .max().expect("Empty map!")
        -
        self.0.keys()
            .map(|loc| loc.y)
            .min().expect("Empty map!")
    }

    fn from_input(input: &str) -> Result<Map> {
        let mut map = Map::new();

        for (y, line) in input.lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                let location = Location{x: x as u8, y: y as u8};
                let feature = Feature::from_str(&character.to_string())?;
                let cart = Cart::new_from_char(character);
                let position = Position { feature, cart };
                map.0.insert(location, position);
            }
        }

        Ok(map)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Cart {
    heading: Heading,
    next_turn: Turn,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Location {
    y: u8,
    x: u8,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    feature: Feature,
    cart: Option<Cart>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Feature {
    Track(Orientation),
    Intersection, // +
    Curve(Rotation),
    Empty,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
    Horizontal, // -
    Vertical,   // |
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Rotation {
    Clockwise,        // `/`
    CounterClockwise, // `\`
}

impl Cart {
    fn new_from_char(c: char) -> Option<Cart> {
        let direction = match c {
            '>' => Some(East),
            '<' => Some(West),
            '^' => Some(North),
            'v' => Some(South),
            _ => None,
        };

        if let Some(heading) = direction {
            Some(Cart {
                heading,
                next_turn: Left
            })
        } else {
            None
        }
    }

    fn turn(&mut self) {
        self.heading = self.heading.turn(&self.next_turn);
        self.next_turn = self.next_turn.next();
    }
}

impl Heading {
    fn turn(&self, turn: &Turn) -> Heading {
        match (self, turn) {
            (_, Straight) => self.clone(),
            (North, Right) | (South, Left) => East,
            (East,  Right) | (West,  Left) => South,
            (South, Right) | (North, Left) => West,
            (West,  Right) | (East,  Left) => North,
        }
    }
}

impl Turn {
    fn next(&self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

impl FromStr for Feature {
    type Err = std::io::Error;
    fn from_str(s: &str) -> std::result::Result<Feature, Self::Err> {
        match s {
            "-" | ">" | "<" => Ok(Track(Horizontal)),
            "|" | "^" | "v" => Ok(Track(Vertical)),
            "+"  => Ok(Intersection),
            "/"  => Ok(Curve(Clockwise)),
            "\\" => Ok(Curve(CounterClockwise)),
            " "  => Ok(Empty),
            _ => inputerr!(),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            self.0.iter()
                .map(|(loc, pos)| {
                    if loc.x == self.width() {
                        format!("{}\n", pos)
                    } else {
                        pos.to_string()
                    }
                }).collect::<String>()
        )
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match self {
                Track(Horizontal)       => '-',
                Track(Vertical)         => '|',
                Intersection            => '+',
                Curve(Clockwise)        => '/',
                Curve(CounterClockwise) => '\\',
                Empty                   => ' ',
            }
        )
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match self.heading {
                North => '^',
                South => 'v',
                East  => '>',
                West  => '<',
            }
        )
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match &self.cart {
                Some(cart) => cart.to_string(),
                None => self.feature.to_string(),
            }
        )
    }
}
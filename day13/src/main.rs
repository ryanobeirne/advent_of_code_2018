use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::io::{self, Read, stdin};
use std::collections::HashMap;

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

    let mut map = Map::from_input(&input);

    Ok(())
}

struct Map(HashMap<Position, Option<Cart>>);

impl Map {
    fn new() -> Map {
        Map(HashMap::new())
    }

    fn from_input(input: &str) -> Result<Map> {
        let mut map = Map::new();

        for (y, line) in input.lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                let location = Location{x: x as u8, y: y as u8};
                let feature = Feature::from_str(&character.to_string())?;
                let cart = Cart::new_from_char(character);
                let position = Position { location, feature };
                map.0.insert(position, cart);
            }
        }

        Ok(map)
    }
}

#[derive(Hash)]
struct Cart {
    heading: Heading,
    next_turn: Turn,
}

#[derive(Clone, Hash)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Hash)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(PartialEq, Eq, Hash)]
struct Location {
    x: u8,
    y: u8,
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    location: Location,
    feature: Feature,
}

#[derive(PartialEq, Eq, Hash)]
enum Feature {
    Track(Orientation),
    Intersection, // +
    Curve(Rotation),
    Empty,
}

#[derive(PartialEq, Eq, Hash)]
enum Orientation {
    Horizontal, // -
    Vertical,   // |
}

#[derive(PartialEq, Eq, Hash)]
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
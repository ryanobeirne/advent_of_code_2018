use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::io::{self, Read, stdin};
use std::collections::{btree_map::Entry, BTreeMap};

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

    let location = loop {
        // println!("{}\n", &map);
        if let Some(loc) = map.tick() {
            break loc;
        }
    };

    println!("Part 1: {},{}", location.x, location.y);

    Ok(())
}

struct Map(BTreeMap<Location, Position>);

impl Map {
    fn new() -> Map {
        Map(BTreeMap::new())
    }

    fn tick(&mut self) -> Option<Location> {
        let mut carts = Vec::<(Location, Option<Cart>, Feature)>::new();

        for (loc, pos) in self.0.iter_mut() {
            if let Some(_cart) = pos.cart {
                let (new_loc, new_cart) = loc.move_cart(pos);
                carts.push((new_loc, new_cart, pos.feature));
                pos.cart = None;
            }
        }

        let collision = Map::collision_check(&carts);
        if collision.is_some() {
            return collision
        }

        for (loc, cart, _feature) in carts.iter() {
            let mut position = self.0.get(&loc).expect("Location is not in map!").clone();
            position.cart = *cart;

            self.0.insert(*loc, position);
        }
        
        None
    }

    fn collision_check(carts: &Vec<(Location, Option<Cart>, Feature)>) -> Option<Location> {
        let mut counter = BTreeMap::<Location, usize>::new();

        for (loc, _cart, _feature) in carts {
            *counter.entry(*loc).or_insert(0) += 1;
        }

        for (loc, count) in counter.iter() {
            if *count > 1 {
                return Some(*loc);
            }
        }

        None
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

    #[allow(dead_code)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Cart {
    heading: Heading,
    next_turn: Turn,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Location {
    y: u8,
    x: u8,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    feature: Feature,
    cart: Option<Cart>,
}

impl Default for Position {
    fn default() -> Position {
        Position {
            feature: Track(Horizontal),
            cart: None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Feature {
    Track(Orientation),
    Intersection, // +
    Curve(Rotation),
    Empty,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Orientation {
    Horizontal, // -
    Vertical,   // |
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Rotation {
    Clockwise,        // `/`
    CounterClockwise, // `\`
}

impl Location {
    fn move_cart(self, pos: &mut Position) -> (Location, Option<Cart>) {
        if let Some(mut cart) = pos.cart {

            match pos.feature {
                Track(_) => (),
                Intersection => cart.turn(),
                Curve(dir) => cart.curve(dir),
                Empty => panic!("Cart has come off the tracks!"),
            }

            (self.move_by_heading(cart.heading), Some(cart))
        } else {
            (self, pos.cart)
        }
        
    }

    fn move_by_heading(&self, heading: Heading) -> Location {
        match heading {
            North => self.move_north(),
            South => self.move_south(),
            East  => self.move_east(),
            West  => self.move_west(),
        }
    }

    fn move_east(&self) -> Location {
        Location {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn move_west(&self) -> Location {
        Location {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn move_north(&self) -> Location {
        Location {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn move_south(&self) -> Location {
        Location {
            x: self.x,
            y: self.y + 1,
        }
    }
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

    fn curve(&mut self, dir: Rotation) {
        self.heading = self.heading.curve(dir);
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

    fn curve(&self, rotation: Rotation) -> Heading {
        match (self, rotation) {
            (North, Clockwise) | (South, CounterClockwise) => East,
            (North, CounterClockwise) | (South, Clockwise) => West,
            (East, Clockwise) | (West, CounterClockwise)   => North,
            (East, CounterClockwise) | (West, Clockwise)   => South,
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
                    if loc.x >= self.width() {
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
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

    let map = Map::from_input(&input)?;

    let part_1 = part1(&mut map.clone());
    println!("Part 1: {},{}", part_1.x, part_1.y);

    let part_2 = part2(&mut map.clone());
    println!("Part 2: {},{}", part_2.x, part_2.y);

    Ok(())
}

#[cfg(debug_assertions)]
fn sleep(sec: u64) {
    std::thread::sleep(std::time::Duration::from_secs(sec))
}

fn part1(map: &mut Map) -> Location {
    loop {
        #[cfg(debug_assertions)] {
            println!("{}", map);
            sleep(1);
        }

        if let Some(loc) = map.tick() {
            break loc;
        }
    }
}

fn part2(map: &mut Map) -> Location {
    loop {
        #[cfg(debug_assertions)] {
            println!("{}", map);
            sleep(1);
        }

        if map.cart_count() <= 1 {
            break map.first_cart_loc();
        }

        map.tick();

    }
}

#[derive(Clone)]
struct Map(BTreeMap<Location, Position>);

impl Map {
    fn new() -> Map {
        Map(BTreeMap::new())
    }

    fn get(&self, loc: &Location) -> &Position {
        self.0.get(loc).expect("Location is not in map!")
    }

    fn tick(&mut self) -> Option<Location> {
        let cart_locs = self.cart_locs();
        let collisions = self.collision_check(cart_locs);

        if collisions.is_empty() {
            None
        } else {
            Some(collisions[0])
        }
    }

    fn cart_locs(&self) -> Vec<Location> {
        self.0.iter()
            .filter(|(_loc, pos)| pos.cart.is_some())
            .map(|(loc, _pos)| *loc)
            .collect()
    }

    fn collision_check(&mut self, cart_locs: Vec<Location>) -> Vec<Location> {
        let mut collisions = Vec::<Location>::new();

        for location in cart_locs.iter() {
            let mut cart = self.remove_cart(location);
            // println!("Before Mutation: {:?}", &cart);

            if let Some((new_loc, new_cart)) = self.get_new_loc(location, &mut cart) {
                // println!("       Mutation: {:?}", Some(&new_cart));
                if self.has_cart(&new_loc) {
                    collisions.push(new_loc);
                    self.remove_cart(&new_loc);
                } else {
                    self.insert_cart(&new_loc, new_cart);
                }
            }
        }

        collisions
    }

    fn get_new_loc(&self, loc: &Location, cart: &mut Option<Cart>) -> Option<(Location, Cart)> {
        if let Some(mut c) = cart {
            let feature = &self.get(loc).feature;
            Some(loc.move_cart(&mut c, feature))
        } else {
            None
        }
    }

    fn remove_cart(&mut self, loc: &Location) -> Option<Cart> {
        if let Some(pos) = self.0.remove(loc) {
            self.0.insert(*loc, Position { feature: pos.feature, cart: None });
            pos.cart
        } else {
            None
        }
    }

    fn insert_cart(&mut self, loc: &Location, cart: Cart) {
        if let Some(mut position) = self.0.remove(loc) {
            position.cart = Some(cart);
            self.0.insert(*loc, position);
        }
    }

    fn cart_count(&self) -> usize {
        self.0.values()
            .filter(|pos| pos.cart.is_some())
            .count()
    }

    fn first_cart_loc(&self) -> Location {
        self.0.iter()
            .filter(|(_loc, pos)| pos.cart.is_some())
            .map(|(loc, _pos)| *loc)
            .nth(0).expect("Map has no carts!")
    }

    fn has_cart(&self, loc: &Location) -> bool {
        self.get(loc).cart.is_some()
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Cart {
    heading: Heading,
    next_turn: Turn,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Feature {
    Track(Orientation),
    Intersection, // +
    Curve(Rotation),
    Empty,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Orientation {
    Horizontal, // -
    Vertical,   // |
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Rotation {
    Clockwise,        // `/`
    CounterClockwise, // `\`
}

impl Location {
    fn move_cart(self, cart: &mut Cart, feat: &Feature)  -> (Location, Cart) {
            match feat {
                Track(_) => (),
                Intersection => cart.turn(),
                Curve(dir) => cart.curve(dir),
                Empty => panic!("Cart has come off the tracks!"),
            }

            (self.move_by_heading(cart.heading), *cart)
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

    fn curve(&mut self, dir: &Rotation) {
        self.heading = self.heading.curve(dir);
    }
}

impl Heading {
    fn turn(&self, turn: &Turn) -> Heading {
        match (self, turn) {
            (_, Straight) => *self,
            (North, Right) | (South, Left) => East,
            (East,  Right) | (West,  Left) => South,
            (South, Right) | (North, Left) => West,
            (West,  Right) | (East,  Left) => North,
        }
    }

    fn curve(&self, rotation: &Rotation) -> Heading {
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
        let width = self.width();
        write!(f, "{}",
            self.0.iter()
                .map(|(loc, pos)| {
                    if loc.x >= width {
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
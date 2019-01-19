use std::io::{self, Read, stdin};
use std::str::FromStr;
use std::fmt;

fn main() -> Result <(), io::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let mut map = Map::from_input(&input)?;

    let mut area = map.area();

    let mut seconds: i64 = 0;
    loop {
        let new_area = map.area();
        if new_area <= area {
            area = new_area;
        } else {
            map.decrement();
            seconds -= 1;
            break;
        }
        map.increment();
        seconds += 1;
    }

    println!("{}\n{}", map, seconds);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    position: Position,
    velocity: Velocity,
}

impl Point {
    fn increment(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn decrement(&mut self) {
        self.position.x -= self.velocity.x;
        self.position.y -= self.velocity.y;
    }
}

#[derive(Debug)]
struct Map(Vec<Point>);

impl Map {
    fn from_input(input: &str) -> Result<Map, io::Error> {
        Ok(Map(
            input.lines()
            .map(|line| Point::from_str(line).expect("Problem parsing Point!"))
            .collect()
        ))
    }

    fn increment(&mut self) {
        for point in &mut self.0 {
            point.increment()
        }
    }

    fn decrement(&mut self) {
        for point in &mut self.0 {
            point.decrement()
        }
    }

    fn width(&self) -> i64 {
        let left = self.0.iter()
            .map(|p| p.position.x)
            .min().expect("Map is empty!");

        let right = self.0.iter()
            .map(|p| p.position.x)
            .max().expect("Map is empty!");

        right - left
    }

    fn height(&self) -> i64 {
        let top = self.0.iter()
            .map(|p| p.position.y)
            .min().expect("Map is empty!");

        let bot = self.0.iter()
            .map(|p| p.position.y)
            .max().expect("Map is empty!");

        bot - top
    }
    
    fn bounds(&self) -> Bounds {
        let top = self.0.iter()
            .map(|p| p.position.y)
            .min().expect("Map is empty!");

        let bot = self.0.iter()
            .map(|p| p.position.y)
            .max().expect("Map is empty!");

        let left = self.0.iter()
            .map(|p| p.position.x)
            .min().expect("Map is empty!");

        let right = self.0.iter()
            .map(|p| p.position.x)
            .max().expect("Map is empty!");

        Bounds{top, bot, left, right}
    }

    fn area(&self) -> i64 {
        self.width() * self.height()
    }

    fn normalize_positions(&self) -> Vec<Position> {
        let bounds = self.bounds();

        self.0.iter()
            .map(|p|
                Position {
                    x: p.position.x - bounds.left,
                    y: p.position.y - bounds.top,
                }
            )
            .collect()
    }
}

#[allow(dead_code)]
struct Bounds {
    top: i64,
    bot: i64,
    left: i64,
    right: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Velocity {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Point, io::Error> {
        let split: Vec <i64> = s.split('<')
            .flat_map(|s| s.split('>'))
            .flat_map(|s| s.split(','))
            .map(|s| s.trim())
            .filter_map(|s| s.parse().ok())
            .collect();

        if split.len() != 4 {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        } else {
            Ok(
                Point {
                    position: Position {x: split[0], y: split[1]},
                    velocity: Velocity {x: split[2], y: split[3]}
                }
            )
        }

    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        let positions = self.normalize_positions();

        for y in 0..= self.height() {
            for x in 0..= self.width() {
                if positions.contains(&&Position{x ,y}) {
                    s.push('#');
                } else {
                    s.push(' ')
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
#[test]
fn point_from_str() {
    let s = "position=<-40271, -20082> velocity=< 4,  2>";
    let point = Point::from_str(s).unwrap();
    println!("{:?}", point);
    let expected = Point {
        position: Position {x: -40271, y: -20082},
        velocity: Velocity {x: 4, y: 2},
    };

    assert_eq!(expected, point);
}
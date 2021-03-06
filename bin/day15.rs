use std::collections::{HashMap, BTreeMap};
use std::str::FromStr;
use std::fmt;
use std::io::{Read, Write, stdout, stdin};

use CreatureType::{Elf, Goblin};
use Feature::{Unit, Wall};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let input = &mut String::new();
    stdin().read_to_string(input)?;

    let mut map = Map::from_str(&input)?;

    writeln!(stdout(), "{}", map)?;

    let (winner, score) = map.run().expect(r#"Thought we had a winner. Guess not. ¯\_(ツ)_/¯"#);

    println!("Part 1:\n\tWinner: {:?}\n\tScore: {}", winner, score);

    Ok(())
}

#[derive(Debug)]
struct Map {
    layout: BTreeMap<Point, Option<Feature>>,
    score: HashMap<CreatureType, usize>,
}

impl Map {
    fn run(&mut self) -> Option<(&CreatureType, &usize)> {
        self.initialize();

        loop {
            self.round();
            if self.has_winner() {
                return self.leader();
            }
            dbg!(&self.score);
        }
    }

    fn initialize(&mut self) {
        for creature in self.layout.values()
            .filter_map(|f| *f)
            .filter(|f| f.is_unit())
            .map(|f| f.unwrap_unit().team) {
                self.score.insert(creature, 200);
            }
    }

    fn round(&mut self) {
        let points_creatures = self.layout.iter()
            .filter(|(_p, f)| f.is_some() && f.unwrap().is_unit())
            .map(|(p, f)| (p, f.unwrap())) 
            .map(|(p, f)| (p, f.unwrap_unit()));

        for (point, attacker) in points_creatures {
            let area = Area::from(point);
            for a_point in area.around.iter()
                .filter_map(|ap| self.layout.get(ap))
                .filter_map(|ap| ap.as_ref())
            {

            }
        }
    }

    fn leader(&self) -> Option<(& CreatureType, & usize)> {
        self.score.iter()
            .max_by(|(_ca, sa), (_cb, sb)| sa.cmp(sb))
    }

    fn has_winner(&self) -> bool {
        let mut tally = HashMap::new();

        for (_point, feature) in self.layout.iter()
            .filter(|(_p, f)| f.is_some() && f.unwrap().is_unit())
            .map(|(p, f)| (p, f.unwrap())) 
        {
            *tally.entry(feature.unwrap_unit().team).or_insert(0_usize) += 1;
        }
            
        tally.keys().count() == 1
    }

    fn closest_enemy(&self, point: &Point) -> &Creature {

        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq)]
struct Area<'a> {
    center: &'a Point,  // The middle point
    around: [Point; 8], // The surrounding 8 points
}

impl<'a> From<&'a Point> for Area<'a> {
    fn from(center: &'a Point) -> Area<'a> {
        Area {
            center,
            around: [
                center.top_left(), center.top_middle(), center.top_right(),
                center.mid_left(),                      center.mid_right(),
                center.bot_left(), center.bot_middle(), center.bot_right()
            ],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Creature {
    team: CreatureType,
    damage: usize,
    loc: Point,
}

impl Creature {
    fn attack(&self, other: &mut Creature) {
        other.damage += 20
    }

    fn is_enemy(&self, other: &Creature) -> bool {
        self.team.enemy() == other.team
    }
}

#[derive(Debug, Clone, Copy)]
enum Feature {
    Unit(Creature),
    Wall,
}

impl Feature {
    fn is_unit(&self) -> bool {
        if let Unit(_) = self {
            true
        } else {
            false
        }
    }

    fn unwrap_unit(self) -> Creature {
        if let Unit(creature) = self {
            return creature;
        } else {
            panic!("Attempted to unwrap a Wall!");
        }
    }
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point {
    y: i8,
    x: i8,
}

impl Point {
    fn top_left(self) -> Point {
        Point { x: self.x - 1, y: self.y - 1 }
    }

    fn top_middle(self) -> Point {
        Point { x: self.x, y: self.y - 1 }
    }

    fn top_right(self) -> Point {
        Point { x: self.x + 1 , y: self.y - 1 }
    }
    
    fn mid_left(self) -> Point {
        Point { x: self.x - 1, y: self.y }
    }
    
    fn mid_right(self) -> Point {
        Point { x: self.x + 1, y: self. y}
    }

    fn bot_left(self) -> Point {
        Point { x: self.x - 1, y: self.y + 1}
    }

    fn bot_middle(self) -> Point {
        Point { x: self.x, y: self.y + 1 }
    }

    fn bot_right(self) -> Point {
        Point { x: self.x + 1, y: self.y + 1}
    }
}

#[test]
fn area_from_point() {
    let point = Point { x: 1, y: 1 };
    let area_expected = Area {
        center: &point,
        around: [
            Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 },
            Point { x: 0, y: 1 },                       Point { x: 2, y: 1 },
            Point { x: 0, y: 2 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 },
        ],
    };
    let area_from = Area::from(&point);

    assert_eq!(area_expected, area_from);

    Area::from(&Point { x: 0, y: 0} );
}

impl Default for Map {
    fn default() -> Map {
        Map { 
            layout: BTreeMap::new(),
            score: HashMap::new(),
        }
    }
}

impl Map {
    fn width(&self) -> i8 {
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
                let point = Point { x: x as i8, y: y as i8 };
                let feature = match c {
                    'E' => Some(Unit(Creature {team: Elf, damage: 0, loc: point})),
                    'G' => Some(Unit(Creature {team: Goblin, damage: 0, loc: point})),
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
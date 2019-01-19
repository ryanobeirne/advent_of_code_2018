use std::collections::BTreeMap;
use std::isize::MIN;
use std::env::Args;

fn main() {
    let args = std::env::args();
    let part1 = part1(args);
    println!("Most Powerful {:?}", part1);
}

fn part1(args: Args) -> Coord {
    let serial = args.skip(1)
        .collect::<Vec<String>>()
        .first()
        .expect("Not enough arguments!")
        .parse::<usize>()
        .expect("Cannot parse usize from arguments!");

    let grid = Grid::new(serial);

    let mut power_map = BTreeMap::<Coord, isize>::new();

    for coord in grid.rack.keys() {
        match grid.get3x3(coord) {
            Some(vec) => {
                let power: isize = vec.into_iter()
                    .map(|fc| fc.power_level)
                    .sum();

                power_map.insert(*coord, power);
            },
            None => ()
        }
    }

    let mut max = (Coord{x: 0, y: 0}, MIN);
    for p_grid in power_map.into_iter() {
        if p_grid.1 > max.1 {
            max = p_grid;
        }
    }

    max.0
}

#[derive(Debug)]
struct Grid {
    rack: BTreeMap<Coord, FuelCell>,
    serial: usize,
}

impl Grid {
    fn new(serial: usize) -> Grid {
        let mut rack = BTreeMap::new();

        for y in 1..=300 {
            for x in 1..=300 {
                let coord = Coord{x,y};
                rack.insert(coord, FuelCell::new(coord, serial));
            }
        }

        Grid { rack, serial }
    }

    fn get3x3(&self, top_left: &Coord) -> Option<Vec<FuelCell>> {
        let a = *self.rack.get(top_left)?;
        let b = *self.rack.get(&top_left.increment(1, 0))?;
        let c = *self.rack.get(&top_left.increment(2, 0))?;
        let d = *self.rack.get(&top_left.increment(0, 1))?;
        let e = *self.rack.get(&top_left.increment(1, 1))?;
        let f = *self.rack.get(&top_left.increment(2, 1))?;
        let g = *self.rack.get(&top_left.increment(0, 2))?;
        let h = *self.rack.get(&top_left.increment(1, 2))?;
        let i = *self.rack.get(&top_left.increment(2, 2))?;
        
        Some(vec![a, b, c, d, e, f, g, h, i])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FuelCell {
    rackid: usize,
    power_level: isize,
}

impl FuelCell {
    fn new(coord: Coord, serial: usize) -> FuelCell {
        let rackid = (coord.x + 10) as usize;
        let power = ((rackid as isize * coord.y) + serial as isize) * rackid as isize;

        let pow_str: Vec<isize> = power.to_string()
            .chars()
            .filter_map(|c| c.to_string().parse().ok())
            .collect();

        let hundreds = match pow_str.len() {
            0 | 1 | 2 => 0,
            _ => pow_str[pow_str.len() - 3]
        };

        let power_level = hundreds - 5;

        FuelCell { rackid, power_level }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn increment(&self, x: isize, y: isize) -> Coord {
        Coord {
            x: self.x + x,
            y: self.y + y
        }
    }
}

#[test]
fn fuelcell() {
    let fc = FuelCell::new(Coord{x: 122, y: 79}, 57);
    assert_eq!(fc.power_level, -5);


    let fc = FuelCell::new(Coord{x: 217, y: 196}, 39);
    assert_eq!(fc.power_level, 0);

    let fc = FuelCell::new(Coord{x: 101, y: 153}, 71);
    assert_eq!(fc.power_level, 4);
}
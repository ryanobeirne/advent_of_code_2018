use std::collections::{BTreeSet, BTreeMap};
use std::i32::MIN;
use std::env::args;
extern crate rayon;
use rayon::prelude::*;

fn main() {
    let serial = args().skip(1)
        .collect::<Vec<String>>()
        .first()
        .expect("Not enough arguments!")
        .parse::<u32>()
        .expect("Cannot parse u32 from arguments!");

    let grid = Grid::new(serial);

    let part1 = part1(&grid);
    println!("Part 1: {},{}", part1.x, part1.y);

    let part2 = part2(&grid);
    println!("Part 2: {},{},{}", part2.0.x, part2.0.y, part2.1);
}

fn part1(grid: &Grid) -> Coord {

    let mut power_map = BTreeMap::<Coord, i32>::new();

    for coord in grid.rack.keys() {
        match grid.get3x3(coord) {
            Some(vec) => {
                let power: i32 = vec.into_iter()
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

fn part2(grid: &Grid) -> (Coord, i32) {
    let power_map = grid.all_squares().into_par_iter()
        .map(|(coord, size)| 
            ((coord, size), grid.get_square_power(&coord, size))
        ).collect::<BTreeMap<(Coord, i32), i32>>();

    dbg!("Done collecting power_map");

    let mut max = ((Coord{x: 0, y: 0}, MIN), MIN);
    for p_grid in power_map.into_iter() {
        if p_grid.1 > max .1 {
            max = p_grid;
        }
    }

    max.0
}

#[derive(Debug)]
struct Grid {
    rack: BTreeMap<Coord, FuelCell>,
    serial: u32,
}

impl Grid {
    fn new(serial: u32) -> Grid {
        let mut rack = BTreeMap::new();

        for y in 1..=300 {
            for x in 1..=300 {
                let coord = Coord{x,y};
                rack.insert(coord, FuelCell::new(coord, serial));
            }
        }

        Grid { rack, serial }
    }

    #[cfg(test)]
    fn new_with_size(serial: u32, size: i32) -> Grid {
        let mut rack = BTreeMap::new();

        for y in 1..=size {
            for x in 1..=size {
                let coord = Coord{x,y};
                rack.insert(coord, FuelCell::new(coord, serial));
            }
        }

        Grid { rack, serial }
    }

    fn all_squares(&self) -> BTreeSet<(Coord, i32)> {
        let mut all_squares = BTreeSet::new();

        for coord1 in self.rack.keys() {
            for coord2 in self.rack.keys()
                .filter(|c2| {
                    c2.x >= coord1.x && c2.y >= coord1.y &&
                    c2.x - coord1.x == c2.y - coord1.y
                })
            {
                let size = coord2.x - coord1.x;
                all_squares.insert((*coord1, size));
            }
        }

        all_squares
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

    fn get_square_power(&self, coord: &Coord, size: i32) -> i32 {
        let mut coords = Vec::<Coord>::new();

        for x in coord.x ..= coord.x + size {
            for y in coord.y ..= coord.y + size {
                coords.push(Coord{x,y});
            }
        }

        coords.par_iter()
            .filter_map(|c| self.rack.get(c) )
            .map(|fc| fc.power_level)
            .sum()
    }
}

#[test]
fn grid_test() {
    let grid = Grid::new_with_size(100, 3);
    let all_squares = grid.all_squares();
    for (coord, size) in all_squares.iter() {
        dbg!((coord, size, grid.get_square_power(&coord, *size)));
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FuelCell {
    rackid: u32,
    power_level: i32,
}

impl FuelCell {
    fn new(coord: Coord, serial: u32) -> FuelCell {
        let rackid = (coord.x + 10) as u32;
        let power = ((rackid as i32 * coord.y) + serial as i32) * rackid as i32;

        let pow_str: Vec<i32> = power.to_string()
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
    x: i32,
    y: i32,
}

impl Coord {
    fn increment(&self, x: i32, y: i32) -> Coord {
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
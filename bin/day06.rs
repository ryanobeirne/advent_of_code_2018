use std::error::Error;
use std::io::{stdin, Read};
use std::str::FromStr;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let coords: Vec<Coord> = input.lines()
        .map(|line| Coord::from_str(line))
        .filter_map(|r| r.ok())
        .collect();

    let grid = Grid::new(coords)?;

    let answer_1 = part1(&grid);
    println!("Day 6, Part 1: {}", answer_1);

    let answer_2 = part2(&grid);
    println!("Day 6, Part 2: {}", answer_2);

    Ok(())
}

fn part1(grid: &Grid) -> usize {
    *grid.area_map().iter()
        .filter(|(coord, _)| !coord.edge(grid))
        .map(|(_, count)| count)
        .max()
        .expect("Couldn't find maximum!")
}

fn part2(grid: &Grid) -> usize {
    let area_map = grid.area_map();

    grid.coords.iter()
        .map(|coord| (coord, grid.sum_distance(&coord)) )
        .filter(|(_, d)| d < &10000_i32)
        .map(|(c, _)| c)
        .filter_map(|coord| area_map.get(&coord))
        .sum()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn edge(&self, grid: &Grid) -> bool {
        let min_max = Grid::min_max(&grid.coords);

        self.x <= min_max.0 && self.x >= min_max.1 &&
        self.y <= min_max.2 && self.y >= min_max.3
    }
}

impl FromStr for Coord {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Coord> {
        let split: Vec<&str> = s.split(',')
            .map(|i| i.trim()
            ).collect();
        
        Ok(Coord{
            x: split[0].parse()?,
            y: split[1].parse()?,
        })
    }

}

#[derive(Debug)]
struct Grid {
    coords: Vec<Coord>,
    all_points: Vec<Coord>,
}

impl Grid {
    fn new(coords: Vec<Coord>) -> Result<Grid> {
        let (min_x, max_x, min_y, max_y) = Grid::min_max(&coords);

        let mut all_points = Vec::new();
        for y in min_y ..= max_y {
                for x in min_x ..= max_x {
                    all_points.push(Coord {x: x as i32, y: y as i32});
                }
        }
        
        Ok(Grid {
            coords,
            all_points,
        })
    }

    fn min_max(coords: &Vec<Coord>) -> (i32, i32, i32, i32) {
        if coords.is_empty() {
            panic!("Cannot find min_max from empty set!");
        }

        let min_x  = coords.iter().map(|c| c.x).min().unwrap();
        let max_x  = coords.iter().map(|c| c.x).max().unwrap();
        let min_y  = coords.iter().map(|c| c.y).min().unwrap();
        let max_y  = coords.iter().map(|c| c.y).max().unwrap();

        (min_x, max_x, min_y, max_y)
    }

    fn closest_coords(&self, coord: &Coord) -> (Vec<Coord>, i32) {
        if self.coords.contains(coord) {
            return (vec![*coord], 0);
        }

        let cm: (Vec<Coord>, i32) = (Vec::new(), std::i32::MAX);
        let mut closest = cm.clone();

        for loc in &self.coords {
            let distance = coord.distance(loc);
            if distance < closest.1 {
                closest = (vec![*loc], distance);
            } else if distance == closest.1 {
                closest.0.push(*loc);
            }
        }

        if closest.0.is_empty() {
                panic!("Could not find a closest Coordinate!");
        }

        // println!("{:?}", closest);

        closest
    }
    
    fn sum_distance(&self, coord: &Coord) -> i32 {
        self.coords.iter().map(|c| coord.distance(&c)).sum()
    }

    fn area_map(&self) -> HashMap<Coord, usize> {
        let mut area_map = HashMap::new();

        for point in &self.all_points {
            let closest = self.closest_coords(point);
            if closest.0.len() == 1 {
                *area_map.entry(closest.0[0]).or_insert(0) += 1;
            }
        }

        area_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stringer() -> Result<()> {
        let s = "1, 2";
        assert_eq!(
            Coord::from_str(s)?,
            Coord { x: 1, y: 2}
        );
        Ok(())
    }

    #[test]
    fn closer() -> Result<()> {
        let grid = Grid::new(
            vec![
                Coord {x: 1, y: 1}, Coord {x: 1, y: 6},
                Coord {x: 8, y: 3}, Coord {x: 3, y: 4},
                Coord {x: 5, y: 5}, Coord {x: 8, y: 9},
            ]
        )?;

        let closest = grid.closest_coords(&Coord{x: 0, y: 0});
        assert!(
            closest.0.contains(&Coord{x: 1, y: 1})
        );

        let closest2 = grid.closest_coords(&Coord{x: 3, y: 9});
        assert!(
            closest2.0.contains(&Coord{x: 1, y: 6}) &&
            closest2.0.contains(&Coord{x: 8, y: 9})
        );

        Ok(())
    }
}

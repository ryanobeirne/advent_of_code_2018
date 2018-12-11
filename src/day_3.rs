pub fn go() {
    println!("Day 3");
    part_1();
}

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point{
    #[allow(dead_code)]
    fn inside_rectangle(&self, rect: &Rectangle) -> bool {
        let rect_br = rect.bottom_right();
        self.x > rect.origin.x &&
        self.x <= rect_br.x     &&
        self.y > rect.origin.y &&
        self.y <= rect_br.y
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rectangle {
    id: usize,
    origin: Point,
    size: Point,
}

impl Rectangle {
    fn build_from_line(s: &String) -> Rectangle {
        let split: Vec<&str> = s.split_whitespace().collect();

        let id = split[0].trim_left_matches('#').parse::<usize>().unwrap_or(0);
        
        let split_comma: Vec<&str> = split[2].split(',').collect();
        let origin_xy: Vec<usize> = split_comma.iter()
            .map(|s| s.trim_right_matches(':').parse().unwrap_or(0))
            .collect();
        let origin = Point {
            x: origin_xy[0],
            y: origin_xy[1],
        };

        let size_xy: Vec<usize> = split[3].split('x')
            .map(|i| i.parse::<usize>().unwrap_or(0))
            .collect();
        let size = Point {
            x: size_xy[0],
            y: size_xy[1],
        };

        Rectangle {
            id,
            origin,
            size,
        }

    }

    fn bottom_right(&self) -> Point {
        let x = self.origin.x + self.size.x;
        let y = self.origin.y + self.size.y;
        Point { x, y}
    }

    #[allow(dead_code)]
    fn overlaps(&self, other: &Self) -> bool {
        let self_br = self.bottom_right();
        let other_br = other.bottom_right();

        self.origin.x < other_br.x     &&
        self_br.x     > other.origin.x &&
        self.origin.y < other_br.y     &&
        self_br.y     > other.origin.y
    }

    fn contains(&self, point: &Point) -> bool {
        let self_br = self.bottom_right();

        point.x > self.origin.x &&
        point.x <= self_br.x    &&
        point.y > self.origin.y &&
        point.y <= self_br.y
    }
}

fn part_1() {
    let lines = super::input::read(3);

    let rec_vec: Vec<Rectangle> = lines.iter()
        .map(|s| Rectangle::build_from_line(s))
        .collect();

    let mut gt2_count = 0;

    let arr_xy = [[0; 1000]; 1000];

    for (x, row) in arr_xy.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let mut overlap_count = 0;
            let point = &Point { x, y };

            for rect in &rec_vec {
                if rect.contains(point) {
                    overlap_count += 1;
                    if overlap_count == 2 { continue }
                }
            }

            if overlap_count >= 2 {
                gt2_count += 1;
            }
        }

    }

    println!("\tPart 1: {}", gt2_count);
}

use std::fmt;
use std::fs;

#[derive(Debug)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug)]
struct Position {
    horizontal: u32,
    depth: u32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "horizontal : {}, depth: {}, puzzle answer: {}",
            self.horizontal,
            self.depth,
            self.horizontal * self.depth
        )
    }
}

fn main() {
    let f = fs::read_to_string("input.txt").expect("Could not read file");
    let directions: Vec<Direction> = f.split("\n").map(parse_input).collect();

    println!(
        "Part One Result - {}",
        calculate_final_position(&directions)
    );
}

// Take the individual rows and parse them in to our Direction enum types, not
// because it is necessary but because it is a cool Rust thing I just learned
// about so we'll have a Vec e.g. `[Forward(3), Down(9), Forward(5), Up(1) ...]`
// for super fun match-based processing.
fn parse_input(s: &str) -> Direction {
    // e.g. forward 1
    let mut x = s.split_whitespace();
    let d = x.next().unwrap();
    let v_str = x.next().unwrap();
    let v: u32 = v_str.parse().unwrap();

    match d {
        "forward" => return Direction::Forward(v),
        "down" => return Direction::Down(v),
        "up" => return Direction::Up(v),
        _ => panic!("unknown direction {}", d),
    }
}

fn calculate_final_position(d: &Vec<Direction>) -> Position {
    let (mut horizontal, mut depth) = (0, 0);

    for i in d {
        match i {
            Direction::Down(v) => depth = depth + v,
            Direction::Up(v) => depth = depth - v,
            Direction::Forward(v) => horizontal = horizontal + v,
        }
    }

    Position { horizontal, depth }
}

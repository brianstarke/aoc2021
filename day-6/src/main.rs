use std::fs;
use std::time;

fn main() {
    let input = include_str!("../input.txt");

    let fishies: Vec<u8> = parse_fishies(input);

    println!("Part One Result - {}", solve_1(fishies.clone()));
    part2_array();
    println!("Part Two Result - {}", solve_2(fishies.clone()));
}

// Each fishy is a timer value, no higher than 8
fn parse_fishies(i: &str) -> Vec<u8> {
    return i.split(",").map(|x| x.parse().unwrap()).collect();
}

fn solve_1(f: Vec<u8>) -> usize {
    let mut x = f.clone();

    for _i in 0..80 {
        x = increment_day(x);
    }

    return x.len();
}

fn part2_array() {
    let input: Vec<u8> = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let mut tracker: [u64; 9] = [0; 9];

    for i in input {
        tracker[i as usize] += 1;
    }

    for _ in 0..256 {
        tracker.rotate_left(1);
        tracker[6] += tracker[8];
    }

    println!("{:?}", tracker.into_iter().sum::<u64>());
}

fn solve_2(f: Vec<u8>) -> usize {
    let mut x = f.clone();

    for i in 0..256 {
        let t0 = time::Instant::now();

        x = increment_day(x);

        println!(
            "day {}, len {}, time {:?}",
            i,
            x.len(),
            time::Instant::now().duration_since(t0)
        );
    }

    return x.len();
}

fn increment_day(fishies: Vec<u8>) -> Vec<u8> {
    let mut new_fishies: Vec<u8> = Vec::new();

    for f in fishies {
        if f > 0 {
            new_fishies.push(f - 1);
        } else {
            new_fishies.push(6);
            new_fishies.push(8);
        }
    }

    new_fishies
}

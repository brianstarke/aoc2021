use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").expect("Could not read file");
    let measurements: Vec<u16> = f.split("\n").map(|x| x.parse().unwrap()).collect();

    println!(
        "Part One Result - {}",
        calculate_increases(measurements.clone())
    );
    println!(
        "Part Two Result - {}",
        calculate_increases_2(measurements.clone())
    );
}

fn calculate_increases(m: Vec<u16>) -> u16 {
    let mut num_increases: u16 = 0;

    for n in 0..m.len() {
        if n == 0 {
            continue;
        }

        if m[n] > m[n - 1] {
            num_increases += 1;
        }
    }

    num_increases
}

fn calculate_increases_2(m: Vec<u16>) -> u16 {
    let mut sums: Vec<u16> = Vec::new();

    for n in 0..m.len() - 2 {
        sums.push(m[n] + m[n + 1] + m[n + 2])
    }

    calculate_increases(sums)
}

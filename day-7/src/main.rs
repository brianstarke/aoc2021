fn main() {
    let input = include_str!("../input.txt");
    let positions: Vec<i16> = input.split(",").map(|x| x.parse().unwrap()).collect();

    println!("Part One Result - {}", solve_1(positions.clone()));
    println!("Part Two Result - {}", solve_2(positions.clone()));
}

fn solve_1(mut postitions: Vec<i16>) -> i32 {
    postitions.sort();

    let mut least_fuel = -1;

    for new_pos in postitions[0]..postitions[postitions.len() - 1] {
        let fuel = fuel_to_move_to(&postitions, new_pos);
        if least_fuel == -1 {
            least_fuel = fuel;
        }
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }

    return least_fuel;
}

fn solve_2(mut p: Vec<i16>) -> i32 {
    p.sort();

    let mut least_fuel = -1;

    for new_pos in p[0]..p[p.len() - 1] {
        let fuel = fuel_to_move_to_2(&p, new_pos);
        if least_fuel == -1 {
            least_fuel = fuel;
        }
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }

    return least_fuel;
}

fn fuel_to_move_to(current_positions: &Vec<i16>, new_pos: i16) -> i32 {
    // pretty sure we don't need to actually calculate this for all values since i'm
    // 90% certain the number we're looking for is the mean of min/max... but I haven't
    // seen part 2 yet soooooo... let's just brute force it for now...

    let mut total_fuel: i32 = 0;

    for x in current_positions {
        let fuel_cost: i32 = (new_pos - x).into();
        total_fuel = total_fuel + fuel_cost.abs();
    }

    return total_fuel;
}

fn fuel_to_move_to_2(current_positions: &Vec<i16>, new_pos: i16) -> i32 {
    let mut total_fuel: i32 = 0;

    for x in current_positions {
        let fuel_cost: i32 = (new_pos - x).into();
        total_fuel = total_fuel + gauss_sum(fuel_cost.abs());
    }

    return total_fuel;
}

fn gauss_sum(i: i32) -> i32 {
    return (i * (i + 1)) / 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_to_move_to_works() {
        let current_positions: Vec<i16> = vec![1, 5, 10];

        assert_eq!(fuel_to_move_to(&current_positions, 5), 9)
    }

    #[test]
    fn fuel_to_move_to_2_works() {
        let current_positions: Vec<i16> = vec![1, 5, 10];

        assert_eq!(fuel_to_move_to_2(&current_positions, 5), 25)
    }
}

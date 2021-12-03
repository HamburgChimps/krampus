use std::{fs, vec};

fn day1() {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    let measurements: Vec<_> = input.lines().collect();
    let mut increases = 0;

    for (i, m) in measurements.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let prev = measurements.get(i - 1);

        if m.trim().parse::<u32>().unwrap() > prev.unwrap().trim().parse::<u32>().unwrap() {
            increases += 1;
        }
    }

    println!("day 1 part 1 answer: {:#}", increases);

    let window_sums: Vec<u32> = Vec::new();
    let mut window_increases: u32 = 0;

    for (i, m) in measurements.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let prev = measurements.get(i - 1);

        if m.trim().parse::<u32>().unwrap() > prev.unwrap().trim().parse::<u32>().unwrap() {
            increases += 1;
        }
    }
}

fn main() {
    day1();
}

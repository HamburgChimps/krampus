use std::fs;

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

    println!("day 1 part 1 answer: {}", increases);

    const WINDOW_SIZE: usize = 3;

    let mut prev_window_sum;
    let mut current_window_sum = 0;
    let mut win_increases: u32 = 0;

    let mut k = 0;

    while k < WINDOW_SIZE {
        current_window_sum += measurements.get(k).unwrap().parse::<u32>().unwrap();
        k += 1;
    }

    for (i, _m) in measurements.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if let None = measurements.get(i + WINDOW_SIZE - 1) {
            break;
        }

        prev_window_sum = current_window_sum;

        current_window_sum -= measurements.get(i - 1).unwrap().parse::<u32>().unwrap();
        current_window_sum += measurements
            .get(i + WINDOW_SIZE - 1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        if current_window_sum > prev_window_sum {
            win_increases += 1;
        }
    }

    println!("day 1 part 2 answer: {}", win_increases);
}

fn main() {
    day1();
}

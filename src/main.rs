use std::{cmp::min, fmt, fs};

fn main() {
    day1();
    day2();
    day3();
    day4();
    day5();
    day6();
    day7();
    day8();
}

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

fn day2() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let mut instructions = input.lines();

    let mut horizontal_position: u32 = 0;
    let mut vertical_position: i32 = 0;

    for i in instructions {
        let inx_parts: Vec<_> = i.split(" ").collect();

        let direction = inx_parts.first().unwrap();
        let units: u32 = inx_parts.last().unwrap().parse().unwrap();

        if let "forward" = *direction {
            horizontal_position += units;
        }

        if let "down" = *direction {
            vertical_position += units as i32;
        }

        if let "up" = *direction {
            vertical_position -= units as i32;
        }
    }

    println!(
        "day 2 part 1 answer: {}",
        horizontal_position as i32 * vertical_position
    );

    horizontal_position = 0;
    vertical_position = 0;

    let mut aim: i32 = 0;
    instructions = input.lines();

    for i in instructions {
        let inx_parts: Vec<_> = i.split(" ").collect();

        let direction = inx_parts.first().unwrap();
        let units: u32 = inx_parts.last().unwrap().parse().unwrap();

        if let "forward" = *direction {
            horizontal_position += units;
            vertical_position += aim * units as i32;
        }

        if let "down" = *direction {
            aim += units as i32;
        }

        if let "up" = *direction {
            aim -= units as i32;
        }
    }

    println!(
        "day 2 part 2 answer: {}",
        horizontal_position as i32 * vertical_position
    );
}

fn day3() {
    let input = fs::read_to_string("input/day3.txt").unwrap();
    let diagnostics = input.lines();
    let mut diagnostics_serialized: Vec<Vec<u32>> = Vec::new();

    for diagnostic in diagnostics {
        let diagnostic_serialized: Vec<u32> =
            diagnostic.chars().map(|c| c.to_digit(2).unwrap()).collect();
        diagnostics_serialized.push(diagnostic_serialized);
    }

    let counts: Vec<(u32, u32)> = diagnostics_serialized
        .iter()
        .map(|diagnostic_serialized| {
            diagnostic_serialized
                .into_iter()
                .map(|bit| {
                    if *bit == 0 {
                        return (1, 0);
                    }

                    return (0, 1);
                })
                .collect()
        })
        // perhaps try sum w/ type annotation here
        .reduce(|mut acc: Vec<(u32, u32)>, counts| {
            for (i, count) in counts.iter().enumerate() {
                acc[i].0 += count.0;
                acc[i].1 += count.1;
            }
            return acc;
        })
        .unwrap();

    let gamma = u32::from_str_radix(
        counts
            .iter()
            .map(|count| if count.0 > count.1 { '0' } else { '1' })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();
    let epsilon = !gamma & ((1 << counts.len()) - 1);

    println!("day 3 part 1 answer: {}", gamma * epsilon);

    let mut oxygen_generator_rating_candidates = diagnostics_serialized.clone();
    let mut co2_scrubber_rating_candidates = diagnostics_serialized.clone();

    let oxygen_generator_rating: u32;
    let co2_scrubber_rating: u32;

    for pos in 0..oxygen_generator_rating_candidates.first().unwrap().len() {
        if oxygen_generator_rating_candidates.len() == 1 {
            break;
        }

        let counts: Vec<(u32, u32)> = oxygen_generator_rating_candidates
            .iter()
            .map(|oxygen_generator_rating_candidate| {
                oxygen_generator_rating_candidate
                    .into_iter()
                    .map(|bit| {
                        if *bit == 0 {
                            return (1, 0);
                        }

                        return (0, 1);
                    })
                    .collect()
            })
            .reduce(|mut acc: Vec<(u32, u32)>, counts| {
                for (i, count) in counts.iter().enumerate() {
                    acc[i].0 += count.0;
                    acc[i].1 += count.1;
                }
                return acc;
            })
            .unwrap();

        let count = counts.get(pos).unwrap();

        if count.1 >= count.0 {
            oxygen_generator_rating_candidates = oxygen_generator_rating_candidates
                .into_iter()
                .filter(|d| *d.get(pos).unwrap() == 1)
                .collect();
            continue;
        }

        oxygen_generator_rating_candidates = oxygen_generator_rating_candidates
            .into_iter()
            .filter(|d| *d.get(pos).unwrap() == 0)
            .collect();
    }

    oxygen_generator_rating = u32::from_str_radix(
        oxygen_generator_rating_candidates
            .pop()
            .unwrap()
            .into_iter()
            .map(|b| char::from_digit(b, 2).unwrap())
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    for pos in 0..co2_scrubber_rating_candidates.first().unwrap().len() {
        if co2_scrubber_rating_candidates.len() == 1 {
            break;
        }

        let counts: Vec<(u32, u32)> = co2_scrubber_rating_candidates
            .iter()
            .map(|co2_scrubber_rating_candidate| {
                co2_scrubber_rating_candidate
                    .into_iter()
                    .map(|bit| {
                        if *bit == 0 {
                            return (1, 0);
                        }

                        return (0, 1);
                    })
                    .collect()
            })
            .reduce(|mut acc: Vec<(u32, u32)>, counts| {
                for (i, count) in counts.iter().enumerate() {
                    acc[i].0 += count.0;
                    acc[i].1 += count.1;
                }
                return acc;
            })
            .unwrap();

        let count = counts.get(pos).unwrap();

        if count.1 < count.0 {
            co2_scrubber_rating_candidates = co2_scrubber_rating_candidates
                .into_iter()
                .filter(|d| *d.get(pos).unwrap() == 1)
                .collect();
            continue;
        }

        co2_scrubber_rating_candidates = co2_scrubber_rating_candidates
            .into_iter()
            .filter(|d| *d.get(pos).unwrap() == 0)
            .collect();
    }

    co2_scrubber_rating = u32::from_str_radix(
        co2_scrubber_rating_candidates
            .pop()
            .unwrap()
            .into_iter()
            .map(|b| char::from_digit(b, 2).unwrap())
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    println!(
        "day 3 part 2 answer: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}

fn day4() {
    let input = fs::read_to_string("input/day4.txt").unwrap();

    let (draws_input, boards_input) = input.split_once("\n\n").unwrap();

    let draws: Vec<u32> = draws_input.split(',').map(|d| d.parse().unwrap()).collect();

    #[derive(Debug)]
    struct Board {
        data: Vec<Vec<Num>>,
        bingo: bool,
        last_marked: Option<Num>,
    }

    #[derive(Debug, Clone)]
    struct Num {
        data: u32,
        marked: bool,
    }

    impl Board {
        fn mark(&mut self, draw: u32) {
            for row in &mut self.data {
                for num in row {
                    if num.data == draw {
                        num.marked = true;
                        self.last_marked = Some(num.clone());
                    }
                }
            }
        }

        fn set_bingo_status(&mut self) {
            let num_columns = self.data.get(0).unwrap().len();
            let num_rows = self.data.len();

            let mut marked_columns: Vec<Vec<&Num>> =
                vec![Vec::with_capacity(num_rows); num_columns];

            for row in &self.data {
                let marked: Vec<(usize, &Num)> =
                    row.iter().enumerate().filter(|&(_i, n)| n.marked).collect();

                for &(i, num) in &marked {
                    marked_columns[i].push(num);
                }

                if marked.len() == num_columns {
                    self.bingo = true;
                    break;
                }
            }

            for column in marked_columns {
                if column.len() == num_rows {
                    self.bingo = true;
                    break;
                }
            }
        }

        fn score(&self) -> u32 {
            let sum = self
                .data
                .iter()
                .flatten()
                .filter(|&n| !n.marked)
                .map(|n| n.data)
                .sum::<u32>();

            sum * self.last_marked.as_ref().unwrap().data
        }

        fn reset(&mut self) {
            for row in &mut self.data {
                for num in row {
                    num.marked = false;
                }
            }

            self.bingo = false;
            self.last_marked = None;
        }
    }

    impl FromIterator<Vec<Num>> for Board {
        fn from_iter<T: IntoIterator<Item = Vec<Num>>>(iter: T) -> Self {
            let mut b = Board {
                data: Vec::new(),
                bingo: false,
                last_marked: None,
            };

            for row in iter {
                b.data.push(row);
            }

            b
        }
    }

    let mut boards: Vec<Board> = boards_input
        .split("\n\n")
        .map(|b| {
            b.split("\n")
                .map(|br| {
                    br.split(" ")
                        .filter(|s| !s.is_empty())
                        .map(|n| Num {
                            data: n.parse().unwrap(),
                            marked: false,
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut found_bingo = false;

    for draw in draws.iter() {
        if found_bingo {
            break;
        }

        for board in &mut boards {
            board.mark(*draw);
            board.set_bingo_status();

            if board.bingo {
                found_bingo = true;
                break;
            }
        }
    }

    let winning_board = boards.iter().find(|&b| b.bingo).unwrap();
    println!("day 4 part 1 answer: {}", winning_board.score());

    for board in &mut boards {
        board.reset();
    }

    let mut boards_without_bingo = boards.len();

    for draw in draws {
        let mut current_bingo_board_idx = None;
        for (i, board) in boards.iter_mut().enumerate() {
            let prev_bingo_status = board.bingo;

            board.mark(draw);
            board.set_bingo_status();

            if board.bingo && !prev_bingo_status {
                current_bingo_board_idx = Some(i);
                boards_without_bingo -= 1;

                if boards_without_bingo == 0 {
                    break;
                }
            }
        }

        if boards_without_bingo == 0 {
            for i in 0..boards.len() {
                if i != current_bingo_board_idx.unwrap() {
                    boards.get_mut(i).unwrap().bingo = false;
                }
            }

            break;
        }
    }

    let winning_board = boards.iter().find(|&b| b.bingo).unwrap();

    println!("day 4 part 2 answer: {}", winning_board.score());
}

fn day5() {
    struct Grid {
        data: Vec<u32>,
        height: u32,
    }

    impl Grid {
        fn new(height: u32, width: u32) -> Grid {
            Grid {
                data: vec![0; (height * width) as usize],
                height,
            }
        }

        fn trace(&mut self, lines: Vec<Vec<Vec<u32>>>) {
            for line in lines {
                let mut points = Vec::new();

                let line_start = (line[0][0], line[0][1]);
                let line_end = (line[1][0], line[1][1]);

                let (mut x, mut y) = line_start;

                loop {
                    points.push((x, y));

                    let old_x = x;
                    let old_y = y;

                    if x < line_end.0 {
                        x += 1;
                    }

                    if x > line_end.0 {
                        x -= 1;
                    }

                    if y < line_end.1 {
                        y += 1;
                    }

                    if y > line_end.1 {
                        y -= 1;
                    }

                    if old_x == x && old_y == y {
                        break;
                    }
                }

                for point in points {
                    self.mark(point.0, point.1);
                }
            }
        }

        fn mark(&mut self, x: u32, y: u32) {
            self.data[(self.height * y + x) as usize] += 1;
        }

        fn overlaps(&self) -> usize {
            self.data.iter().filter(|&&e| e >= 2).count()
        }
    }

    impl fmt::Display for Grid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                self.data
                    .iter()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, element)| {
                        acc = match element {
                            0 => format!("{}{}", acc, '.'),
                            _ => format!("{}{}", acc, element),
                        };
                        if i != 0 && (i + 1) % self.height as usize == 0 {
                            acc = format!("{}{}", acc, '\n');
                        }

                        acc
                    })
            )
        }
    }

    let input = fs::read_to_string("input/day5.txt").unwrap();

    let lines: Vec<Vec<Vec<u32>>> = input
        .lines()
        .map(|l| {
            l.split("->")
                .map(|point| {
                    point
                        .trim()
                        .split(',')
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut grid_height: u32 = 0;
    let mut grid_width: u32 = 0;

    for line in &lines {
        for point in line {
            let &x = point.get(0).unwrap();
            let &y = point.get(1).unwrap();

            if y > grid_height {
                grid_height = y;
            }

            if x > grid_width {
                grid_width = x;
            }
        }
    }

    grid_height += 1;
    grid_width += 1;

    let mut grid = Grid::new(grid_height, grid_width);

    let relevant_lines: Vec<Vec<Vec<u32>>> = lines
        .iter()
        .filter(|&line| {
            line[0].iter().zip(line[1].iter()).any(
                |(p1_ordered_pair_part, p2_ordered_pair_part)| {
                    p1_ordered_pair_part == p2_ordered_pair_part
                },
            )
        })
        .cloned()
        .collect();

    grid.trace(relevant_lines);

    println!("day 5 part 1 answer: {}", grid.overlaps());

    let mut grid = Grid::new(grid_height, grid_width);
    grid.trace(lines);

    println!("day 5 part 2 answer: {}", grid.overlaps());
}

fn day6() {
    struct Lanternfish {
        countdown: u32,
    }

    impl Lanternfish {
        fn new(countdown: u32) -> Lanternfish {
            Lanternfish { countdown }
        }

        fn live_life(&mut self) -> Option<Lanternfish> {
            if self.countdown == 0 {
                self.countdown = 6;
                return Some(Lanternfish::new(8));
            }

            self.countdown -= 1;

            None
        }
    }

    let input = fs::read_to_string("input/day6.txt").unwrap();

    let mut fishies: Vec<Lanternfish> = input
        .split(',')
        .map(|countdown| Lanternfish::new(countdown.parse().unwrap()))
        .collect();

    for _day in 0..80 {
        let mut baby_fishies: Vec<Lanternfish> = Vec::new();
        for fishy in &mut fishies {
            if let Some(baby_fishy) = fishy.live_life() {
                baby_fishies.push(baby_fishy);
            }
        }

        fishies.append(&mut baby_fishies);
    }

    println!("day 6 part 1 answer: {}", fishies.into_iter().count());

    let fishies: Vec<Lanternfish> = input
        .split(',')
        .map(|countdown| Lanternfish::new(countdown.parse().unwrap()))
        .collect();

    let mut fishies_by_age: [u64; 9] = fishies.into_iter().fold([0; 9], |mut acc, fishy| {
        acc[fishy.countdown as usize] += 1;
        acc
    });

    for _day in 0..256 {
        let fishy_creators = fishies_by_age[0];
        for i in 0..9 {
            if i == 0 {
                continue;
            }
            fishies_by_age[i - 1] = fishies_by_age[i];
        }

        fishies_by_age[6] += fishy_creators;
        fishies_by_age[8] = fishy_creators;
    }

    println!(
        "day 6 part 2 answer: {}",
        fishies_by_age.into_iter().sum::<u64>()
    );
}

fn day7() {
    let input = fs::read_to_string("input/day7.txt").unwrap();

    let mut crabs: Vec<u32> = input.split(',').map(|c| c.parse().unwrap()).collect();

    crabs.sort();

    let crabs_size = crabs.len();

    let median = crabs[crabs_size / 2];

    let mut fuel_spent = 0;

    for &crab in &crabs {
        if crab > median {
            fuel_spent += crab - median;
        }

        if crab < median {
            fuel_spent += median - crab;
        }
    }

    println!("day 7 part 1 answer: {}", fuel_spent);

    let mean: u32 = crabs.iter().sum::<u32>() / crabs_size as u32;

    let mut fuel_candiates: (u32, u32) = (0, 0);

    for &crab in &crabs {
        let distance_candidates: (u32, u32) = (
            (crab as i32 - mean as i32).abs() as u32,
            (crab as i32 - (mean + 1) as i32).abs() as u32,
        );

        fuel_candiates.0 += (1..=distance_candidates.0).sum::<u32>();
        fuel_candiates.1 += (1..=distance_candidates.1).sum::<u32>();
    }

    let optimal_fuel_expenditure = min(fuel_candiates.0, fuel_candiates.1);

    println!("day 7 part 2 answer: {}", optimal_fuel_expenditure);
}

fn day8() {
    let input = fs::read_to_string("input/day8example.txt").unwrap();

    let displays: Vec<Vec<Vec<&str>>> = input
        .lines()
        .map(|l| {
            l.split('|')
                .map(|p| p.trim().split(' ').collect())
                .collect()
        })
        .collect();

    let unique_segment_digit_count = displays
        .iter()
        .map(|d| d.get(1).unwrap())
        .flatten()
        .filter(|&out| out.len() == 2 || out.len() == 3 || out.len() == 4 || out.len() == 7)
        .count();

    println!("day 8 part 1 answer: {}", unique_segment_digit_count);

    let segments_on: Vec<u32> = displays
        .into_iter()
        .flatten()
        .flatten()
        .flat_map(|pattern| {
            let mut map: [u32; 7] = [0; 7];

            for char in pattern.chars() {
                match char {
                    'a' => map[0] = 1,
                    'b' => map[1] = 1,
                    'c' => map[2] = 1,
                    'd' => map[3] = 1,
                    'e' => map[4] = 1,
                    'f' => map[5] = 1,
                    'g' => map[6] = 1,
                    _ => (),
                };
            }

            map
        })
        .collect();

    const SEGMENT_LEN: usize = 7;
    const ROW_LEN: usize = 14;

    let mut current_row_start = 0;
    let mut current_row_end = SEGMENT_LEN * ROW_LEN;

    while current_row_end <= segments_on.len() {
        let mut segment_sums: [u32; ROW_LEN] = [0; ROW_LEN];

        let mut row_segment_map: [&str; SEGMENT_LEN] = [""; SEGMENT_LEN];

        let mut current_segment_start = current_row_start;
        let mut current_segment_end = current_row_start + SEGMENT_LEN;

        for i in 0..segment_sums.len() {
            segment_sums[i] = (&segments_on[current_segment_start..current_segment_end])
                .iter()
                .sum::<u32>();

            current_segment_start = current_segment_end;
            current_segment_end += SEGMENT_LEN;
        }

        println!("segment_sums: {:?}", segment_sums);

        current_row_start = current_row_end;
        current_row_end += SEGMENT_LEN * ROW_LEN;
    }
}

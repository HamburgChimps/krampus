use std::{fs, vec};

fn main() {
    day1();
    day2();
    day3();
    day4();
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
            self.data
                .iter()
                .flatten()
                .filter(|&n| !n.marked)
                .map(|n| n.data)
                .sum::<u32>()
                * self.last_marked.as_ref().unwrap().data
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

    for draw in draws {
        let mut current_bingo_board_idx = None;
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(draw);
            board.set_bingo_status();

            if board.bingo {
                current_bingo_board_idx = Some(i);
            }
        }

        let boards_without_bingo: Vec<&Board> = boards.iter().filter(|&b| !b.bingo).collect();

        if boards_without_bingo.len() == 0 {
            for i in 0..boards.len() {
                if i != current_bingo_board_idx.unwrap() {
                    boards.get_mut(i).unwrap().bingo = false;
                }
            }
        }
    }

    let winning_board = boards.iter().find(|&b| b.bingo).unwrap();

    println!("winning board: {:#?}", winning_board);
    println!("day 4 part 2 answer: {}", winning_board.score());
}

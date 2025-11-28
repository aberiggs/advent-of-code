fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        grid.push(row);
    }
    return grid;
}

pub fn part1(input: &str) {
    let grid = parse_input(input);

    let mut num_safe = 0;

    for row in grid {
        if is_row_safe(&row, false) {
            num_safe += 1;
        }
    }

    println!("Number of safe reports for part 1: {}", num_safe);
}

pub fn part2(input: &str) {
    let grid = parse_input(input);
    let mut num_safe = 0;

    for row in grid {
        if is_row_safe(&row, true) {
            num_safe += 1;
        }
    }

    println!("Number of safe reports for part 2: {}", num_safe);
}

#[derive(PartialEq)]
enum Order {
    Increasing,
    Decreasing,
}

fn options_after_bad_level(row: &Vec<i32>, index: usize) -> (Vec<i32>, Vec<i32>) {
    // Take the example [1, 10, 5, 6, 7]
    // We initial encounter a bad level at 5, but if we remove it it's not safe (10 to 6 is not increasing)
    // However, if we remove 10, it's safe (1 to 6 is increasing)
    // Now take the example [1, 10, 5, 11, 12]
    // In this case, we obviously should remove 5
    // Thus we need to consider the options where we remove the level at i or at i-1.

    let option_a: Vec<i32> = row[..index - 1]
        .iter()
        .chain(&row[index..])
        .copied()
        .collect();
    let option_b: Vec<i32> = row[..index]
        .iter()
        .chain(&row[index + 1..])
        .copied()
        .collect();
    (option_a, option_b)
}

fn follows_order(current_val: i32, prev_val: i32, order: &Order) -> bool {
    if *order == Order::Increasing {
        return current_val > prev_val;
    } else {
        return current_val < prev_val;
    }
}

fn is_row_safe(row: &Vec<i32>, tolerate_bad_level: bool) -> bool {
    let order = if row[0] < row[row.len() - 1] {
        Order::Increasing
    } else {
        Order::Decreasing
    };

    for i in 1..row.len() {
        let diff = (row[i] - row[i - 1]).abs();
        if diff > 3 {
            // Failure by jump
            if tolerate_bad_level {
                let (option_a, option_b) = options_after_bad_level(row, i);
                if is_row_safe(&option_a, false) || is_row_safe(&option_b, false) {
                    return true;
                }
            }

            return false;
        }

        if !follows_order(row[i], row[i - 1], &order) {
            // Failure by order
            if tolerate_bad_level {
                let option_a: Vec<i32> = row[..i - 1].iter().chain(&row[i..]).copied().collect();
                let option_b: Vec<i32> = row[..i].iter().chain(&row[i + 1..]).copied().collect();
                if is_row_safe(&option_a, false) || is_row_safe(&option_b, false) {
                    return true;
                }
            }

            return false;
        }
    }

    return true;
}

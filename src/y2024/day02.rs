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
        let mut safe_diff = true;
        let mut is_increasing = true;
        let mut is_decreasing = true;

        let mut prev_value = row[0];
        for i in 1..row.len() {
            let num = row[i];

            if num > prev_value {
                is_decreasing = false;
            } else if num < prev_value {
                is_increasing = false;
            }

            let diff = (prev_value - num).abs();

            if diff > 3 || diff < 1 {
                safe_diff = false;
                break;
            }

            prev_value = num;
        }

        if (is_increasing || is_decreasing) && safe_diff {
            num_safe += 1;
        }
    }

    println!("Number of safe reports for part 1: {}", num_safe);
}

// TODO: Unfinished
pub fn part2(input: &str) {
    let grid = parse_input(input);
    let mut num_safe = 0;

    println!("Number of safe reports for part 2: {}", num_safe);
}

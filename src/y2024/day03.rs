// NOTE: Could possibly use regex for this problem, but going to do a more manual approach
pub fn part1(input: &str) {
    let mut result = 0;

    for (i, matched) in input.match_indices("mul(") {
        let start_index = i + matched.len();

        let substr = &input[start_index..];
        // Look for the first occurrence of ',' and ')', which should complete the mul() expression
        let comma_index_opt = substr.find(',');
        let close_paren_index_opt = substr.find(')');
        if comma_index_opt.is_none() || close_paren_index_opt.is_none() {
            continue;
        }

        let comma_index = comma_index_opt.unwrap();
        let close_paren_index = close_paren_index_opt.unwrap();

        if comma_index > close_paren_index {
            // Comma must come before the closing parenthesis
            continue;
        }

        let num_str_a = &substr[..comma_index];
        let num_str_b = &substr[comma_index + 1..close_paren_index];

        let num_a_res = num_str_a.parse::<i32>();
        let num_b_res = num_str_b.parse::<i32>();
        if num_a_res.is_err() || num_b_res.is_err() {
            continue;
        }

        let num_a = num_a_res.unwrap();
        let num_b = num_b_res.unwrap();

        result += num_a * num_b;
    }

    println!("Part 1 result: {}", result);
}

pub fn part2(input: &str) {
    let mut result = 0;

    let mut do_indices: Vec<usize> = Vec::new();
    do_indices.push(0);
    for (i, _) in input.match_indices("do()") {
        do_indices.push(i);
    }
    let mut dont_indices: Vec<usize> = Vec::new();
    for (i, _) in input.match_indices("don't()") {
        dont_indices.push(i);
    }
    dont_indices.push(input.len());

    let mut valid_ranges: Vec<(usize, usize)> = Vec::new();

    let mut dont_index = 0;
    for do_index in do_indices {
        if dont_index >= dont_indices.len() {
            break;
        }

        while dont_index < dont_indices.len() && dont_indices[dont_index] < do_index {
            dont_index += 1;
        }

        if dont_index >= dont_indices.len() {
            break;
        }

        valid_ranges.push((do_index, dont_indices[dont_index]));
    }

    for (i, matched) in input.match_indices("mul(") {
        let start_index = i + matched.len();

        // Check if the start index is within a valid range
        let mut is_valid = false;
        for &(start, end) in &valid_ranges {
            if start_index > start && start_index < end {
                // The start index is within a valid range
                is_valid = true;
                break;
            }
        }

        if !is_valid {
            continue;
        }

        let substr = &input[start_index..];
        // Look for the first occurrence of ',' and ')', which should complete the mul() expression
        let comma_index_opt = substr.find(',');
        let close_paren_index_opt = substr.find(')');
        if comma_index_opt.is_none() || close_paren_index_opt.is_none() {
            continue;
        }

        let comma_index = comma_index_opt.unwrap();
        let close_paren_index = close_paren_index_opt.unwrap();

        if comma_index > close_paren_index {
            // Comma must come before the closing parenthesis
            continue;
        }

        let num_str_a = &substr[..comma_index];
        let num_str_b = &substr[comma_index + 1..close_paren_index];

        let num_a_res = num_str_a.parse::<i32>();
        let num_b_res = num_str_b.parse::<i32>();
        if num_a_res.is_err() || num_b_res.is_err() {
            continue;
        }

        let num_a = num_a_res.unwrap();
        let num_b = num_b_res.unwrap();

        result += num_a * num_b;
    }

    println!("Part 2 result: {}", result);
}

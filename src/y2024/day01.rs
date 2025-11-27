use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let (left_str, right_str) = line.split_once(" ").unwrap();
        left.push(left_str.trim().parse().unwrap());
        right.push(right_str.trim().parse().unwrap());
    }

    return (left, right);
}

pub fn part1(input: &str) {
    let (mut left, mut right) = parse_input(input);

    // Sort the left and right vectors
    left.sort_unstable();
    right.sort_unstable();

    assert_eq!(left.len(), right.len());

    let mut dist = 0;

    for i in 0..left.len() {
        dist += (left[i] - right[i]).abs();
    }

    println!("Total distance: {}", dist);
}

pub fn part2(input: &str) {
    let (left, right) = parse_input(input);

    let mut right_map = HashMap::new();

    // Create a hashmap from the right vector
    for value in right {
        if right_map.contains_key(&value) {
            right_map.insert(value, right_map.get(&value).unwrap() + 1);
        } else {
            right_map.insert(value, 1);
        }
    }

    let mut similarity = 0;

    for value in left {
        if right_map.contains_key(&value) {
            similarity += value * right_map.get(&value).unwrap();
        }
    }

    println!("Total similarity: {}", similarity);
}

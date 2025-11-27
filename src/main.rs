mod y2024;

fn main() {
    // TODO: Make this a command line argument
    let year = 2024;
    // TODO: Make this a command line argument
    let day = 1;

    match year {
        2024 => match day {
            1 => {
                let day01_input = include_str!("../input/y2024/day01.txt");
                y2024::day01::part1(day01_input);
                y2024::day01::part2(day01_input);
            }
            _ => {
                println!("Day {} not found for year {}", day, year);
            }
        },
        _ => {
            println!("Year {} not found", year);
        }
    }
}

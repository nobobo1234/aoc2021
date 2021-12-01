use std::fs;
use std::error::Error;

const INPUT: &str = "inputs/day01.txt";

fn read_file_as_string() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(INPUT)?;
    Ok(contents)
}

pub fn part1() -> Result<i32, &'static str> {
    let file = read_file_as_string().map_err(|_| "Could not read file")?;
    let lines = file.lines();

    let mut increased = 0;
    let mut previous = -1;
    for line in lines {
        let integer: i32 = line.parse().unwrap();
        if previous >= 0 && previous < integer {
            increased += 1;
        }
        previous = integer;
    }

    Ok(increased)
}

pub fn part2() -> Result<i32, &'static str> {
    let file = read_file_as_string().map_err(|_| "Could not read file")?;
    let lines = file.lines();

    let mut increased = 0;
    let mut total_index = 0;
    let mut previous_index = 0;
    let mut previous_items = vec![-1, -1, -1];
    let mut previous_sum = -1;
    for line in lines {
        let integer: i32 = line.parse().unwrap();
        previous_items[previous_index] = integer;

        let sum = previous_items.iter().sum();
        if previous_sum < sum && total_index > 2 {
            increased += 1;
        }
        previous_sum = sum;

        previous_index = (previous_index + 1) % 3;
        total_index += 1;
    }

    Ok(increased)
}

use std::fs;
use std::error::Error;

const INPUT: &str = "inputs/day02.txt";

fn read_file_as_string() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(INPUT)?;
    Ok(contents)
}

pub fn part1() -> Result<i64, &'static str> {
    let file = read_file_as_string().map_err(|_| "Could not read file")?;
    let lines = file.lines();

    let mut depth: i64 = 0;
    let mut position = 0;
    for line in lines {
        let mut iter = line.split_whitespace();
        let direction = iter.next().unwrap();
        let size: i64 = match iter.next() {
            None => 0,
            Some(value) => value.parse::<i64>().unwrap()
        };

        match direction {
            "up" => {
                depth -= size;
            },
            "down" => {
                depth += size;
            },
            "forward" => {
                position += size;
            },
            _ => ()
        };
    }

    Ok(depth * position)
}
pub fn part2() -> Result<i64, &'static str> {
    let file = read_file_as_string().map_err(|_| "Could not read file")?;
    let lines = file.lines();

    let mut depth: i64 = 0;
    let mut position = 0;
    let mut aim = 0;
    for line in lines {
        let mut iter = line.split_whitespace();
        let direction = iter.next().unwrap();
        let size: i64 = match iter.next() {
            None => 0,
            Some(value) => value.parse::<i64>().unwrap()
        };

        match direction {
            "up" => {
                aim -= size;
            },
            "down" => {
                aim += size;
            },
            "forward" => {
                position += size;
                depth += aim * size;
            },
            _ => ()
        };
    }

    Ok(depth * position)
}

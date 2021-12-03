use std::fs;
use std::error::Error;

const INPUT: &str = "inputs/day03.txt";

fn read_file_as_string() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(INPUT)?;
    Ok(contents)
}

pub fn part1() -> Result<i64, &'static str> {
    let file = read_file_as_string().map_err(|_| "Could not read file")?;
    let length: i64 = file.lines().clone().count().try_into().unwrap();

    let solution = file.lines()
        // Converting every line into a vector of integers.
        .map(|e| e.chars().map(|e| e.to_digit(10).unwrap() as i64).collect())
        // Summing every ith number in every binary number
        .fold(vec![0; 12], |sum, e: Vec<i64>| -> Vec<i64> {
            sum.iter().zip(e).map(|(sum_el, curr_el)| sum_el + curr_el).collect()
        })
        .iter()
        // If the e >= length / 2 then 1 is the most common, otherwise 0.
        // Change every element so that it contains the most common bit.
        .map(|e| if e >= &(length / 2) { 1 } else { 0 })
        // Convert the thing to the oxygen and co2 in a vector. Both are
        // opposites so e - 1 will be the opposite of e.
        .fold([String::new(), String::new()], |[s1, s2], e| {
            [format!("{}{}", &s1, e), format!("{}{}", &s2, e - 1)]
        })
        // Convert every binary string to an integer
        .map(|e| {println!("{}", e); i64::from_str_radix(&e, 2).unwrap()})
        .into_iter()
        // Take the product of the vector.
        .product();

    Ok(solution)
}

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
            [format!("{}{}", &s1, e), format!("{}{}", &s2, if e == 1 { 0 } else { 1 })]
        })
        // Convert every binary string to an integer
        .map(|e| i64::from_str_radix(&e, 2).unwrap())
        .into_iter()
        // Take the product of the vector.
        .product();

    Ok(solution)
}

pub fn part2() -> Result<i64, &'static str> {
    let file = read_file_as_string().map_err(|_| "Could not read file")?;

    let lines: Vec<Vec<i64>> = file.lines()
        // Converting every binary into a vector of integers.
        .map(|e| e.chars().map(|e| e.to_digit(10).unwrap() as i64).collect())
        .collect();

    // Create an array with the lines two times, one for the oxygen number and
    // one for the co2 number.
    let mut oxyco2 = [lines.clone(), lines.clone()];

    let result = oxyco2.iter_mut()
        // Enumerate over the two lists to get the two indexes
        .enumerate()
        .map(|(i, list)| {
            // Loop over every possible position in the binary numbers.
            let mut cursor = 0;
            while cursor < 12 {
                // Find most common bit at cursor position by checking if sum of
                // all bits is >= half of the length.
                let half_length: i64 = (list.len() / 2).try_into().unwrap();
                let common = list.iter()
                    .fold(0, |sum, e| sum + e[cursor]) >= half_length;

                // Now 1 = true and 0 = false, turn it into an i64.
                let mut common: i64 = common.try_into().unwrap();

                // We want to invert it for the second list.
                if i == 1 {
                    common = if common == 1 { 0 } else { 1 };
                }

                // Filter out all the bits that have the most common bit at the
                // cursor position.
                if list.len() > 1 {
                    list.retain(|number| number[cursor] == common);
                }

                cursor += 1;
            }

            list
                .iter()
                // Convert all the vectors with individual bits as numbers to
                // bits as strings.
                .map(|e| {
                    e.iter()
                        .map(|e| e.to_string())
                        .collect()
                }).collect()
        })
        // Convert the only left-over (aka the first element) of each list from
        // binary to number
        .map(|e: Vec<String>| i64::from_str_radix(e.first().unwrap(), 2).unwrap())
        // Take the product of the list.
        .product();

    Ok(result)
}

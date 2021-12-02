mod day01;
mod day02;

fn main() {
    println!("Running day 1:");

    let part1 = match day01::part1() {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {}", e);
            -1
        }
    };
    let part2 = match day01::part2() {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {}", e);
            -1
        }
    };

    println!("Solution to part 1: {}", part1);
    println!("Solution to part 2: {}", part2);
    println!("Running day 1:");

    let part1 = match day02::part1() {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {}", e);
            -1
        }
    };
    let part2 = match day02::part2() {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {}", e);
            -1
        }
    };

    println!("Solution to part 1: {}", part1);
    println!("Solution to part 2: {}", part2);
}

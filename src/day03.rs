use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn solve_day_three() -> u32 {
    solve("src/input/day03.txt")
}

fn solve(filename: &str) -> u32 {
    // Stream lines from file
    let file = File::open(filename).unwrap();
    let buffer = io::BufReader::new(file);
    let lines = buffer.lines();

    // Get the maximum voltage for each line.
    let max_voltages = lines.map(|line| determine_max_voltage(line.unwrap()));

    // Sum the results
    max_voltages.sum()
}

fn determine_max_voltage(s: String) -> u32 {
    let mut left_digit = 0;
    let mut right_digit = 0;

    for c in s.chars().rev() {
        match c.to_digit(10) {
            None => {
                println!("Could not parse {c}");
            }
            Some(num) => {
                if right_digit == 0 {
                    right_digit = num;
                } else if left_digit == 0 {
                    left_digit = num;
                } else if num >= left_digit {
                    if right_digit < left_digit {
                        right_digit = left_digit;
                    }
                    left_digit = num;
                }
            }
        }
    }

    let result = 10 * left_digit + right_digit;
    println!("{s} -> {result}");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_three_example() {
        assert_eq!(solve("src/input/day03test.txt"), 357);
    }

    #[test]
    fn test_solve_day_three() {
        assert_eq!(solve_day_three(), 17281);
    }
}

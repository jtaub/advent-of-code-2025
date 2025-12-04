use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn solve_day_three_part_two() -> i64 {
    solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day03.txt")
}

fn solve(filename: &str) -> i64 {
    // Stream lines from file
    let file = File::open(filename).unwrap();
    let buffer = io::BufReader::new(file);
    let lines = buffer.lines();

    // Get the maximum voltage for each line.
    let max_voltages = lines.map(|line| determine_max_voltage(line.unwrap()));

    // Sum the results
    max_voltages.sum()
}

/// The max voltage is the largest 12 digit number you can make from the digits in the input,
/// reading the string from left to right.
fn determine_max_voltage(s: String) -> i64 {
    let mut to_remove = s.len() - 12;
    let mut stack: Vec<u32> = Vec::new();

    for c in s.chars() {
        let num = c.to_digit(10).unwrap();

        while to_remove > 0 && !stack.is_empty() && stack.last().unwrap() < &num {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(num);
    }

    // Join the numbers
    stack
        .iter()
        .take(12)
        .fold(0i64, |acc, num| 10 * acc + *num as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_three_example() {
        assert_eq!(
            solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day03test.txt"),
            3121910778619
        );
    }

    #[test]
    fn test_solve_day_three() {
        // this answer is too low
        assert_eq!(solve_day_three_part_two(), 171388730430281);
    }
}

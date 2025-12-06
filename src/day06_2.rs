pub fn solve_day_six_part_two() -> u64 {
    solve("src/input/day06.txt")
}

fn solve(input: &str) -> u64 {
    // Load the entire file into memory as a 2D grid
    let file_contents = std::fs::read_to_string(input).expect("Failed to read input file");
    let input = file_contents.as_str();
    let num_lines = input.lines().count();

    // Parse the operations, pretty easy.
    let operations: Vec<char> = input
        .lines()
        .last()
        .expect("Input was empty")
        .split_ascii_whitespace()
        .map(|s| match s.chars().next() {
            Some('+') => '+',
            Some('*') => '*',
            Some(other) => panic!("Invalid operation {other}"),
            None => panic!("Got empty string"),
        })
        .collect();

    // Get the numbers.
    let number_string = input
        .lines()
        .take(num_lines - 1) // filter out the operations line
        .collect::<Vec<&str>>()
        .join("\n");
    let numbers = read_vertical_ints(number_string.as_str());
    assert_eq!(numbers.len(), operations.len());

    operations
        .iter()
        .zip(numbers)
        .map(|(op, nums)| match op {
            '+' => nums.iter().map(|&n| n as u64).sum::<u64>(),
            '*' => nums.iter().map(|&n| n as u64).product::<u64>(),
            _ => panic!("Invalid operation {op}"),
        })
        .sum::<u64>()
}

fn read_vertical_ints(s: &str) -> Vec<Vec<u32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_six_part_two_example() {
        assert_eq!(solve("src/input/day06test.txt"), 3263827);
    }

    #[test]
    fn test_solve_day_six_part_two() {
        assert_eq!(solve_day_six_part_two(), 0);
    }
}

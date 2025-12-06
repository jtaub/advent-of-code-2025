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
    let lines: Vec<&str> = s.lines().filter(|line| !line.trim().is_empty()).collect();
    if lines.is_empty() {
        return Vec::new();
    }

    // Pad all lines to same length
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded_lines: Vec<String> = lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_len))
        .collect();

    // Find column groups by identifying word boundaries
    // A column is part of a word if any row has a digit there
    let in_word: Vec<bool> = (0..max_len)
        .map(|col| {
            padded_lines.iter().any(|line| {
                line.chars()
                    .nth(col)
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
            })
        })
        .collect();

    // Group consecutive "in_word" columns
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut current_group: Vec<usize> = Vec::new();
    for (col, &is_digit_col) in in_word.iter().enumerate() {
        if is_digit_col {
            current_group.push(col);
        } else if !current_group.is_empty() {
            groups.push(current_group);
            current_group = Vec::new();
        }
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }

    // For each group, read each column vertically to form integers
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .filter_map(|&col| {
                    let digits: String = padded_lines
                        .iter()
                        .filter_map(|line| line.chars().nth(col).filter(|c| c.is_ascii_digit()))
                        .collect();
                    if digits.is_empty() {
                        None
                    } else {
                        digits.parse::<u32>().ok()
                    }
                })
                .collect()
        })
        .collect()
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
        assert_eq!(solve_day_six_part_two(), 11602774058280);
    }
}

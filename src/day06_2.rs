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

    // Convert to Vec<Vec<u8>> for O(1) column access.
    // Since we only care about ASCII digits and spaces, bytes work fine.
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded_lines: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| {
            let mut bytes = l.as_bytes().to_vec();
            bytes.resize(max_len, b' '); // pad with spaces
            bytes
        })
        .collect();

    // Find column groups by identifying word boundaries
    // A column is part of a word if any row has a digit there
    let in_word: Vec<bool> = (0..max_len)
        .map(|col| padded_lines.iter().any(|line| line[col].is_ascii_digit()))
        .collect();

    // Group consecutive "in_word" columns into (start, end) ranges
    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut start: Option<usize> = None;
    for (col, &is_digit_col) in in_word.iter().enumerate() {
        if is_digit_col {
            if start.is_none() {
                start = Some(col);
            }
        } else if let Some(s) = start {
            groups.push((s, col));
            start = None;
        }
    }
    if let Some(s) = start {
        groups.push((s, max_len));
    }

    // For each group, read each column vertically to form integers
    // Compute digit values directly instead of building strings
    groups
        .iter()
        .map(|&(start, end)| {
            (start..end)
                .filter_map(|col| {
                    let mut value: u32 = 0;
                    let mut has_digit = false;
                    for line in &padded_lines {
                        let byte = line[col];
                        if byte.is_ascii_digit() {
                            value = value * 10 + (byte - b'0') as u32;
                            has_digit = true;
                        }
                    }
                    if has_digit { Some(value) } else { None }
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

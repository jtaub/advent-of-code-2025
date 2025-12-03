pub fn solve_day_two_part_two() -> i64 {
    solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day02.txt")
}

fn solve(filename: &str) -> i64 {
    // Parse the input
    let input_string = std::fs::read_to_string(filename).unwrap();

    // Convert the input to an iterator of (start, end) range pairs.
    let ranges = input_string.split(',').map(|s| parse_range(s));

    // Find the invalid IDs in each range.
    let invalid_id_sums = ranges.map(|(start, end)| sum_invalid_ids(start, end));

    // Sum the invalid IDs.
    invalid_id_sums.sum()
}

/// Converts a range string like "1-3" into a (start, end) pair.
fn parse_range(range: &str) -> (i64, i64) {
    let mut split = range.split('-');
    (
        split.next().unwrap().parse::<i64>().unwrap(),
        split.next().unwrap().parse::<i64>().unwrap(),
    )
}

/// Returns the sum of all invalid IDs in the given range.
fn sum_invalid_ids(start: i64, end: i64) -> i64 {
    (start..=end).filter(|&n| is_invalid(n)).sum()
}

/// Returns true if the given ID is invalid, which in this case means it is formed by concatenating a number repeatedly.
/// ```
fn is_invalid(n: i64) -> bool {
    let length = n.ilog10() + 1;
    let result = (2..=length).any(|num_parts| split_and_compare(n, length, num_parts));
    result
}

fn split_and_compare(n: i64, length: u32, num_parts: u32) -> bool {
    // If the number cannot be divided evenly into the specified number of parts, it cannot be invalid.
    if length % num_parts != 0 {
        return false;
    }

    let chunk_size = 10i64.pow(length / num_parts);
    let mut remaining = n;
    let mut prev: i64 = -1;

    for _ in 0..num_parts {
        let part = remaining % chunk_size;
        if prev != -1 && part != prev {
            return false;
        }
        prev = part;
        remaining /= chunk_size;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_two_example() {
        assert_eq!(
            solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day02test.txt"),
            4174379265,
        );
    }

    #[test]
    fn test_solve_day_two() {
        assert_eq!(solve_day_two_part_two(), 50857215650);
    }
}

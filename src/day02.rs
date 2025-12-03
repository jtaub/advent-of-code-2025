pub fn solve_day_two() -> i64 {
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
    (start..=end).filter(|&n| is_invalid(n)).map(|n| n as i64).sum()
}

/// Returns true if the given ID is invalid, which in this case means it is formed by concatenating two numbers repeatedly.
///
/// Examples:
/// ```rust
/// assert!(is_invalid(11));
/// assert!(is_invalid(22));
/// assert!(is_invalid(123123));
/// assert!(!is_invalid(1));
/// assert!(!is_invalid(12312));
/// ```
fn is_invalid(n: i64) -> bool {
    // If the number is odd length, then it cannot be invalid.
    let length = n.ilog10() + 1;
    if length % 2 == 1 {
        return false;
    }

    let rightmost_digits = n % (10i64.pow(length / 2));
    let leftmost_digits = n / (10i64.pow(length / 2));
    rightmost_digits == leftmost_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_two_example() {
        assert_eq!(
            solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day02test.txt"),
            1227775554
        );
    }

    #[test]
    fn test_solve_day_two() {
        assert_eq!(solve_day_two(), 40055209690);
    }
}

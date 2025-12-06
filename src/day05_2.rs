pub fn solve_day_five_part_two() -> i64 {
    solve("src/input/day05.txt")
}

fn solve(filename: &str) -> i64 {
    let mut fresh_ingredient_ranges: Vec<(i64, i64)> = std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| match s.split_once('-') {
            Some((start, end)) => (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()),
            None => panic!("Invalid input format {s}"),
        })
        .collect();

    fresh_ingredient_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    merge_intervals(fresh_ingredient_ranges)
        .iter()
        .map(|(start, end)| (end - start) + 1)
        .sum()
}

fn merge_intervals(fresh_ingredient_ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut sorted_ingredient_ranges: Vec<(i64, i64)> = Vec::new();
    let mut curr = fresh_ingredient_ranges[0];

    for i in 1..fresh_ingredient_ranges.len() {
        let next = fresh_ingredient_ranges[i];

        if curr.1 >= next.0 {
            curr = (curr.0, curr.1.max(next.1));
        } else {
            sorted_ingredient_ranges.push(curr);
            curr = next;
        }
    }

    sorted_ingredient_ranges.push(curr);
    sorted_ingredient_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_five_example() {
        assert_eq!(solve("src/input/day05test.txt"), 14);
    }

    #[test]
    fn test_solve_day_five() {
        assert_eq!(solve_day_five_part_two(), 344771884978261);
    }
}

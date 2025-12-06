pub fn solve_day_five() -> i32 {
    solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day05.txt")
}

fn solve(filename: &str) -> i32 {
    let binding = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut lines = binding.lines();

    let mut fresh_ingredient_ranges: Vec<(i64, i64)> = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| match s.split_once('-') {
            Some((start, end)) => (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()),
            None => panic!("Invalid input format {s}"),
        })
        .collect();

    fresh_ingredient_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let merged_ranges = merge_intervals(fresh_ingredient_ranges);

    let available_ingredients: Vec<i64> = lines
        .map(|s| match s.parse::<i64>() {
            Ok(num) => num,
            Err(_) => panic!("Invalid input format {s}"),
        })
        .collect();

    let mut result = 0;
    for id in available_ingredients {
        if binary_search(&merged_ranges, id) {
            result += 1;
        }
    }
    result
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

fn binary_search(ranges: &[(i64, i64)], target: i64) -> bool {
    let mut left = 0;
    let mut right = ranges.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let (start, end) = ranges[mid];

        if start <= target && target <= end {
            return true;
        } else if target < start {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_five_example() {
        assert_eq!(
            solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day05test.txt"),
            3
        );
    }

    #[test]
    fn test_solve_day_five() {
        assert_eq!(solve_day_five(), 0);
    }
}

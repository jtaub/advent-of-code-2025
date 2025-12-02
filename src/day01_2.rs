pub fn solve_day_one_part_two() -> i32 {
    solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day01.txt")
}

fn solve(filename: &str) -> i32 {
    let (count, _final_pos) = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars.next().unwrap();
            let num = chars.as_str().parse::<i32>().unwrap();
            (first, num)
        })
        .fold((0, 50), |(count, pos), (first, num)| {
            let new_pos = match first {
                'R' => pos + (num % 100),
                'L' => pos - (num % 100),
                _ => panic!("Invalid direction: {first}"),
            };

            let z = num.abs() / 100;
            let increment = if pos == 0 { 0 } else { 1 };
            let (zeroes, new_loc) = if new_pos == 0 || new_pos == 100 {
                (z + increment, 0)
            } else if new_pos > 100 {
                (z + increment, new_pos - 100)
            } else if new_pos < 0 {
                (z + increment, new_pos + 100)
            } else {
                (z, new_pos)
            };

            (zeroes + count, new_loc)
        });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day01test.txt"),
            6
        );
    }

    #[test]
    fn test_solve_day_one() {
        assert_eq!(solve_day_one_part_two(), 6616);
    }
}

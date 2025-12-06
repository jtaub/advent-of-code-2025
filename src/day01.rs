pub fn solve_day_one() -> i32 {
    solve("src/input/day01.txt")
}

fn solve(filename: &str) -> i32 {
    let (count, _final_pos) = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars.next().unwrap();
            let num = chars.as_str().parse::<i32>().unwrap();
            (first, num % 100)
        })
        .fold((0, 50), |(count, pos), (first, num)| {
            let new_pos = match first {
                'R' => pos + num,
                'L' => pos - num,
                _ => panic!("Invalid direction: {first}"),
            };

            if new_pos == 0 || new_pos == 100 {
                (count + 1, 0)
            } else if new_pos > 100 {
                (count, new_pos - 100)
            } else if new_pos < 0 {
                (count, new_pos + 100)
            } else {
                (count, new_pos)
            }
        });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("src/input/day01test.txt"), 3);
    }

    #[test]
    fn test_solve_day_one() {
        assert_eq!(solve_day_one(), 1092);
    }
}

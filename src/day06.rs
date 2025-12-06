pub fn solve_day_six() -> u64 {
    solve("/home/jtaubner/IdeaProjects/advent-of-code-2025/src/input/day06.txt")
}

fn solve(input: &str) -> u64 {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    std::fs::read_to_string(input)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .for_each(|split_iter| {
            split_iter
                .enumerate()
                .for_each(|(i, s)| match s.parse::<u64>() {
                    Ok(num) => {
                        while numbers.len() <= i {
                            numbers.push(Vec::new());
                        }
                        numbers[i].push(num);
                    }
                    Err(_) => {
                        let operation = s.chars().next().expect("Got empty string on line {i}");
                        operations.push(operation)
                    }
                })
        });

    operations
        .iter()
        .enumerate()
        .map(|(i, operation)| match operation {
            '+' => numbers[i].iter().sum::<u64>(),
            '*' => numbers[i].iter().product::<u64>(),
            _ => panic!("Invalid operation {operation}"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_six_example() {
        assert_eq!(
            solve("/home/jtaubner/IdeaProjects/advent-of-code-2025/src/input/day06test.txt"),
            4277556
        );
    }

    #[test]
    fn test_solve_day_six() {
        assert_eq!(solve_day_six(), 4583860641327);
    }
}

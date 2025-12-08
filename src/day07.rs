pub fn solve_day_seven() -> u32 {
    solve("src/input/day07.txt")
}

const SPLITTER: char = '^';
const BEAM: char = '|';

fn solve(input: &str) -> u32 {
    // Load the entire file into memory as a 2D grid
    let file_contents = std::fs::read_to_string(input).expect("Failed to read input file");
    let mut grid: Vec<Vec<char>> = file_contents.lines().map(|s| s.chars().collect()).collect();

    // Find the index of the beam
    let origin_index = grid[0]
        .iter()
        .enumerate()
        .find(|(_i, c)| **c == 'S')
        .expect("Could not find S")
        .0;

    // Initialize the first beam.
    grid[1][origin_index] = '|';

    let mut result = 0;
    for i in 1..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == SPLITTER {
                if grid[i - 1][j] == BEAM {
                    grid[i][j - 1] = BEAM;
                    grid[i][j + 1] = BEAM;
                    result += 1;
                }
            } else if grid[i - 1][j] == BEAM {
                grid[i][j] = BEAM;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_seven_example() {
        assert_eq!(solve("src/input/day07test.txt"), 21);
    }

    #[test]
    fn test_solve_day_seven() {
        assert_eq!(solve_day_seven(), 1518);
    }
}

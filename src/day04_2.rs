use std::{
    fs::File,
    io::{self, BufRead},
};

const PAPER: char = '@';

pub fn solve_day_four_part_two() -> i32 {
    solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day04.txt")
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn solve(filename: &str) -> i32 {
    // Read the contents of the file into a 2D char array.
    let mut grid = read_file_contents(filename);

    // Solve the puzzle using the grid.
    let mut result = 0;

    loop {
        let papers_removed = count_papers(&mut grid);
        if papers_removed == 0 {
            break;
        }
        result += papers_removed;
    }

    result
}

fn count_papers(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != PAPER {
                continue;
            }

            let mut adjacent_count = 0;

            for (dx, dy) in DIRECTIONS {
                if let (Some(x), Some(y)) = (i.checked_add_signed(dx), j.checked_add_signed(dy)) {
                    if x < grid.len() && y < grid[x].len() && grid[x][y] == PAPER {
                        adjacent_count += 1;
                    }
                }
            }

            if adjacent_count < 4 {
                result += 1;
                grid[i][j] = 'x';
            }
        }
    }

    result
}

fn read_file_contents(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Failed to open file");
    let lines = io::BufReader::new(file).lines();
    let mut grid = Vec::new();
    for line in lines {
        let line = line.expect("Failed to read line");
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day_four_example() {
        assert_eq!(
            solve("/home/jtaubner/RustroverProjects/advent-of-code-2025/src/input/day04test.txt"),
            43
        );
    }

    #[test]
    fn test_solve_day_four() {
        assert_eq!(solve_day_four_part_two(), 1478);
    }
}

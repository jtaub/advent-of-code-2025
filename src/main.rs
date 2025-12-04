mod day01;
mod day01_2;
mod day02;
mod day02_2;
mod day03;
mod day03_2;

fn main() {
    println!("Day 1: {}", day01::solve_day_one());
    println!("Day 1 part 2: {}", day01_2::solve_day_one_part_two());
    println!("Day 2: {}", day02::solve_day_two());
    println!("Day 2 part 2: {}", day02_2::solve_day_two_part_two());
    println!("Day 3: {}", day03::solve_day_three());
    println!("Day 3 part 2: {}", day03_2::solve_day_three_part_two());
}

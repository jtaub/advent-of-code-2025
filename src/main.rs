mod day01;
mod day01_2;
mod day02;
mod day02_2;
mod day03;
mod day03_2;
mod day04;
mod day04_2;
mod day05;
mod day05_2;
mod day06;
mod day06_2;
mod day07;

fn main() {
    println!("Day 1: {}", day01::solve_day_one());
    println!("Day 1 part 2: {}", day01_2::solve_day_one_part_two());
    println!("Day 2: {}", day02::solve_day_two());
    println!("Day 2 part 2: {}", day02_2::solve_day_two_part_two());
    println!("Day 3: {}", day03::solve_day_three());
    println!("Day 3 part 2: {}", day03_2::solve_day_three_part_two());
    println!("Day 4: {}", day04::solve_day_four());
    println!("Day 4 part 2: {}", day04_2::solve_day_four_part_two());
    println!("Day 5: {}", day05::solve_day_five());
    println!("Day 5 part 2: {}", day05_2::solve_day_five_part_two());
    println!("Day 6: {}", day06::solve_day_six());
    println!("Day 6 part 2: {}", day06_2::solve_day_six_part_two());
    println!("Day 7: {}", day07::solve_day_seven());
}

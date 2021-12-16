#![feature(test)]
mod util;
mod day01;
mod day02;
mod day03;
mod day04;
fn main() {
    let cwd = std::env::current_dir().unwrap();

    let path = format!("{}{}", cwd.display(), "/src/day01/input.txt");
    println!("Day 01 (A): {}", day01::solve_part1(&path).unwrap());
    println!("Day 01 (B): {}", day01::solve_part2(&path).unwrap());

    let path = format!("{}{}", cwd.display(), "/src/day02/input.txt");
    println!("Day 02 (A): {}", day02::solve_part1(&path).unwrap());
    println!("Day 02 (B): {}", day02::solve_part2(&path).unwrap());

    let path = format!("{}{}", cwd.display(), "/src/day03/input.txt");
    println!("Day 03 (A): {}", day03::solve_part1(&path).unwrap());
    println!("Day 03 (B): {}", day03::solve_part2(&path).unwrap());

    let path = format!("{}{}", cwd.display(), "/src/day04/test.txt");
    println!("Day 03 (A): {}", day04::solve_part1(&path).unwrap());
    //println!("Day 03 (B): {}", day04::solve_part2(&path).unwrap());

}

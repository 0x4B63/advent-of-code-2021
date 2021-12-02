mod day01;
mod day02;

fn main() {
    let cwd = std::env::current_dir().unwrap();

    let path = format!("{}{}", cwd.display(), "/src/day01/input.txt");
    println!("Day 01 (A): {}", day01::solve_part1(&path).unwrap());
    println!("Day 01 (B): {}", day01::solve_part2(&path).unwrap());

    let path = format!("{}{}", cwd.display(), "/src/day02/input.txt");
    println!("Day 02 (A): {}", day02::solve_part1(&path).unwrap());
    println!("Day 02 (B): {}", day02::solve_part2(&path).unwrap());
}

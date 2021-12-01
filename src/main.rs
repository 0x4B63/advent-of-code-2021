mod day01;

fn main() {
    let cwd = std::env::current_dir().unwrap();

    let path = format!("{}{}", cwd.display(), "/src/day01/input.txt");
    println!("{}", day01::solve_part1(&path).unwrap());

    let path = format!("{}{}", cwd.display(), "/src/day01/input.txt");
    println!("{}", day01::solve_part2(&path).unwrap());
}

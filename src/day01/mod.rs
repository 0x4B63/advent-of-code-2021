use crate::util::load;
extern crate test;

#[derive(Debug)]
pub enum AdventError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

pub fn solve_part1(inputfile: &str) -> Result<usize, AdventError> {    
    let input = load(inputfile).map_err(AdventError::Io)?;
    let mut prev: usize = input[0].trim().parse::<usize>().map_err(AdventError::Parse)?;
    let mut output: usize = 0;

    for line in input {
        let curr = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
        if prev < curr {
            output += 1;
        }
        prev = curr;
    }
    Ok(output)
}

pub fn solve_part2(inputfile: &str) -> Result<usize, AdventError> {
    let input = load(inputfile).map_err(AdventError::Io)?;
    let mut a = input[0].trim().parse::<usize>().map_err(AdventError::Parse)?;
    let mut b = input[1].trim().parse::<usize>().map_err(AdventError::Parse)?;
    let mut c = input[2].trim().parse::<usize>().map_err(AdventError::Parse)?;
    let mut output: usize = 0;
    let mut prev = a + b + c;

    for line in input {
        a = b;
        b = c;
        c = line.trim().parse::<usize>().map_err(AdventError::Parse)?;

        if prev < a + b + c {
            output += 1;
        }
        prev = a + b + c;
    }
    Ok(output)
}

#[bench]
fn bench_part1(bench: &mut test::Bencher) {
    bench.iter(|| {
        let cwd = std::env::current_dir().unwrap();
        let path = format!("{}{}", cwd.display(), "/src/day01/input.txt");
        let _ = solve_part1(&path);
    });
}

#[bench]
fn bench_part2(bench: &mut test::Bencher) {
    bench.iter(|| {
        let cwd = std::env::current_dir().unwrap();
        let path = format!("{}{}", cwd.display(), "/src/day01/input.txt");
        let _ = solve_part2(&path);
    });
}

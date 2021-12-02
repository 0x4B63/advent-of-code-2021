use std::{io, num, fs::File, io::{BufReader, BufRead}, fmt::Debug};
extern crate test;


#[derive(Debug)]
pub enum AdventError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

pub fn solve_part1(inputfile: &str) -> Result<usize, AdventError> {
    let file = File::open(inputfile).map_err(AdventError::Io)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).map_err(AdventError::Io)?;
    let mut prev: usize = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
    line.clear();
    let mut output: usize = 0;
    
    loop {
        match reader.read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                let curr = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
                if prev < curr {
                    output += 1;
                }
                prev = curr;
                line.clear();
            },
            Err(err) => {
                return Err(AdventError::Io(err));
            }
        }
    }
    Ok(output)
}


pub fn solve_part2(inputfile: &str) -> Result<usize, AdventError> {
    let file = File::open(&inputfile).map_err(AdventError::Io)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut output: usize = 0;

    reader.read_line(&mut line).map_err(AdventError::Io)?;
    let mut a = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
    line.clear();

    reader.read_line(&mut line).map_err(AdventError::Io)?;
    let mut b = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
    line.clear();

    reader.read_line(&mut line).map_err(AdventError::Io)?;
    let mut c = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
    line.clear();

    let mut prev = a + b + c;

    loop {
        match reader.read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                
                a = b;
                b = c;
                c = line.trim().parse::<usize>().map_err(AdventError::Parse)?;

                if prev < a + b + c {
                    output += 1;
                }

                prev = a + b + c;
                line.clear();
            },
            Err(err) => {
                return Err(AdventError::Io(err));
            }
        };
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
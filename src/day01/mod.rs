#![allow(dead_code)]
use std::{io, num};
use std::{fs::File, io::{BufReader, BufRead}, fmt::Debug};

#[derive(Debug)]
pub enum AdventError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

pub fn solve_part1(inputfile: &str) -> Result<usize, AdventError> {
    let file = File::open(inputfile).map_err(AdventError::Io)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut prev: usize = 0;
    let mut output: usize = 0;

    loop {
        match reader.read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                let parsed = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
                if prev < parsed {
                    output += 1;
                }
                prev = parsed;
                line.clear();
            },
            Err(err) => {
                return Err(AdventError::Io(err));
            }
        }
    }
    Ok(output - 1)
}

#[allow(unused_assignments)]
pub fn solve_part2(inputfile: &str) -> Result<usize, AdventError> {
    let file = File::open(&inputfile).map_err(AdventError::Io)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut a: usize = 0;
    let mut b: usize = 0;
    let mut c: usize = 0;
    let mut prev: usize = 0;
    let mut curr: usize = 0;
    let mut output: usize = 0;

    reader.read_line(&mut line).map_err(AdventError::Io)?;
    a = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
    line.clear();

    reader.read_line(&mut line).map_err(AdventError::Io)?;
    b = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
    line.clear();

    reader.read_line(&mut line).map_err(AdventError::Io)?;
    c = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
    line.clear();

    prev = a + b + c;

    loop {
        match reader.read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                
                a = b;
                b = c;
                c = line.trim().parse::<usize>().map_err(AdventError::Parse)?;
                curr = a + b + c;

                if prev < curr {
                    output += 1;
                }

                prev = curr;
                line.clear();
            },
            Err(err) => {
                return Err(AdventError::Io(err));
            }
        };
    }
    Ok(output)
}
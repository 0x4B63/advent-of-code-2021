use std::{io::{self, BufReader, BufRead}, num, fs::File};

#[derive(Debug)]
pub enum AdventError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

pub fn solve_part1(inputfile: &str) -> Result<usize, AdventError> {
    let file = File::open(inputfile).map_err(AdventError::Io)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let (mut x,mut y) = (0,0);

    loop {
        match reader.read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                let cmds: Vec<&str> = line.split(' ').collect();
                let cmd = cmds[0];
                let dist = cmds[1].trim().parse::<usize>().map_err(AdventError::Parse)?;

                match (cmd, dist) {
                    ("forward", dist) => {
                        x += dist;
                    },
                    ("up", dist) => {
                        y -= dist;
                    },
                    ("down", dist) => {
                        y += dist;
                    },
                    (_,_) => {}
                }
                line.clear();
            },
            Err(err) => {
                return Err(AdventError::Io(err));
            }
        }
    }
    Ok(x * y)
}

pub fn solve_part2(inputfile: &str) -> Result<usize, AdventError> {
    let file = File::open(inputfile).map_err(AdventError::Io)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let (mut x,mut y, mut z) = (0,0,0);

    loop {
        match reader.read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                let cmds: Vec<&str> = line.split(' ').collect();
                let cmd = cmds[0];
                let dist = cmds[1].trim().parse::<usize>().map_err(AdventError::Parse)?;

                match (cmd, dist) {
                    ("forward", dist) => {
                        x += dist;
                        y += z * dist;
                    },
                    ("up", dist) => {
                        z -= dist;
                    },
                    ("down", dist) => {
                        z += dist;
                    },
                    (_,_) => {}
                }
                line.clear();
            },
            Err(err) => {
                return Err(AdventError::Io(err));
            }
        }
    }
    Ok(x * y)
}
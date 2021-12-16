use std::{fs::File, io::{self, BufReader, BufRead}};

#[derive(Debug)]
pub enum AdventError {
    Io(io::Error),
}

pub fn solve_part1(inputfile: &str) -> Result<usize, AdventError> {
    let input = load(inputfile).unwrap();
    let len = input[0].len() - 1;
    let half_count = input.len() / 2;
    let mut store: Vec<usize> = std::iter::repeat(0).take(len).collect();

    for line in input {
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => {
                    store[i] += 1;
                },
                _ => {}
            }
        }
    }

    let gamma: String = store.iter().fold(String::new(), |acc, x|
        if x > &(&half_count){acc + "1"} else {acc + "0"});
    let epsilon: String = store.iter().fold(String::new(), |acc, x|
        if x > &(&half_count){acc + "0"} else {acc + "1"});
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    Ok(gamma * epsilon)
}

// Note: This solution is pretty janky. Refactor later.
pub fn solve_part2(inputfile: &str) -> Result<usize, AdventError> {
    let mut input = load(inputfile).unwrap();
    let mut input2 = input.clone();
    let mut pattern = String::new();
    let mut pattern2 = String::new();
    let length = input[0].len() - 1;
    let length2 = input2[0].len() - 1;
    for i in 0..length {
        pattern += bitstuff(&input, i);
        input = reduce(input, &pattern);
        if input.len() == 1 {
            break;
        }
    }

    for i in 0..length2 {
        pattern2 += bitstuff_inverse(&input2, i);
        input2 = reduce(input2, &pattern2);
        if input2.len() == 1 {
            break;
        }
    }

    let out_a = usize::from_str_radix(&input[0].trim(), 2).unwrap();
    let out_b = usize::from_str_radix(&input2[0].trim(), 2).unwrap();
    Ok(out_a * out_b)
}

fn load(inputfile: &str) -> Result<Vec<String>, AdventError> {
    let file = File::open(inputfile).map_err(AdventError::Io)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut out: Vec<String> = Vec::with_capacity(1000);

    loop {
        match reader.read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                out.push(line.clone());
                line.clear();
            },
            Err(err) => {
                return Err(AdventError::Io(err));
            }
        }
    }
    Ok(out)
}

fn bitstuff(input: &Vec<String>, pos: usize) -> &str {
    let mut a = 0;
    let mut b = 0;
    for line in input {
        if line.chars().nth(pos).unwrap() == '1' {
            a += 1;
        } else {
            b += 1;
        }
    }
    if a >= b { return "1"; }
    "0"
}

fn bitstuff_inverse(input: &Vec<String>, pos: usize) -> &str {
    let mut a = 0;
    let mut b = 0;
    for line in input {
        if line.chars().nth(pos).unwrap() == '0' {
            a += 1;
        } else {
            b += 1;
        }
    }
    if a <= b { return "0"; }
    "1"
}

fn reduce(input: Vec<String>, pattern: &str) -> Vec<String> {
    input.into_iter().filter(|x| x.starts_with(&pattern)).collect()
}
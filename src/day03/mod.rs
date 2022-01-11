use crate::util::load;

#[derive(Debug)]
pub enum AdventError {
    Io(std::io::Error),
}

pub fn solve_part1(inputfile: &str) -> Result<usize, AdventError> {
    let input = load(inputfile).map_err(AdventError::Io)?;
    let len = input[0].len() - 1;
    let half_count = input.len() / 2;
    let mut store: Vec<usize> = std::iter::repeat(0).take(len).collect();

    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                store[i] += 1;
            }
        }
    }

    let gamma: String = store.iter().fold(String::new(), |acc, x| {
        if x > (&half_count) {
            acc + "1"
        } else {
            acc + "0"
        }
    });
    let epsilon: String = store.iter().fold(String::new(), |acc, x| {
        if x > (&half_count) {
            acc + "0"
        } else {
            acc + "1"
        }
    });
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    Ok(gamma * epsilon)
}

// Note: This solution is pretty janky. Refactor later.
pub fn solve_part2(inputfile: &str) -> Result<usize, AdventError> {
    let mut input = load(inputfile).map_err(AdventError::Io)?;
    let mut input2 = input.clone();
    let mut pattern = String::new();
    let mut pattern2 = String::new();
    let length = input[0].len() - 1;
    let length2 = length;
    for i in 0..length {
        pattern += bitstuff(&input, i, false);
        input = input.into_iter().filter(|x| x.starts_with(&pattern)).collect();
        if input.len() == 1 {
            break;
        }
    }

    for i in 0..length2 {
        pattern2 += bitstuff(&input2, i, true);
        input2 = input2.into_iter().filter(|x| x.starts_with(&pattern2)).collect();
        if input2.len() == 1 {
            break;
        }
    }

    let out_a = usize::from_str_radix(input[0].trim(), 2).unwrap();
    let out_b = usize::from_str_radix(input2[0].trim(), 2).unwrap();
    Ok(out_a * out_b)
}

fn bitstuff(input: &[String], pos: usize, invert: bool) -> &str {
    let mut a = 0;
    let mut b = 0;
    for line in input {
        if invert {
            if line.chars().nth(pos).unwrap() == '0' {
                a += 1;
            } else {
                b += 1;
            }
        } else if line.chars().nth(pos).unwrap() == '1' {
            a += 1;
        } else {
            b += 1;
        }
    }
    if invert {
        if a <= b {
            return "0";
        }
        "1"
    } else {
        if a >= b {
            return "1";
        }
        "0"
    }
}

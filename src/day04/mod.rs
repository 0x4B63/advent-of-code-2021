use std::{io};
use crate::util::load;

#[derive(Debug)]
pub enum AdventError {
    Io(io::Error),
}

pub fn solve_part1(inputfile: &str) -> Result<usize, AdventError> {
    let mut input = load(inputfile).map_err(AdventError::Io).unwrap();
    let _cmds = input[0].clone();
    input.remove(0);
    let mut _boards: Vec<Vec<u8>> = Vec::new();
    
    Ok(0)
}

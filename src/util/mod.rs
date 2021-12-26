use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load(inputfile: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(inputfile)?;
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
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(out)
}

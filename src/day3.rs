use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn day3pt1(filename: &str) -> i32 {
    let path = Path::new(filename);
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            // do work
        }
    }
    0 // result
}

pub fn day3pt2(filename: &str) -> i32 {
    let path = Path::new(filename);
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            // do work
        }
    }
    0 // result
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

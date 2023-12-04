use regex::{Match, Regex};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn day4pt1(filename: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let mut lines = read_lines(path)?.flatten();
    let mut results: Vec<u64> = Vec::new();
    // TODO
    Ok(results.iter().sum())
}

pub fn day4pt2(filename: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let mut lines = read_lines(path)?.flatten();
    let mut results: Vec<u64> = Vec::new();
    // TODO
    Ok(results.iter().sum())
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

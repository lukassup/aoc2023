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

pub fn day5pt1(filename: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let lines = read_lines(path)?.flatten();
    // TODO
    Ok(0)
}

pub fn day5pt2(filename: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let lines = read_lines(path)?.flatten();
    // TODO
    Ok(0)
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

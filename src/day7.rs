use std::error::Error;
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

pub fn day7pt1(filename: &str) -> Result<i64, Box<dyn Error>> {
    let path = Path::new(filename);
    let mut lines_iter = read_lines(path)?.flatten();
    // TODO
    Ok(0)
}

pub fn day7pt2(filename: &str) -> Result<i64, Box<dyn Error>> {
    let path = Path::new(filename);
    let mut lines_iter = read_lines(path)?.flatten();
    // TODO
    Ok(0)
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

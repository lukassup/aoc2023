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
    let lines = read_lines(path)?.flatten();
    let results: Vec<u64> = lines
        .filter_map(|line| {
            let (part1, part2) = line.split_once(':')?;
            let card_num = part1.split_once(' ')?.1.trim().parse::<u64>().ok()?;
            let (win_str, draw_str) = part2.split_once('|')?;
            let win_nums: Vec<u64> = win_str
                .split(' ')
                .filter_map(|n| n.trim().parse::<u64>().ok())
                .collect();
            let draw_nums: Vec<u64> = draw_str
                .split(' ')
                .filter_map(|n| n.trim().parse::<u64>().ok())
                .collect();
            let score = win_nums.iter().fold(0, |result, win_num| {
                if draw_nums.contains(win_num) {
                    if result == 0 {
                        1
                    } else {
                        result * 2
                    }
                } else {
                    result
                }
            });
            Some(score)
        })
        .collect();
    Ok(results.iter().sum())
}

pub fn day4pt2(filename: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let lines = read_lines(path)?.flatten();
    let results: Vec<u64> = Vec::new();
    // TODO
    Ok(results.iter().sum())
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

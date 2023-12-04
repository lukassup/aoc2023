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
    let results_iter = lines.filter_map(|line| {
        let part2 = line.split_once(':')?.1;
        let (win_str, draw_str) = part2.split_once('|')?;
        let win_nums_iter = win_str
            .split(' ')
            .filter_map(|n| n.trim().parse::<u64>().ok());
        let draw_nums: Vec<u64> = draw_str
            .split(' ')
            .filter_map(|n| n.trim().parse::<u64>().ok())
            .collect();
        let win_count = win_nums_iter
            .filter(|win_num| draw_nums.contains(&win_num))
            .count() as u32;
        if win_count > 0 {
            Some(2u64.pow(win_count - 1))
        } else {
            None
        }
    });
    Ok(results_iter.sum())
}

pub fn day4pt2(filename: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let lines = read_lines(path)?.flatten();
    let card_scores: Vec<u64> = lines
        .filter_map(|line| {
            let part2 = line.split_once(':')?.1;
            let (win_str, draw_str) = part2.split_once('|')?;
            let win_nums_iter = win_str
                .split(' ')
                .filter_map(|n| n.trim().parse::<u64>().ok());
            let draw_nums: Vec<u64> = draw_str
                .split(' ')
                .filter_map(|n| n.trim().parse::<u64>().ok())
                .collect();
            let win_count = win_nums_iter
                .filter(|win_num| draw_nums.contains(&win_num))
                .count() as u64;
            Some(win_count)
        })
        .collect();

    // build vector to store count of card copies, we start with one copy for each card
    let mut card_copies: Vec<u64> = vec![1; card_scores.len()];
    // count per-card copies
    for index in 0..card_scores.len() {
        let score = card_scores[index];
        let copies_current = card_copies[index];
        if score > 0 {
            for index_copy in (index + 1)..(index + 1 + score as usize) {
                card_copies[index_copy] += copies_current;
            }
        }
    }
    let result = card_copies.iter().sum();
    Ok(result)
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

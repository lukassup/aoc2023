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
    let mut card_copies: Vec<(&u64, u64)> = card_scores
        .iter()
        .map(|card_score| (card_score, 1))
        .collect();
    // count per-card copies
    for i in 0..card_copies.len() {
        let (score, copies_initial) = card_copies[i];
        if *score > 0 {
            for j in (i + 1)..(i + 1 + *score as usize) {
                card_copies[j].1 += copies_initial;
            }
        }
    }
    let result = card_copies.iter().fold(0, |sum, (_, copies)| sum + copies);
    Ok(result)
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

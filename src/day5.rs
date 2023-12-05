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

fn parse_seeds(line: &str) -> Vec<i64> {
    line.split_once(':')
        .expect("error splitting by ':'")
        .1
        .split_ascii_whitespace()
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect()
}

/// Each line within a map contains three numbers:
///
/// `{dst_range_start} {src_range_start} {range_length}`
fn parse_mapping(lines_iter: &mut impl Iterator<Item = String>) -> Vec<(i64, i64, i64)> {
    let mut mapping: Vec<(i64, i64, i64)> = Vec::new();
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }
        let mut value_iter = line
            .split_ascii_whitespace()
            .filter_map(|item| item.trim().parse::<i64>().ok());
        if let (Some(v1), Some(v2), Some(v3)) =
            (value_iter.next(), value_iter.next(), value_iter.next())
        {
            mapping.push((v1, v2, v3));
        }
    }
    mapping
}

fn remap(initial_nums: Vec<i64>, mapping: &Vec<(i64, i64, i64)>) -> Vec<i64> {
    initial_nums
        .iter()
        .map(|&num| {
            let mut remapped_num = num;
            for &(dst_range_start, src_range_start, range_len) in mapping {
                let src_range = src_range_start..(src_range_start + range_len);
                let src_dst_difference = src_range_start - dst_range_start;
                if src_range.contains(&num) {
                    remapped_num = num - src_dst_difference;
                    break;
                }
            }
            remapped_num
        })
        .collect()
}

pub fn day5pt1(filename: &str) -> Result<i64, Box<dyn Error>> {
    let path = Path::new(filename);
    let mut lines_iter = read_lines(path)?.flatten();
    let mut numbers: Vec<i64> = Vec::new();
    while let Some(line) = lines_iter.next() {
        match line {
            l if l.starts_with("seeds:") => {
                numbers = parse_seeds(&l);
            }
            l if l.starts_with("seed-to-soil map:") => {
                let mapping = parse_mapping(&mut lines_iter);
                numbers = remap(numbers, &mapping);
            }
            l if l.starts_with("soil-to-fertilizer map:") => {
                let mapping = parse_mapping(&mut lines_iter);
                numbers = remap(numbers, &mapping);
            }
            l if l.starts_with("fertilizer-to-water map:") => {
                let mapping = parse_mapping(&mut lines_iter);
                numbers = remap(numbers, &mapping);
            }
            l if l.starts_with("water-to-light map:") => {
                let mapping = parse_mapping(&mut lines_iter);
                numbers = remap(numbers, &mapping);
            }
            l if l.starts_with("light-to-temperature map:") => {
                let mapping = parse_mapping(&mut lines_iter);
                numbers = remap(numbers, &mapping);
            }
            l if l.starts_with("temperature-to-humidity map:") => {
                let mapping = parse_mapping(&mut lines_iter);
                numbers = remap(numbers, &mapping);
            }
            l if l.starts_with("humidity-to-location map:") => {
                let mapping = parse_mapping(&mut lines_iter);
                numbers = remap(numbers, &mapping);
            }
            _ => {}
        }
    }
    Ok(*numbers.iter().min().unwrap_or(&0))
}

pub fn day5pt2(filename: &str) -> Result<i64, Box<dyn Error>> {
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

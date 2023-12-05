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

fn parse_seeds(line: &str) -> Vec<u64> {
    line.split_once(':')
        .expect("error splitting by ':'")
        .1
        .split_ascii_whitespace()
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .collect()
}

fn parse_mapping(lines_iter: &mut impl Iterator<Item = String>) -> Vec<(u64, u64, u64)> {
    let mut mapping: Vec<(u64, u64, u64)> = Vec::new();
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }
        let mut value_iter = line
            .split_ascii_whitespace()
            .filter_map(|item| item.trim().parse::<u64>().ok());
        if let (Some(v1), Some(v2), Some(v3)) =
            (value_iter.next(), value_iter.next(), value_iter.next())
        {
            mapping.push((v1, v2, v3));
        }
    }
    mapping
}

pub fn day5pt1(filename: &str) -> Result<u64, Box<dyn Error>> {
    let path = Path::new(filename);
    let mut lines_iter = read_lines(path)?.flatten();
    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil_mapping: Vec<(u64, u64, u64)> = Vec::new();
    let mut soil_to_fertilizer_mapping: Vec<(u64, u64, u64)> = Vec::new();
    let mut fertilizer_to_water_mapping: Vec<(u64, u64, u64)> = Vec::new();
    let mut water_to_light_mapping: Vec<(u64, u64, u64)> = Vec::new();
    let mut light_to_temperature_mapping: Vec<(u64, u64, u64)> = Vec::new();
    let mut temperature_to_humidity_mapping: Vec<(u64, u64, u64)> = Vec::new();
    let mut humidity_to_location_mapping: Vec<(u64, u64, u64)> = Vec::new();
    while let Some(line) = lines_iter.next() {
        match line {
            l if l.starts_with("seeds:") => {
                seeds.extend(parse_seeds(&l));
            }
            l if l.starts_with("seed-to-soil map:") => {
                seed_to_soil_mapping.extend(parse_mapping(&mut lines_iter));
            }
            l if l.starts_with("soil-to-fertilizer map:") => {
                soil_to_fertilizer_mapping.extend(parse_mapping(&mut lines_iter));
            }
            l if l.starts_with("fertilizer-to-water map:") => {
                fertilizer_to_water_mapping.extend(parse_mapping(&mut lines_iter));
            }
            l if l.starts_with("water-to-light map:") => {
                water_to_light_mapping.extend(parse_mapping(&mut lines_iter));
            }
            l if l.starts_with("light-to-temperature map:") => {
                light_to_temperature_mapping.extend(parse_mapping(&mut lines_iter));
            }
            l if l.starts_with("temperature-to-humidity map:") => {
                temperature_to_humidity_mapping.extend(parse_mapping(&mut lines_iter));
            }
            l if l.starts_with("humidity-to-location map:") => {
                humidity_to_location_mapping.extend(parse_mapping(&mut lines_iter));
            }
            _ => {}
        }
    }
    // dbg!(&seeds);
    // dbg!(&seed_to_soil_mapping[0]);
    // dbg!(&soil_to_fertilizer_mapping[0]);
    // dbg!(&fertilizer_to_water_mapping[0]);
    // dbg!(&water_to_light_mapping[0]);
    // dbg!(&light_to_temperature_mapping[0]);
    // dbg!(&temperature_to_humidity_mapping[0]);
    // dbg!(&humidity_to_location_mapping[0]);
    Ok(0)
}

pub fn day5pt2(filename: &str) -> Result<u64, Box<dyn Error>> {
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

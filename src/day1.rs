use std::collections::HashMap;
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

trait Reversable {
    fn reverse(&self) -> String;
}
impl Reversable for String {
    fn reverse(&self) -> Self {
        self.chars().rev().collect()
    }
}
impl Reversable for &str {
    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }
}
trait NumWordString {
    fn collect_numbers(&self, numdict: &HashMap<&str, i32>) -> i32;
}

impl NumWordString for String {
    fn collect_numbers(&self, numdict: &HashMap<&str, i32>) -> i32 {
        let string = self;
        let mut first = 0;
        'outer: for i in 0..string.len() {
            let prefix = &string[i..];
            for (k, v) in numdict {
                if prefix.starts_with(k) {
                    first = *v;
                    break 'outer;
                }
            }
        }

        let string_rev: String = self.reverse();
        let mut last = 0;
        'outer: for i in 0..string_rev.len() {
            let prefix = &string_rev[i..];
            for (k, v) in numdict {
                let k_rev: &String = &k.reverse();
                if prefix.starts_with(k_rev) {
                    last = *v;
                    break 'outer;
                }
            }
        }
        10 * first + last
    }
}

pub fn day1pt1(filename: &str) -> i32 {
    let path = Path::new(filename);
    let values: HashMap<&str, i32> = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let mut results: Vec<i32> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            results.push(line.collect_numbers(&values));
        }
    }
    results.iter().sum()
}

pub fn day1pt2(filename: &str) -> i32 {
    let path = Path::new(filename);
    let values: HashMap<&str, i32> = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut results: Vec<i32> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            results.push(line.collect_numbers(&values));
        }
    }
    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reversable_string() {
        let original = String::from("Hello!");
        let expected = String::from("!olleH");
        assert_eq!(original.reverse(), expected);
        assert_ne!(original.reverse(), original);
    }

    #[test]
    fn test_reversable_str() {
        let original = "Hello!";
        let expected = String::from("!olleH");
        assert_eq!(original.reverse(), expected);
        assert_ne!(original.reverse(), String::from(original));
    }

    #[test]
    fn test_collect_numbers_day1() {
        let test_string = String::from("onefxtprsml8fqptvmfthreesix2jbeightwor");
        let map: HashMap<&str, i32> = HashMap::from([
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);
        assert_eq!(test_string.collect_numbers(&map), 82);
        assert_ne!(test_string.collect_numbers(&map), 88);
        assert_ne!(test_string.collect_numbers(&map), 22);
    }

    #[test]
    fn test_collect_numbers_day2() {
        let test_string = String::from("onefxtprsml8fqptvmfthreesix2jbeightwor");
        let map: HashMap<&str, i32> = HashMap::from([
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);
        assert_eq!(test_string.collect_numbers(&map), 12);
        assert_ne!(test_string.collect_numbers(&map), 18);
        assert_ne!(test_string.collect_numbers(&map), 82);
    }
}

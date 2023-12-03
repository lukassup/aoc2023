mod day1;
use crate::day1::day1pt1;
use crate::day1::day1pt2;
mod day2;
use crate::day2::day2pt1;
use crate::day2::day2pt2;
mod day3;
use crate::day3::day3pt1;
use crate::day3::day3pt2;

fn main() {
    println!("ADVENT OF CODE 2023");
    let result = day1pt1("input/day1.txt");
    println!("DAY 1 - part 1: {result}");
    let result = day1pt2("input/day1.txt");
    println!("DAY 1 - part 2: {result}");
    let result = day2pt1("input/day2.txt");
    println!("DAY 2 - part 1: {result}");
    let result = day2pt2("input/day2.txt");
    println!("DAY 2 - part 2: {result}");
    let result = day3pt1("input/day3.txt");
    println!("DAY 3 - part 1: {result}");
    let result = day3pt2("input/day3.txt");
    println!("DAY 3 - part 2: {result}");
}

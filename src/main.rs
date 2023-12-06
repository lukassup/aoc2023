mod day1;
use crate::day1::day1pt1;
use crate::day1::day1pt2;
mod day2;
use crate::day2::day2pt1;
use crate::day2::day2pt2;
mod day3;
use crate::day3::day3pt1;
use crate::day3::day3pt2;
mod day4;
use crate::day4::day4pt1;
use crate::day4::day4pt2;
mod day5;
use crate::day5::day5pt1;
use crate::day5::day5pt2;
mod day6;
use crate::day6::day6pt1;
use crate::day6::day6pt2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ADVENT OF CODE 2023");
    let result = day1pt1("input/day1.txt");
    println!("DAY 1 - part 1: {result}");
    let result = day1pt2("input/day1.txt");
    println!("DAY 1 - part 2: {result}");
    let result = day2pt1("input/day2.txt");
    println!("DAY 2 - part 1: {result}");
    let result = day2pt2("input/day2.txt");
    println!("DAY 2 - part 2: {result}");
    let result = day3pt1("input/day3.txt")?;
    println!("DAY 3 - part 1: {result}");
    let result = day3pt2("input/day3.txt")?;
    println!("DAY 3 - part 2: {result}");
    let result = day4pt1("input/day4.txt")?;
    println!("DAY 4 - part 1: {result}");
    let result = day4pt2("input/day4.txt")?;
    println!("DAY 4 - part 2: {result}");
    let result = day5pt1("input/day5.txt")?;
    println!("DAY 5 - part 1: {result}");
    let result = day5pt2("input/day5.txt")?;
    println!("DAY 5 - part 2: {result}");
    let result = day6pt1("input/day6.txt")?;
    println!("DAY 6 - part 1: {result}");
    let result = day6pt2("input/day6.txt")?;
    println!("DAY 6 - part 2: {result}");
    Ok(())
}

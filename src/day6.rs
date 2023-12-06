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

/// Return two solutions for quadratic equation: `ax^2 + bx + c = 0`
fn solve_quadratic_eq(a: f64, b: f64, c: f64) -> (f64, f64) {
    // D = b^2 - 4ac
    // s1 = (-b + sqrt(D)) / 2a
    // s2 = (-b - sqrt(D)) / 2a
    let d: f64 = b.powi(2) - 4.0 * a * c;
    let s1: f64 = (-b + d.sqrt()) / (2.0 * a);
    let s2: f64 = (-b - d.sqrt()) / (2.0 * a);
    (s1, s2)
}

pub fn day6pt1(filename: &str) -> Result<i64, Box<dyn Error>> {
    let path = Path::new(filename);
    let mut lines_iter = read_lines(path)?.flatten();
    let mut res: Vec<i64> = Vec::new();
    if let (Some(line_time), Some(line_distance)) = (lines_iter.next(), lines_iter.next()) {
        let times = line_time
            .split_once(':')
            .ok_or("error splitting by ':'")?
            .1
            .split_ascii_whitespace()
            .filter_map(|t_str| t_str.parse::<i64>().ok());
        let distances = line_distance
            .split_once(':')
            .ok_or("error splitting by ':'")?
            .1
            .split_ascii_whitespace()
            .filter_map(|d_str| d_str.parse::<i64>().ok());
        let times_distances = times.zip(distances).collect::<Vec<_>>();
        // s (mm) -> distance travelled
        // t (ms) -> current time duration
        // x (ms) -> time the button was pressed
        // v_0 = 0 (ms), initial velocity
        // v = x (mm/ms), velocity for each ms button was pressed
        // d < v_0*t + x*(t - x) = tx - x^2
        for &(t, d) in &times_distances {
            // -x^2 + tx - d > 0
            // a=-1, b=t, c=-d
            let a = -1.0;
            let b = t as f64;
            let c = -d as f64;
            let (s1, s2) = solve_quadratic_eq(a, b, c);
            let s1 = s1.ceil() as i64;
            let s2 = s2.ceil() as i64;
            // count all integers satisfying equation range: solution1..solution2
            res.push((s1..s2).count() as i64);
        }
    }
    Ok(res.iter().product())
}

pub fn day6pt2(filename: &str) -> Result<i64, Box<dyn Error>> {
    let path = Path::new(filename);
    let mut lines_iter = read_lines(path)?.flatten();
    let mut res: Vec<i64> = Vec::new();
    if let (Some(line_time), Some(line_distance)) = (lines_iter.next(), lines_iter.next()) {
        let t = line_time
            .split_once(':')
            .ok_or("error splitting by ':'")?
            .1
            .split_ascii_whitespace()
            .collect::<String>()
            .parse::<i64>()?;
        let d = line_distance
            .split_once(':')
            .ok_or("error splitting by ':'")?
            .1
            .split_ascii_whitespace()
            .collect::<String>()
            .parse::<i64>()?;
        // same algo as in part 1
        let a = -1.0;
        let b = t as f64;
        let c = -d as f64;
        let (s1, s2) = solve_quadratic_eq(a, b, c);
        let s1 = s1.ceil() as i64;
        let s2 = s2.ceil() as i64;
        // count all integers satisfying equation range: solution1..solution2
        res.push((s1..s2).count() as i64);
    }
    Ok(res.iter().product())
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

use regex::{Match, Regex};
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

fn collect_symbol_positions(line: &str) -> Vec<(usize, usize, usize)> {
    let pattern: Regex = Regex::new(r"[^\d.]").unwrap();
    pattern
        .captures_iter(&line)
        .filter_map(|c| c.get(0))
        .map(|m| (m.start() - 1, m.start(), m.start() + 1))
        .collect()
}

fn collect_gear_positions(line: &str) -> Vec<(usize, usize, usize)> {
    let pattern: Regex = Regex::new(r"\*").unwrap();
    pattern
        .captures_iter(&line)
        .filter_map(|c| c.get(0))
        .map(|m| (m.start() - 1, m.start(), m.start() + 1))
        .collect()
}

pub fn day3pt1(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let num_pattern: Regex = Regex::new(r"\b\d+\b")?;

    let path = Path::new(filename);
    let mut lines = read_lines(path)?.flatten(); // skip line read errors

    let mut line_prev: Option<String> = None;
    let mut results: Vec<i32> = Vec::new();
    loop {
        match (line_prev.clone(), lines.next(), lines.next()) {
            // first iteration: (_, lines[0], lines[1])
            // last iteration: (lines[len(lines)-1], lines[len(lines)], _)
            // (only when total number of lines is not even)
            (None, Some(line_top), Some(line_mid)) | //#
            (Some(line_top), Some(line_mid), None) => {
                let positions_top = collect_symbol_positions(&line_top);
                let positions_mid = collect_symbol_positions(&line_mid);

                // collect numbers adjacent to symbols
                let nums_top: Vec<i32> = num_pattern
                    .captures_iter(&line_top)
                    .filter_map(|c| c.get(0))
                    .filter_map(|num| {
                        if positions_top.iter().any(|(p1, _, p3)| {
                            num.range().contains(p1) // _
                                || num.range().contains(p3)
                        }) || positions_mid.iter().any(|(p1, p2, p3)| {
                            num.range().contains(p1)
                                || num.range().contains(p2)
                                || num.range().contains(p3)
                        }) {
                            num.as_str().parse::<i32>().ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                results.extend(nums_top);

                let nums_mid: Vec<i32> = num_pattern
                    .captures_iter(&line_mid)
                    .filter_map(|c| c.get(0))
                    .filter_map(|num| {
                        if positions_top.iter().any(|(p1, p2, p3)| {
                            num.range().contains(p1)
                                || num.range().contains(p2)
                                || num.range().contains(p3)
                        }) || positions_mid.iter().any(|(p1, _, p3)| {
                            num.range().contains(p1) // _
                                || num.range().contains(p3)
                        }) {
                            num.as_str().parse::<i32>().ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                results.extend(nums_mid);

                // save previous last line for next iteration
                line_prev = Some(line_mid);
            }
            // second iteration: (lines[1], lines[2], lines[3])
            // other iterations: (lines[n-1], lines[n], lines[n+1])
            (Some(line_top), Some(line_mid), Some(line_bot)) => {
                let positions_top = collect_symbol_positions(&line_top);
                let positions_mid = collect_symbol_positions(&line_mid);
                let positions_bot = collect_symbol_positions(&line_bot);

                // collect numbers adjacent to symbols
                let nums_top: Vec<i32> = num_pattern
                    .captures_iter(&line_top)
                    .filter_map(|c| c.get(0))
                    .filter_map(|num| {
                        // we only need to check with the line after it
                        // numbers on the bottom line should already be colleted in previous iteration
                        if positions_mid.iter().any(|(p1, p2, p3)| {
                            num.range().contains(p1)
                                || num.range().contains(p2)
                                || num.range().contains(p3)
                        }) {
                            num.as_str().parse::<i32>().ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                results.extend(nums_top);

                let nums_mid: Vec<i32> = num_pattern
                    .captures_iter(&line_mid)
                    .filter_map(|c| c.get(0))
                    .filter_map(|num| {
                        if positions_top.iter().any(|(p1, p2, p3)| {
                            num.range().contains(p1)
                                || num.range().contains(p2)
                                || num.range().contains(p3)
                        }) || positions_mid.iter().any(|(p1, _, p3)| {
                            num.range().contains(p1) // _
                                || num.range().contains(p3)
                        }) || positions_bot.iter().any(|(p1, p2, p3)| {
                            num.range().contains(p1)
                                || num.range().contains(p2)
                                || num.range().contains(p3)
                        }) {
                            num.as_str().parse::<i32>().ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                results.extend(nums_mid);

                let nums_bot: Vec<i32> = num_pattern
                    .captures_iter(&line_bot)
                    .filter_map(|c| c.get(0))
                    .filter_map(|num| {
                        if positions_mid.iter().any(|(p1, p2, p3)| {
                            num.range().contains(p1)
                                || num.range().contains(p2)
                                || num.range().contains(p3)
                        }) || positions_bot.iter().any(|(p1, _, p3)| {
                            num.range().contains(p1) // _
                                || num.range().contains(p3)
                        }) {
                            num.as_str().parse::<i32>().ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                results.extend(nums_bot);

                // save previous last line for next iteration
                line_prev = Some(line_bot);
            }
            _ => break,
        }
    }
    Ok(results.iter().sum())
}

pub fn day3pt2(filename: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let num_pattern: Regex = Regex::new(r"\b\d+\b")?;

    let path = Path::new(filename);
    let mut lines = read_lines(path)?.flatten(); // skip line read errors

    let mut results: Vec<i64> = Vec::new();
    let mut line_prev2: Option<String> = None;
    let mut line_prev1: Option<String> = None;
    loop {
        match (line_prev2.clone(), line_prev1.clone(), lines.next()) {
            (None, None, Some(line_bot)) => {
                line_prev1 = Some(line_bot);
            }
            (None, Some(line_mid), Some(line_bot)) => {
                let positions_mid = collect_gear_positions(&line_mid);
                let nums_mid: Vec<Match> = num_pattern
                    .captures_iter(&line_mid)
                    .filter_map(|c| c.get(0))
                    .collect();
                let nums_bot: Vec<Match> = num_pattern
                    .captures_iter(&line_bot)
                    .filter_map(|c| c.get(0))
                    .collect();

                let gearnums_mid: Vec<i64> = positions_mid
                    .iter()
                    .filter_map(|(p1, p2, p3)| {
                        let gnum_mid_iter = nums_mid
                            .iter()
                            .filter(|m| {
                                m.range().contains(p1) //#
                            || m.range().contains(p3)
                            })
                            .filter_map(|m| m.as_str().parse::<i64>().ok());

                        let gnum_bot_iter = nums_bot
                            .iter()
                            .filter(|m| {
                                m.range().contains(p1) //#
                            || m.range().contains(p2)
                            || m.range().contains(p3)
                            })
                            .filter_map(|m| m.as_str().parse::<i64>().ok());

                        let gearnums: Vec<i64> = gnum_mid_iter.chain(gnum_bot_iter).collect();
                        if gearnums.len() == 2 {
                            dbg!(&gearnums);
                            Some(gearnums.iter().product())
                        } else {
                            None
                        }
                    })
                    .collect();
                results.extend(gearnums_mid);
                line_prev2 = Some(line_mid);
                line_prev1 = Some(line_bot);
            }
            (Some(line_top), Some(line_mid), Some(line_bot)) => {
                let positions_mid = collect_gear_positions(&line_mid);

                // collect numbers
                let nums_top: Vec<Match> = num_pattern
                    .captures_iter(&line_top)
                    .filter_map(|c| c.get(0))
                    .collect();
                let nums_mid: Vec<Match> = num_pattern
                    .captures_iter(&line_mid)
                    .filter_map(|c| c.get(0))
                    .collect();
                let nums_bot: Vec<Match> = num_pattern
                    .captures_iter(&line_bot)
                    .filter_map(|c| c.get(0))
                    .collect();

                let gearnums_mid: Vec<i64> = positions_mid
                    .iter()
                    .filter_map(|(p1, p2, p3)| {
                        // check if there are two numbers for current "gear" position in top & mid lines
                        // return multiplied result
                        let gnum_top_iter = nums_top
                            .iter()
                            .filter(|m| {
                                m.range().contains(p1) //#
                                || m.range().contains(p2)
                                || m.range().contains(p3)
                            })
                            .filter_map(|m| m.as_str().parse::<i64>().ok());

                        let gnum_mid_iter = nums_mid
                            .iter()
                            .filter(|m| -> bool {
                                m.range().contains(p1) //#
                                || m.range().contains(p3)
                            })
                            .filter_map(|m| m.as_str().parse::<i64>().ok());

                        let gnum_bot_iter = nums_bot
                            .iter()
                            .filter(|m| {
                                m.range().contains(p1) //#
                            || m.range().contains(p2)
                            || m.range().contains(p3)
                            })
                            .filter_map(|m| m.as_str().parse::<i64>().ok());

                        let gearnums: Vec<i64> = gnum_top_iter
                            .chain(gnum_mid_iter)
                            .chain(gnum_bot_iter)
                            .collect();
                        if gearnums.len() == 2 {
                            Some(gearnums.iter().product())
                        } else {
                            None
                        }
                    })
                    .collect();
                results.extend(gearnums_mid);

                line_prev2 = Some(line_mid);
                line_prev1 = Some(line_bot);
            }
            _ => break,
        }
    }
    Ok(results.iter().sum())
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

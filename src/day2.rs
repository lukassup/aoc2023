use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}
impl Draw {
    fn new() -> Self {
        Draw {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn red(&self) -> i32 {
        self.red
    }
    fn green(&self) -> i32 {
        self.green
    }
    fn blue(&self) -> i32 {
        self.blue
    }
    fn match_lt_cond(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

#[derive(Debug, PartialEq, Eq)]

struct GameParseError;

#[derive(Debug)]
struct Game {
    id: i32,
    draws: Vec<Draw>,
}

impl Game {
    fn new(id: i32) -> Self {
        Self {
            id,
            draws: Vec::new(),
        }
    }

    fn match_lt_cond(&self) -> bool {
        self.draws.iter().all(Draw::match_lt_cond)
    }
    fn max_red(&self) -> i32 {
        self.draws.iter().map(Draw::red).max().unwrap_or(0)
    }
    fn max_green(&self) -> i32 {
        self.draws.iter().map(Draw::green).max().unwrap_or(0)
    }
    fn max_blue(&self) -> i32 {
        self.draws.iter().map(Draw::blue).max().unwrap_or(0)
    }
    fn power(&self) -> i32 {
        self.max_red() * self.max_green() * self.max_blue()
    }
}

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some((game_str, score_str)) = string.split_once(':') {
            if let Some((_, game_id)) = game_str.split_once(' ') {
                let game_id: i32 = game_id.parse().unwrap();
                let mut game = Game::new(game_id);
                score_str
                    .split(';')
                    .map(|draw| {
                        draw.trim()
                            .split(',')
                            .flat_map(|s| s.trim().split_once(' '))
                            .collect()
                    })
                    .for_each(|draw_data: Vec<(&str, &str)>| {
                        let mut draw = Draw::new();
                        draw_data.iter().for_each(|(count, color)| match color {
                            &"red" => draw.red += count.parse::<i32>().unwrap_or(0),
                            &"green" => draw.green += count.parse::<i32>().unwrap_or(0),
                            &"blue" => draw.blue += count.parse::<i32>().unwrap_or(0),
                            _ => {}
                        });
                        game.draws.push(draw);
                    });
                return Ok(game);
            } else {
                Err(GameParseError)
            }
        } else {
            Err(GameParseError)
        }
    }
}

pub fn day2pt1(filename: &str) -> i32 {
    let path = Path::new(filename);
    let mut games: Vec<Game> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            if let Ok(game) = Game::from_str(&line) {
                games.push(game);
            }
        }
    }
    games
        .iter()
        .filter(|g| g.match_lt_cond())
        .fold(0, |sum, game| sum + game.id)
}

pub fn day2pt2(filename: &str) -> i32 {
    let path = Path::new(filename);
    let mut games: Vec<Game> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            if let Ok(game) = Game::from_str(&line) {
                games.push(game);
            }
        }
    }
    games.iter().fold(0, |sum, game| sum + game.power())
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
}

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
    fn new(id: i32, draws: Vec<Draw>) -> Self {
        Self { id, draws }
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
        let (game_str, draws_str) = string.split_once(':').ok_or(GameParseError)?;
        let game_id: i32 = game_str
            .split_once(' ')
            .ok_or(GameParseError)?
            .1
            .parse()
            .or_else(|_| Err(GameParseError))?;
        let draws: Vec<Draw> = draws_str
            .split(';')
            .map(|draw_str| {
                let mut draw = Draw::new();
                draw_str.split(',').try_for_each(|draw_str| {
                    let (score_str, color_str) = draw_str.trim().split_once(' ')?;
                    let score: i32 = score_str.trim().parse().ok()?;
                    match color_str {
                        "red" => draw.red = score,
                        "green" => draw.green = score,
                        "blue" => draw.blue = score,
                        _ => {}
                    }
                    Some(())
                });
                draw
            })
            .collect();
        Ok(Game::new(game_id, draws))
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

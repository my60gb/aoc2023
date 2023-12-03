use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(s: &str) {
    let max_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum: u32 = s
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| game.is_possible(&max_draw))
        .map(|game| u32::from(game.id))
        .sum();
    println!("Total Pt1 is: {sum}");
}

fn part2(s: &str) {
    let sum: u32 = s
        .lines()
        .map(|l| l.parse::<Game>().unwrap().fewest_cubes_required().power())
        .sum();

    println!("Total Pt2 is: {sum}");
}

#[derive(Debug)]
/// A set of draws with an id
struct Game {
    id: u8,
    draws: Vec<Draw>,
}

impl Game {
    fn is_possible(&self, max_draw: &Draw) -> bool {
        self.draws.iter().all(|d| !d.over_limit(max_draw))
    }

    fn fewest_cubes_required(&self) -> Draw {
        let red = self.draws.iter().map(|d| d.red).max().unwrap();
        let green = self.draws.iter().map(|d| d.green).max().unwrap();
        let blue = self.draws.iter().map(|d| d.blue).max().unwrap();

        Draw { red, green, blue }
    }
}

#[derive(Debug)]
/// One draw of cubes out of the bag
struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

impl Draw {
    fn over_limit(&self, max_draw: &Draw) -> bool {
        self.red > max_draw.red || self.green > max_draw.green || self.blue > max_draw.blue
    }

    fn power(&self) -> u32 {
        u32::from(self.red) * u32::from(self.green) * u32::from(self.blue)
    }
}

fn color_count(list: &[(u8, String)], color: &str) -> u8 {
    let found: Vec<_> = list.iter().filter(|e| e.1.eq(color)).map(|e| e.0).collect();
    *found.first().unwrap_or(&0)
}

impl FromStr for Draw {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let end_pos: usize = s.find(';').unwrap_or(s.len());
        let draw = &s[..end_pos];
        let parts: Vec<(u8, String)> = draw
            .split_whitespace()
            .map(|e| e.replace(',', ""))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|c| (c[0].parse().unwrap(), c[1].clone()))
            .collect();

        let red = color_count(&parts, "red");
        let green = color_count(&parts, "green");
        let blue = color_count(&parts, "blue");

        Ok(Draw { red, green, blue })
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.find(':').unwrap();
        let id: u8 = s[5..colon].parse().unwrap();

        let semicolons: Vec<_> = s
            .char_indices()
            .filter(|c| c.1 == ';')
            .map(|c| c.0)
            .collect();
        let mut draws = vec![];

        let first_draw: Draw = s[colon + 1..].parse().unwrap();
        draws.push(first_draw);
        for idx in &semicolons {
            let draw: Draw = s[idx + 1..].parse().unwrap();
            draws.push(draw);
        }

        Ok(Game { id, draws })
    }
}

use std::{cmp, fs};

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("input").unwrap().trim().to_owned();

    part1(&input);
    part2(&input);
}

fn part1(s: &str) {
    let total: u32 = s.lines().map(parse_line).sum();

    println!("Total Pt1 is: {total}");
}

fn part2(s: &str) {
    let total: u32 = s.lines().map(parse_line2).sum();

    println!("Total Pt2 is: {total}");
}

fn parse_line(s: &str) -> u32 {
    let digits: Vec<_> = s.trim().chars().filter_map(|c| c.to_digit(10)).collect();

    match digits.len().cmp(&1) {
        cmp::Ordering::Greater => format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse()
            .unwrap(),
        cmp::Ordering::Equal => format!("{}{}", digits[0], digits[0]).parse().unwrap(),
        cmp::Ordering::Less => panic!("problem"),
    }
}

fn parse_line2(s: &str) -> u32 {
    let nums: Vec<_> = find_nums(s).iter().map(|n| parse_num(n)).collect();

    match nums.len().cmp(&1) {
        cmp::Ordering::Greater => format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
            .parse()
            .unwrap(),
        cmp::Ordering::Equal => format!("{}{}", nums[0], nums[0]).parse().unwrap(),
        cmp::Ordering::Less => panic!("problem"),
    }
}

fn find_nums(s: &str) -> Vec<String> {
    let mut found = vec![];

    for (idx, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            found.push((idx, c.to_string()));
        }
    }

    for num in NUMS {
        // Find the first match
        if let Some(idx) = s.find(num) {
            found.push((idx, num.to_owned()));
        }
        // Find the last match
        if let Some(idx) = s.rfind(num) {
            found.push((idx, num.to_owned()));
        }
    }

    found.sort_unstable_by_key(|k| k.0);
    found.into_iter().map(|e| e.1).collect()
}

fn parse_num(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        digit => digit.parse().unwrap(),
    }
}

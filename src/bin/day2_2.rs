use regex::Regex;
use std::{cmp, collections::HashMap, fs};

fn main() {
    let input: String = fs::read_to_string("inputs/day2input.txt").expect("Failed to read input");
    let total: u32 = input
        .trim()
        .split('\n')
        .fold(0, |acc, l| acc + analyze_game(l));
    println!("Day 2 Part 2 Solution: {}", total);
}

fn analyze_game(s: &str) -> u32 {
    let game_r: Regex = Regex::new(
        r#"(?x)
      \s (\d+) \s (red | green | blue) ,*
    "#,
    )
    .unwrap();
    let mut colors: HashMap<&str, u32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let rounds: Vec<&str> = s[s.find(':').expect("Malformed input") + 1..]
        .split(';')
        .collect::<Vec<&str>>();
    rounds.iter().for_each(|r|
        game_r.captures_iter(r).for_each(|c| {
            c.iter()
                .skip(1)
                .flat_map(|x| x)
                .collect::<Vec<_>>()
                .windows(2)
                .for_each(|w| {
                    colors.insert(
                        w[1].as_str(),
                        cmp::max(
                            *colors.get(w[1].as_str()).expect("Invalid color"),
                            w[0].as_str().parse::<u32>().unwrap(),
                        ),
                    );
                })
        }));
    return colors.values().fold(1, |acc, x| acc * x);
}

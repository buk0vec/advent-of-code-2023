use regex::Regex;
use std::{fs, collections::HashMap};

fn main() {
  let input: String = fs::read_to_string("inputs/day2input.txt").expect("Failed to read input");
  let total: u32 = input.trim().split('\n').fold(0, | acc, l | acc + analyze_game(l));
  println!("Day 2 Part 1 Solution: {}", total);
}


fn analyze_game(s: &str) -> u32 {
  let gid_r: Regex = Regex::new(r#"Game (\d+)"#).unwrap();
  let game_r: Regex = Regex::new(
    r#"(?x)
      \s (\d+) \s (red | green | blue) ,*
    "#).unwrap();
  let colors: HashMap<&str, u32> = HashMap::from([("red", 12), ("green" ,13), ("blue", 14)]);
  let gid: u32 = gid_r.captures(s).expect("No game ID")[1].parse::<u32>().unwrap();
  let rounds: Vec<&str> = s[s.find(':').expect("Malformed input") + 1..].split(';').collect::<Vec<&str>>();
  for round in rounds {
    if game_r.captures_iter(round).any(|c|
      c.iter().skip(1).flat_map(|x| x).collect::<Vec<_>>().windows(2).any(|w| 
       match (w[0].as_str(), w[1].as_str()) {
        (n, color) => 
          return colors.get(color).expect("Invalid color") < &n.parse::<u32>().expect("failed to parse amount"),
       } 
      )
    ) {
      return 0
    }
  }
  return gid;
}
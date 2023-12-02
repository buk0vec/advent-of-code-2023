use std::fs;


fn main() {
  let input: String = fs::read_to_string("inputs/day1input.txt").expect("Failed to read input");
  let total: u32 = input.split('\n').fold(0, | acc, l | acc + get_number(l));
  println!("Day 1 Part 1 Solution: {}", total);
}

fn get_number(s: &str) -> u32 {
  let nums: Vec<u32> = s.chars()
  .filter_map(|c| c.to_digit(10))
  .collect();
  let f = nums.first();
  let l = nums.last();
  match (f, l) {
    (Some(f), Some(l)) => return 10 * f + l,
    _ => return 0
  }
}

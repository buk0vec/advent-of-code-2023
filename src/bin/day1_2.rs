use std::fs;


fn main() {
  let input: String = fs::read_to_string("inputs/day1input.txt").expect("Failed to read input");
  let total: u32 = input.split('\n').fold(0, | acc, l | acc + get_number(l));
  println!("Day 1 Part 2 Solution: {}", total);
}

fn get_number(s: &str) -> u32 {
  let valid_digits   = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
  let dlen = [3, 3, 5, 4, 4, 3, 5,  5, 4];
  let mut first: u32 = 0;
  let mut last: u32 = 0;
  let mut remaining = valid_digits.map(|_| 0);
  for c in s.chars() {
    if c.is_numeric() {
      remaining = valid_digits.map(|_| 0);
      if first == 0 {
        first = c.to_digit(10).expect("Failed digit conversion");
      }
      last = c.to_digit(10).expect("Failed digit conversion");
    }
    else {
      for i in 0..9 {
        let next_char = valid_digits[i].chars().nth(remaining[i]).expect("Invalid char access");
        if next_char == c {
          remaining[i] += 1;
          if remaining[i] == dlen[i] {
            if first == 0 {
              first = (i as u32) + 1;
            }
            last = (i as u32) + 1;
            remaining[i] = 0;
          }
        } else {
          remaining[i] = 0;
          if valid_digits[i].chars().nth(0).expect("Failed to get 1st char") == c {
            remaining[i] +=  1;
          }
        }
      }
    }
  }
  println!("{} {}", s, first * 10 + last);
  return (first * 10) + last;
}
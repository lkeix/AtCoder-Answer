use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn min(a: i64, b:i64) -> i64 {
  if a > b {
    return b;
  }
  return a;
}

fn main() {
  let s: String = input();
  let k: usize = input();
  let mut res1: char = '1';
  let mut sequence1: String = "".to_string();
  for c in s.chars() {
    if c != '1' {
      res1 = c;
      break;
    } else {
      sequence1 = format!("{}1", sequence1);
    }
  }
  if sequence1.len() >= k {
    println!("{}", 1);
  } else {
    println!("{}", res1);
  }
}
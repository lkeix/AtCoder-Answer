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
  let n: String = input();
  const Up: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let target: String = n.chars().skip(2).take(n.len() - 3).collect();
  let mut condition1: bool = false;
  let mut condition2: bool = false;
  let mut upperNum: i64 = 0;

  if n.chars().nth(0).unwrap() == 'A' {
    condition1 = true
  }
  for i in target.chars() {
    if i == 'C' {
      condition2 = true;
    }
  }
  for i in n.chars() {
    for j in Up.chars() {
      if i == j {
        upperNum += 1;
      }
    }
  }
  if condition1 && condition2 && upperNum == 2 {
    println!("AC");
  } else {
    println!("WA");
  }
}
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
  let n: i64 = input();
  if 0 < n && n < 1200 {
    println!("ABC");
  } else if 1200 <= n && n < 2800 {
    println!("ARC");
  } else {
    println!("AGC");
  }
}
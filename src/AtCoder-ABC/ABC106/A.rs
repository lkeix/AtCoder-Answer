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
  let v: Vec<i64> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
  let a: i64 = v[0];
  let b: i64 = v[1];
  println!("{}", (a * b) - a - b + 1);
}
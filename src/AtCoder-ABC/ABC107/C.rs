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
  let mut s: String = input();
  let mut v: Vec<i64> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
  let n: usize = v[0] as usize;
  let k: usize = v[1] as usize;
  s = input();
  v = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
  let mut cost: i64 = 10000000000;
  let mut i: usize = 0;
  for i in 0..n-k+1 {
    let left: i64 = v[i];
    let right: i64 = v[i + k - 1];
    let origin: i64 = min(left.abs(), right.abs());
    let distance: i64 = right - left;
    cost = min(cost, origin + distance);
  }
  println!("{}", cost);
}
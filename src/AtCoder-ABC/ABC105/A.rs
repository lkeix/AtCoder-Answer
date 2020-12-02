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
  let v: Vec<&str> = s.split(" ").collect();
  let n: i64 = v[0].parse::<i64>().unwrap();
  let k: i64 = v[1].parse::<i64>().unwrap();
  if n % k != 0 {
    println!("1");
  } else {
    println!("0")
  }
}
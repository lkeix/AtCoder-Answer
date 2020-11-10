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
  for i in 0..25 {
    for j in 0..15 {
      if i*4 + j*7 == n {
        println!("Yes");
        return;
      }
    }
  }
  println!("No");
}
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
  let t: String = input();
  for i in 0..s.len() {
    s = format!("{}{}", s.chars().skip(1).take(s.len() - 1).collect::<String>(), s.chars().skip(0).take(1).collect::<String>());
    if s == t {
      println!("Yes");
      return;
    }
  }
  println!("No");
}
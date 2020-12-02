use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn min(a: i64, b: i64) -> i64 {
  if a > b {
    return b;
  }
  return a;
}

fn max(a: i64, b: i64) -> i64 {
  if a > b {
    return a;
  }
  return b;
}

fn main() {
  let mut s: String = input();
  let mut res: String = "".to_string();
  for i in s.chars() {
    if i != 'B' {
      res = format!("{}{}", res, i);
    } else if res.len() > 0{
      res.remove(res.len() - 1);
    }
  }
  println!("{}", res);
}
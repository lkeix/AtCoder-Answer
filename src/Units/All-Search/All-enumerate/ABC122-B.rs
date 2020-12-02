use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn max(a: i64, b: i64) -> i64 {
  if a > b {
    return a;
  }
  return b;
}

fn main() {
  let mut s: String = input();
  let v: String = "ATGC".to_string();
  let mut cnt: i64 = 0;
  let mut res: i64 = 0;
  for chr in s.chars() {
    let mut matched = false;
    for i in v.chars() {
      if chr == i {
        matched = true;
      }
    }
    if matched {
      cnt += 1;
      res = max(res, cnt);
    } else {
      cnt = 0;
    }
  }
  println!("{}", res);
}
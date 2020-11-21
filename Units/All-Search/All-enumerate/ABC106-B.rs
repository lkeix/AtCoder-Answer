use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn main() {
  let mut n: i64 = input();
  let mut res: i64 = 0;
  for i in 1..n+1 {
    let mut cnt: i64 = 0;
    if i % 2 == 0 {
      continue;
    }
    for j in 1..n+1 {
      if i % j == 0 {
        cnt += 1;
      }
    }
    if cnt == 8 {
      res += 1;
    }
  }
  println!("{}", res);
}
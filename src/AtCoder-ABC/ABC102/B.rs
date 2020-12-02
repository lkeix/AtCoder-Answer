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
  let n: usize = input();
  let s: String = input();
  let mut v: Vec<&str> = s.split(' ').collect();
  v.sort();
  let mut numv: Vec<i64> = Vec::new();
  let mut res: i64 = -1;
  for i in v {
    numv.push(i.parse::<i64>().unwrap());
  }
  for i in 0..n {
    for j in 0..n {
      res = max(res, numv[i] - numv[j]);
    }
  }
  println!("{}", res);
}
use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn solve(n: i64, x: i64) -> i64 {
  let mut res: i64 = 0;
  for i in 1..n+1 {
    for j in i..n+1 {
      for k in j..n+1 {
        if i != k && j != k && i != j && i + j + k == x {
          res += 1;
        }
      }
    }
  }
  return res;
}

fn main() {
  let mut s: String = input();
  let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut res: i64 = 0;
  while v[0] != 0 || v[1] != 0 {
    res = solve(v[0], v[1]);
    println!("{}", res);
    s = input();
    v = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  }
}
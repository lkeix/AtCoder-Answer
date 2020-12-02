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
  let v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut a: Vec<Vec<i64>> = vec![];
  for i in 0..v[0] {
    s = input();
    let tmp: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    a.push(tmp);
  }
  let mut res: i64 = 0;
  for i in 0..v[1] {
    for j in i..v[1] {
      let mut tmp: i64 = 0;
      for k in 0..v[0] {
        tmp += max(a[k][i], a[k][j]);
      }
      res = max(res, tmp);
    }
  }
  println!("{}", res);
}
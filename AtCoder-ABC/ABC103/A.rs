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
  let n: String = input();
  let s: Vec<&str> = n.split(' ').collect();
  let mut a: Vec<i64> = vec![];
  let mut a_s: Vec<Vec<usize>> = vec![];
  let mut cost: i64 = 0;
  for i in s {
    a.push(i.parse::<i64>().unwrap());
  }
  for i in 0..3 {
    for j in 0..3 {
      for k in 0..3 {
        if i != j && j != k && i != k {
          let mut v: Vec<usize> = vec![i, j, k];
          a_s.push(v);
        }
      }
    }
  }
  let mut res: i64 = 1000000000000;
  for i in a_s {
    let mut last: i64 = a[i[0]];
    let mut cost: i64 = 0;
    for j in 0..i.len() {
      cost += i64::abs(a[i[j]] - last);
      last = a[i[j]];
    }
    res = min(res, cost);
  }
  println!("{}", res);
}
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
  let mut n: i64 = input();
  let mut a: Vec<i64> = vec![];
  let mut b: Vec<i64> = vec![];
  for i in 0..n {
    let mut s: String = input();
    let mut tmp: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    a.push(tmp[0]);
    b.push(tmp[1]);
  }
  a.sort();
  b.sort();
  let ent: i64 = a[a.len() / 2];
  let exi: i64 = b[b.len() / 2];
  let mut res: i64 = 0;
  for i in 0..n {
    let mut tmp: i64 = (a[i as usize] - b[i as usize]).abs();
    let mut tmp1: i64 = (ent - a[i as usize]).abs() + (exi - b[i as usize]).abs();
    res += tmp + tmp1;
  }
  println!("{}", res);
}
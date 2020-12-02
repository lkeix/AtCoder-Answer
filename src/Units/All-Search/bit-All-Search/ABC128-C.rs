use std::io::*;
use std::str::FromStr;
// use std::collections::*;

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
  let mut tmp: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut n: i64 = tmp[0];
  let mut m: i64 = tmp[1];
  let mut switches: Vec<Vec<i64>> = vec![];
  let mut p: Vec<i64> = vec![];
  for _i in 0..m {
    s = input();
    let mut a_i: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    switches.push(a_i[1..a_i.len()].to_vec());
  }
  s = input();
  p = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut bits: Vec<Vec<i64>> = vec![];
  for bit in 0..(1<<n) {
    tmp = vec![];
    for j in 0..n {
      if bit & (1<<j) == 0 {
        &tmp.push(j + 1);
      }
    }
    bits.push(tmp);
  }
  let mut ans: i64 = 0;
  for bit in &bits {
    let mut cnt: i64 = 0;
    for i in 0..p.len() {
      let sb: Vec<i64> = sub(bit.to_vec(), switches[i].to_vec());
      if (sb.len() % 2) as i64 == p[i] {
        cnt += 1;
      }
    }
    if cnt == p.len() as i64 {
      ans += 1;
    }
  }
  println!("{}", ans);
}

fn include(mut x: Vec<i64>, mut y: Vec<i64>) -> bool {
  x.sort();
  y.sort();
  if x.len() < y.len() {
    return false;
  }
  let mn: usize = y.len();
  for i in 0..mn {
    if x[i] != y[i] {
      return false;
    }
  }
  return true;
}

fn sub(mut x: Vec<i64>, mut y: Vec<i64>) -> Vec<i64> {
  x.sort();
  y.sort();
  let mut res: Vec<i64> = vec![];
  for i in x {
    for j in &y {
      if i == *j {
        res.push(i);
      }
    }
  }
  return res
}
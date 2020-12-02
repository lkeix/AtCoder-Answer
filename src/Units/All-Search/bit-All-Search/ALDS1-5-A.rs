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
  let mut n: i64 = input();
  let mut s: String = input();
  let mut a_i: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut m: i64 = input();
  s = input();
  let mut p_i: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut sums: Vec<i64> = vec![];
  for bit in 0..(1<<n) {
    let mut sum: i64 = 0;
    for i in 0..n {
      if bit & (1<<i) == 0 {
        sum += a_i[i as usize];
      }
    }
    sums.push(sum);
  }
  for i in p_i {
    let mut ok: bool = false;
    for sum in &sums {
      if i == *sum {
        ok = true;
      }
    }
    if ok {
      println!("yes");
    } else {
      println!("no");
    }
  }
}
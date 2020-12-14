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
 
fn reverse(base: Vec<i64>) -> Vec<i64> {
  let mut res: Vec<i64> = vec![];
  for i in base {
    if i == 0 {
      res.push(1);
    } else {
      res.push(0);
    }
  }
  return res;
}
 
fn f(mut s: String) -> i64 {
  let mut res: i64 = 0;
  let chars: Vec<char> = s.chars().collect();
  let mut itr: usize = 0;
  while itr < chars.len() - 1 {
    if chars[itr] == chars[itr + 1] {
      res += 1;
      itr += 1;
    }
    itr += 1;
  }
  return res;
}

fn main() {
  let mut n: usize = input();
  let mut a: Vec<i64> = vec![];
  let mut b: Vec<i64> = vec![];
  let mut c: Vec<i64> = vec![];
  for i in 0..n {
    let mut s: String = input();
    let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    a.push(v[0]);
    b.push(v[1]);
    c.push(v[2]);
  }
  let mut dp: Vec<i64> = vec![0;3];
  dp[0] = a[0];
  dp[1] = b[0];
  dp[2] = c[0];
  for i in 1..n {
    let mut next: Vec<i64> = vec![0;3];
    // aを選んだ時の最大
    next[0] = max(dp[1] + a[i], dp[2] + a[i]);
    // bを選んだ時の最大
    next[1] = max(dp[0] + b[i], dp[2] + b[i]);
    // cを選んだ時の最大
    next[2] = max(dp[0] + c[i], dp[1] + c[i]);
    dp = next.clone();
  }
  println!("{}", max(dp[0], max(dp[1], dp[2])));
}
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
  let mut s: String = input();
  let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let n: usize = v[0] as usize;
  let w: usize = v[1] as usize;
  let mut items: Vec<(i64, i64)> = Vec::new();
  for _i in 0..n {
    s = input();
    v = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    items.push((v[0], v[1]));
  }
  let mut dp: Vec<i64> = vec![0;w+1];
  for i in 0..n {
    for j in 0..w+1 {
      if j >= items[i].1 as usize {
        dp[j] = max(dp[j], dp[j - items[i].1 as usize] + items[i].0);
      } else {
        dp[j] = dp[j];
      }
    }
  }
  println!("{}", dp[w]);
}
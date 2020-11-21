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
  let mut k: i64 = input();
  let mut chars: Vec<char> = s.chars().collect();
  let mut cnt: i64 = 0;
  // 全て要素が同じ時
  if chars.iter().all(|&ch| ch == chars[0]) {
    let ans: usize = chars.len() * k as usize / 2;
    println!("{}", ans);
    return;
  }
  let mut t1: i64 = f(s.clone());
  let mut t2: i64 = f(format!("{}{}", s.clone().to_string(), s.clone().to_string()));
  let d: i64 = t2 - t1;
  let a: i64 = t1;
  println!("{}", a + (k - 1) * d);
}
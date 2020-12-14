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
  let mut t: String = input();
  let mut cs: Vec<char> = vec![' '];
  let mut ct: Vec<char> = vec![' '];
  if s.len() < t.len() {
    let mut tmp: String = s;
    s = t;
    t = tmp;
  }
  for i in s.chars() {
    cs.push(i)
  }
  for i in t.chars() {
    ct.push(i)
  }
  let mut n: usize = cs.len() + 1;
  let mut m: usize = ct.len();
  let mut dp: Vec<i64> = vec![0;m];
  let mut table: Vec<Vec<i64>> = vec![];
  for i in 0..cs.len() {
    let mut next: Vec<i64> = vec![0;m];
    for j in 0..ct.len() {
      if i == 0 || j == 0 {
        continue;
      } else if cs[i] == ct[j] {
        next[j] = dp[j - 1] + 1
      } else {
        next[j] = max(dp[j], next[j - 1])
      }
    }
    table.push(dp.clone());
    dp = next.clone();
  }
  table.push(dp.clone());
  // 復元
  let mut ans: String = "".to_string();
  let mut i: i64 = cs.len() as i64;
  let mut j: i64 = ct.len() as i64 - 1;
  while i > 0 && j > 0 {
    if table[i as usize][j as usize] == table[i as usize-1][j as usize] { // 上から更新されていた場合
      i -= 1;
    } else if table[i as usize][j as usize] == table[i as usize][j as usize-1] { // 横から更新されていた場合
      j -= 1;
    } else { // 一致していた時
      ans = format!("{}{}", cs[i as usize - 1], ans);
      i -= 1;
      j -= 1;
    }
  }
  println!("{}", ans);
}
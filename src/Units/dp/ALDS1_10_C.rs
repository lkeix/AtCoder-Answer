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
  let n: usize = input();
  let mut dp: Vec<i64> = vec![0;50];
  for _i in 0..n+1 {
    let mut xtmp: String = input();
    let mut ytmp: String = input();
    let mut x: Vec<char> = vec![' '];
    let mut y: Vec<char> = vec![' '];
    for i in xtmp.chars() {
      x.push(i);
    }
    for i in ytmp.chars() {
      y.push(i)
    }
    let mut next: Vec<i64> = vec![0;50];
    for i in 0..x.len() {
      for j in 0..y.len() {
        if i == 0 || j == 0 {
          continue;
        } else if x[i] == y[j] {
          // 文字列が一致した時は次の行へ+1
          next[j] = dp[j - 1] + 1;
        } else {
          // 一致しない場合は値を最大値を保持したい
          // 前の行の一致数と今の行の一致数の最大値を次の列の値とする
          next[j] = max(dp[j], next[j - 1]);
        }
        // 行の更新
        dp = next.clone();
      }
    }
  }
}
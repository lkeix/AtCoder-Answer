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
  let n: i64 = v[0];
  let k: i64 = v[1];
  let mut num: Vec<i64> = vec![0; n as usize + 1];
  for i in 1..n as usize + 1 {
    // kのあまりが1以上n以下の個数を数え上げる
    num[i % k as usize] += 1;
  }
  let mut ans: i64 = 0;
  for a in 0..k as usize {
    let b: usize = ((k as i64 - a as i64) % k as i64) as usize;
    let c: usize = ((k as i64 - a as i64) % k as i64) as usize;
    // b + c がkの倍数でないときはカウントしないのでスキップ
    if (b + c) as i64 % k != 0 {
      continue;
    }
    // 組み合わせを足す
    ans += (num[a] * num[b] * num[c]);
  }
  println!("{}", ans);
}
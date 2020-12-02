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

fn main() {
  let mut s: String = input();
  let mut a_i: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let r: i64 = a_i[0];
  let c: i64 = a_i[1];
  let mut senbei: Vec<Vec<i64>> = vec![];
  for i in 0..r {
    s = input();
    a_i = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    senbei.push(a_i.clone());
  }
  let mut rev: Vec<Vec<i64>> = vec![];
  for i in &senbei {
    rev.push(reverse(i.to_vec()));
  }
  let mut ans: i64 = 0;
  for bit in 0..(1<<r) {
    let mut newSenbei: Vec<Vec<i64>> = senbei.clone();
    // 反転させる
    for i in 0..r {
      if bit & (1<<i) == 0 {
        newSenbei[i as usize] = rev[i as usize].clone();
      }
    }
    let mut tmp: i64 = 0;
    // おせんべいの最大を求める
    for i in 0..c {
      let mut cnt: i64 = 0;
      for j in 0..r {
        if newSenbei[j as usize][i as usize] == 1 {
          cnt += 1;
        }
      }
      tmp += max(r - cnt, cnt);
    }
    ans = max(ans, tmp);
  }
  println!("{}", ans);
}
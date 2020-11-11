use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn min(a: i64, b:i64) -> i64 {
  if a > b {
    return b
  }
  return a
}

fn max(a: i64, b:i64) -> i64 {
  if a > b {
    return a
  }
  return b
}

fn bin_s1(v: &Vec<i64>, mut left: usize, mut right: usize) -> (usize, usize) {
  let i: usize = right;
  while right > left + 1 {
    let mid: usize = left + (right - left) / 2;
    if v[mid] >= v[i] - v[mid] {
      right = mid;
    } else {
      left = mid;
    }
  }
  return (left, right);
}

fn bin_s2(v: &Vec<i64>, mut left: usize, mut right: usize) -> (usize, usize) {
  let i: usize = left;
  while right > left + 1 {
    let mid: usize = left + (right - left) / 2;
    if v[mid] - v[i] >= v[v.len() - 1] - v[mid] {
      right = mid;
    } else {
      left = mid;
    }
  }
  return (left, right);
}

fn main() {
  let n: usize = input();
  let s: String = input();
  let v: Vec<&str> = s.split(' ').collect();
  let mut accv: Vec<i64> = vec![];
  let mut acc: i64 = 0;
  accv.push(0);
  for i in 0..v.len() {
    acc += v[i].parse::<i64>().unwrap();
    accv.push(acc);
  }
  let mut res: i64 = 1000000000000000;
  for i in 0..n+1 {
    let mut pair = bin_s1(&accv, 0, i);
    let mut left: usize = pair.0 as usize;
    let mut right: usize = pair.1 as usize;
    // それぞれの区間和を算出
    // 小さい方の区切りの時
    let p1: i64 = accv[left];
    let q1: i64 = accv[i] - accv[left];
    // 大きい方の区切りの時
    let p2: i64 = accv[right];
    let q2: i64 = accv[i] - accv[right];
    let mut left_min: i64 = min(p1, q1);
    let mut left_max: i64 = max(p1, q1);
    // より均等なものを選ぶ
    if i64::abs(p1 - q1) > i64::abs(p2 - q2) {
      left_min = min(p2, q2);
      left_max = max(p2, q2);
    }
    pair = bin_s2(&accv, i, n);
    left = pair.0 as usize;
    right = pair.1 as usize;
    // それぞれの区間和を算出
    let r1: i64 = accv[left] - accv[i];
    let s1: i64 = accv[n] - accv[left];
    let r2: i64 = accv[right] - accv[i];
    let s2: i64 = accv[n] - accv[right];
    let mut right_min: i64 = min(r1, s1);
    let mut right_max: i64 = max(r1, s1);
    // より均等なものを選ぶ
    if i64::abs(r1 - s1) > i64::abs(r2 - s2) {
      right_min = min(r2, s2);
      right_max = max(r2, s2);
    }
    let mx: i64 = max(left_max, right_max);
    let mn: i64 = min(left_min, right_min);
    res = min(res, i64::abs(mx) - i64::abs(mn));
  }
  println!("{}", res);
}
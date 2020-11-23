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

// キーが存在するかどうかを返す
fn binarySearch(ary: &Vec<i64>, key: i64) -> bool {
  let mut r: usize = ary.len();
  let mut l: usize = 0;
  while r - l > 0 {
    let mut mid: usize = l + (r - l) / 2;
    if key < ary[mid] {
      r = mid;
    } else if key > ary[mid] {
      l = mid + 1;
    } else {
      return true;
    }
  }
  return false;
}

fn f(p: f64, x: f64) -> f64 {
  return p / 2.0_f64.powf(x / 1.5);
}

fn fd(p: f64, x: f64) -> f64 {
  let pow: f64 = 2.0_f64.powf(-1.0/1.5_f64);
  return 1.0 + p * pow.log(1.0_f64.exp()) * 2.0_f64.powf(-1.0 * (x / 1.5));
}

fn main() {
  let mut p: f64 = input();
  let mut l: f64 = 0.0;
  let mut r: f64 = p;
  // 微分値して二分探索で最小値を見つける解法
  for i in 0..1000 {
    let mid: f64 = l + (r - l) / 2.0;
    // 要する時間が最小値よりも小さい場合
    if fd(p, mid) == 0.0 {
      l = mid;
      break;
    } else if fd(p, mid) > 0.0 {
      r = mid;
    } else if fd(p, mid) < 0.0 {
      l = mid;
    }
  }
  l = 0.0;
  r = p;
  // 三分探索で最小値を見つける解法
  for i in 0..1000 {
    let mid13: f64 = l + (r - l) / 3.0;
    let mid23: f64 = l + 2.0*(r - l) / 3.0;
    if mid13 + f(p, mid13) > mid23 + f(p, mid23) { // 右側の方が小さい時 -> 区間の左側はmid13よりも大きいことは自明 => 左の区間を狭める
      l = mid13;
    } else { // 左側の方が小さい時 -> 区間の右側はmid23よりも大きいことは自明 => 右の区間を狭める
      r = mid23;
    }
  }
  println!("{}", l + f(p, l));
}
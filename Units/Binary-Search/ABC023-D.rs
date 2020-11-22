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

fn lower_bound(ary: &Vec<i64>, key: i64) -> usize {
  let mut l: usize = 0;
  let mut r: usize = ary.len();
  while r > l {
    let mid: usize = l + (r - l) / 2;
    if ary[mid] >= key {
      r = mid;
    } else {
      l = mid + 1;
    }
  }
  return l;
}

fn upper_bound(ary: &Vec<i64>, key: i64) -> usize {
  let mut l: usize = 0;
  let mut r: usize = ary.len();
  while r > l {
    let mid: usize = l + (r - l) / 2;
    if ary[mid] > key {
      r = mid;
    } else {
      l = mid + 1;
    }
  }
  return l;
}

fn check(v: &Vec<(i64, i64)>, key: i64) -> bool {
  // 全ての値がkey以下か判定
  let mut cnt: Vec<i64> = vec![0; v.len()];
  for i in v {
    // 既に高度を越えている時
    if i.0 > key {
      return false;
    }
    let x: usize = ((key - i.0) / i.1) as usize;
    // 風船を割った場所に+1、超えている場合は末尾に+1
    cnt[min(v.len() as i64 - 1, x as i64) as usize] += 1;
  }
  // 割った風船の累積和を取る
  for i in 0..cnt.len() - 1 {
    cnt[i + 1] += cnt[i];
  }
  // 1回の射撃で2回割ったとかのデータがあった場合はfalse
  for i in 0..cnt.len() {
    if cnt[i] > i as i64 + 1 { 
      return false;
    }
  }
  return true;
}

fn main() {
  let mut n: i64 = input();
  let mut v: Vec<(i64, i64)> = vec![];
  for i in 0..n {
    let mut s: String = input();
    let tmp: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    v.push((tmp[0], tmp[1]));
  }
  let mut l: i64 = 0;
  let mut r: i64 = 1000000000000000000;
  while l < r {
    let mut mid: i64 = l + (r - l) / 2;
    if !check(&v, mid) {
      l = mid + 1;
    } else {
      r = mid;
    }
  }
  println!("{}", l);
}
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

fn main() {
  let mut n: i64 = input();
  let mut s: String = input();
  let mut a: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  s = input();
  let mut b: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  s = input();
  let mut c: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut res: i64 = 0;
  a.sort();
  b.sort();
  c.sort();
  for i in b {
    let al: i64 = lower_bound(&a, i) as i64;
    let cl: i64 = n - upper_bound(&c, i) as i64;
    res += al * cl;
  }
  println!("{}", res);
}
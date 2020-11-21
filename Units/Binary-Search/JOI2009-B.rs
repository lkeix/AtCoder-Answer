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


fn main() {
  let d: i64 = input();
  let n: i64 = input();
  let m: i64 = input();
  let mut di: Vec<i64> = vec![];
  let mut ki: Vec<i64> = vec![];
  di.push(0);
  for _i in 0..n-1 {
    let tmp: i64 = input();
    di.push(tmp);
  }
  di.push(d);
  di.sort();
  for _i in 0..m {
    let tmp: i64 = input();
    ki.push(tmp);
  }
  let mut res: i64 = 0;
  for i in ki {
    // 最短距離の店を求める
    let mut l: usize = 0;
    let mut r: usize = di.len();
    while r - l > 1 {
      let mut mid: usize = l + (r - l) / 2;
      if di[mid] > i {
        r = mid;
      } else {
        l = mid;
      }
    }
    res += min(i - di[l], di[r] - i);
  }
  println!("{}", res);
}
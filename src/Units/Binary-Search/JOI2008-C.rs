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
  let mut s: String = input();
  let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let n: i64 = v[0];
  let m: i64 = v[1];
  v = vec![0];
  for _i in 0..n {
    let k: i64 = input();
    v.push(k);
  }
  let mut vv: Vec<i64> = vec![];
  for i in &v {
    for j in &v {
      vv.push(i + j);
    }
  }
  let mut mx: i64 = 0;
  vv.sort();
  for i in &vv {
    let tmp: i64 = *i;
    let mut l: usize = 0;
    let mut r: usize = vv.len() as usize - 1;
    while l < r {
      let mid: usize = l + (r - l) / 2;
      if tmp + vv[mid] <= m {
        mx = max(mx, tmp + vv[mid]);
        l = mid + 1;
      } else if tmp + vv[mid] > m {
        r = mid;
      }
    }
  }
  println!("{}", mx);
}
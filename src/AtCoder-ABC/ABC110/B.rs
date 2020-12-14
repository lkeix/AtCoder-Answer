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
  let n: usize = v[0] as usize;
  let m: usize = v[1] as usize;
  let x: i64 = v[2];
  let y: i64 = v[3];
  s = input();
  let mut x_i: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  s = input();
  let mut y_i: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  x_i.push(x);
  y_i.push(y);
  x_i.sort();
  y_i.sort();
  if x_i[x_i.len() - 1] >= y_i[0] {
    println!("War");
  } else {
    println!("No War");
  }
}
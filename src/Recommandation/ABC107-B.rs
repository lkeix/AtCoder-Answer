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
  let mut v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut board: Vec<Vec<bool>> = Vec::new();
  let r: usize = v[0];
  let c: usize = v[1];
  let mut rs: Vec<usize> = Vec::new();
  let mut cs: Vec<usize> = Vec::new();
  for i in 0..r {
    s = input();
    let mut tmp: Vec<bool> = Vec::new();
    for i in s.chars() {
      if i == '#' {
        tmp.push(false);
      } else if i == '.' {
        tmp.push(true);
      }
    }
    board.push(tmp);
  }
  for i in 0..r {
    let mut ok: bool = true;
    for j in 0..c {
      if !board[i][j] {
        ok = false;
      }
    }
    if ok {
      rs.push(i);
    }
  }
  for i in 0..c {
    let mut ok: bool = true;
    for j in 0..r {
      if !board[j][i] {
        ok = false;
      }
    }
    if ok {
      cs.push(i);
    }
  }
  for i in 0..r {
    if rs.contains(&i) {
      continue;
    }
    let mut tans: String = "".to_string();
    for j in 0..c {
      if cs.contains(&j) {
        continue;
      }
      if board[i][j] {
        tans += &".".to_string();
      } else {
        tans += &"#".to_string();
      }
    }
    println!("{}", tans);
  }
}
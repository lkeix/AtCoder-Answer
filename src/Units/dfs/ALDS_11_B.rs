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
  let mut n: usize = input();
  let mut g: Vec<Vec<bool>> = Vec::new();
  let mut tmp: Vec<bool> = Vec::new();
  let mut d: Vec<i64> = vec![];
  let mut f: Vec<i64> = vec![];
  let mut visited: Vec<bool> = vec![];
  // initialize
  for _i in 0..n {
    tmp.push(false);
    visited.push(false);
  }
  for _i in 0..n {
    g.push(tmp.clone());
  }
  for _i in 0..n {
    d.push(0);
    f.push(0);
  }
  for i in 0..n {
    let mut s: String = input();
    let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for j in 0..v[1] as usize {
      g[i][v[j + 2] as usize - 1] = true;
    }
  }
  let mut cnt: i64 = 0;
  let mut stack: Vec<usize> = vec![];
  for i in 0..visited.len() {
    // 訪問していない時は訪問していない場所をスタックに入れて探索回数 + 1する
    if !visited[i] {
      stack.push(i);
      cnt += 1;
    }
    // DFS
    while stack.len() != 0 {
      let mut v: usize = stack[stack.len() - 1];
      // 初回訪問時
      if !visited[v] {
        f[v] = cnt;
        visited[v] = true;
      }
      let num: usize = stack.len();
      for i in 0..g[v].len() {
        // 枝が存在する時
        if g[v][i] && !visited[i] {
          stack.push(i);
          break;
        }
      }
      // 次いくところがない場合
      if num == stack.len() && visited[v] {
        d[v] = cnt + 1;
        stack.pop();
      }
      cnt += 1;
    }
  }
  for i in 0..n {
    println!("{}", format!("{} {} {}", i + 1, f[i], d[i]));
  }
}
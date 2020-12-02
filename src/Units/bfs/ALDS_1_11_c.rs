use std::io::*;
use std::str::FromStr;
use std::collections::*;
 
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
 
// base dfs
/*
fn dfs(g: Vec<Vec<bool>>, v usize) {
  let mut visited: Vec<bool> = Vec::new();
  let mut stack: Vec<usize> = vec![v];
  for _i in 0..g.len() {
    visited.push(false);
  }
  while stack.len() != 0 {
    v = stack[stack.len() - 1];
    let num: usize = stack.len();
    for i in 0..g[v].len() {
      if !visited[v] {
        v.stack(i);
        break;
      }
    }
    if !visited[v] {
      visited[v] = true;
    }
    if num == stack.len() {
      stack.pop();
    }
  }
}
*/

fn bfs(g: Vec<Vec<bool>>, start: usize) -> Vec<i64> {
  let mut que: VecDeque<(usize, i64)> = VecDeque::new();
  let mut res: Vec<i64> = Vec::new();
  for i in 0..g.len() {
    res.push(-1);
  }
  let mut visited = vec![false; g.len()];
  que.push_back((start, 0));
  res[start] = 0;
  visited[start] = true;
  while que.len() != 0 {
    let v: (usize, i64) = que.pop_front().unwrap();
    let num: usize = que.len();
    for to in 0..g[v.0].len() {
      if !visited[to] && g[v.0][to] {
        visited[to] = true;
        que.push_back((to, v.1 + 1));
        res[to] = v.1 + 1;
      }
    }
  }
  return res;
}

fn main() {
  let mut n: usize = input();
  let mut g: Vec<Vec<bool>> = vec![];
  for _i in 0..n {
    g.push(vec![false; n]);
  }
  for _i in 0..n {
    let s: String = input();
    let tmp: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let from: usize = tmp[0];
    let toNum: usize = tmp[1];
    for i in 0..toNum {
      g[from - 1][tmp[i + 2] - 1] = true;
    }
  }
  let mut res = bfs(g.clone(), 0);
  for i in 0..res.len() {
    println!("{} {}", i + 1, res[i]);
  }
}
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
 
fn dfs(mut tree: &Vec<Vec<usize>>,mut accs: Vec<i64>, mut target: usize) -> Vec<i64> {
  let mut res: i64 = accs[0];
  let mut stack: Vec<usize> = vec![0];
  let mut visited = vec![false; tree.len()];
  visited[0] = true;
  while stack.len() != 0 {
    let mut v: usize = stack.pop().unwrap();
    for &k in &tree[v] {
      if !visited[k] {
        accs[k] += accs[v];
        visited[k] = true;
        stack.push(k);
      }
    }
  }
  return accs.to_vec();
}
 
fn main() {
  let mut s: String = input();
  let mut tmp: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let n: usize = tmp[0];
  let q: usize = tmp[1];
  let mut nodes: Vec<Vec<usize>> = Vec::new();
  let mut accs: Vec<i64> = Vec::new();
  for i in 0..n {
    nodes.push(vec![]);
    accs.push(0);
  }
  for i in 0..n-1 {
    s = input();
    tmp = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let start: usize = tmp[0] - 1;
    let end: usize = tmp[1] - 1;
    nodes[start].push(end);
    nodes[end].push(start);
  }
  for i in 0..q {
    s = input();
    tmp = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let tar: usize = tmp[0] - 1;
    let x: i64 = tmp[1] as i64;
    accs[tar] += x;
  }
  let mut ans: String = "".to_string();
  let mut res: Vec<i64> = dfs(&nodes, accs.clone(), 0);
  for i in res {
    ans += &format!("{} ", i).to_string();
  }
  println!("{}", ans);
}
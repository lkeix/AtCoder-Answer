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

fn dfs(mut g: Vec<Vec<bool>>, mut v: (usize, usize), w: usize, h: usize) -> Vec<Vec<bool>> {
  let mut stack: Vec<(usize, usize)> = vec![v];
  while stack.len() != 0 {
    v = stack[stack.len() - 1];
    let num: usize = stack.len();
    // 8近傍の中に移動できる箇所があればスタックにいれる
    for i in 0..3 {
      let mut found: bool = false;
      let mut r: i64 = v.0 as i64 + (i - 1);
      // はみ出す箇所を剪定
      if r < 0 {
        r = 0;
      }
      if r >= h as i64 {
        r = h as i64 - 1
      }
      for j in 0..3 {
        let mut c: i64 = v.1 as i64 + (j - 1);
        // はみ出す箇所を剪定
        if c < 0 {
          c = 0;
        }
        if c >= w as i64 {
          c = w as i64 - 1;
        }
        if g[r as usize][c as usize] {
          stack.push((r as usize, c as usize));
          g[r as usize][c as usize] = false;
          found = true;
        }
      }
      if found {
        break;
      }
    }
    // 移動できる場所がない場合は、popする
    if num == stack.len() {
      stack.pop();
    }
  }
  return g
}

fn main() {
  let mut s: String = input();
  let mut tmp: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut w: usize = tmp[0];
  let mut h: usize = tmp[1];
  let mut answers: Vec<i64> = Vec::new();
  while w != 0 && h != 0 {
    // 入力受け取り
    let mut g: Vec<Vec<bool>> = Vec::new();
    let mut gtmp: Vec<bool> = Vec::new();
    for _i in 0..w {
      gtmp.push(false);
    }
    for i in 0..h {
      s = input();
      g.push(gtmp.clone());
      let rt: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
      for j in 0..rt.len() {
        let val: bool = if rt[j] == 1 { true } else { false }; 
        g[i][j] = val;
      }
    }
    // 処理
    let mut ans: i64 = 0;
    for i in 0..g.len() {
      for j in 0..g[i].len() {
        if g[i][j] {
          ans += 1;
          g = dfs(g, (i, j), w, h);
        }
      }
    }
    answers.push(ans);
    // 次の入力
    s = input();
    tmp = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    w = tmp[0];
    h = tmp[1];
  }
  for i in answers {
    println!("{}", i);
  }
}
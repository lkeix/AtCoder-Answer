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
 
fn dfs(mut board: Vec<Vec<bool>>, mut v: (usize, usize)) -> i64 {
  // 次の移動場所とそこまでのルートを保持するスタック
  let mut stack: Vec<((usize, usize), Vec<(usize, usize)>)> = Vec::new();
  // ルートのデータを保持するpath
  let mut path: Vec<(usize, usize)> = Vec::new();
  let mut res: i64 = 0;
  stack.push((v, vec![]));
  while stack.len() != 0 {
    let tmp = stack.pop().unwrap();
    v = tmp.0;
    path = tmp.1;
    path.push(v);
    let num: usize = stack.len();
    let x1: usize = max(0, v.1 as i64 - 1) as usize;
    let x2: usize = min(v.1 as i64 + 1, board[v.0].len() as i64 - 1) as usize;
    let y1: usize = max(0, v.0 as i64 - 1) as usize;
    let y2: usize = min(v.0 as i64 + 1, board.len() as i64  - 1) as usize;
    // 次の移動場所を調べて、スタックにデータを追加
    if board[v.0][x1] && !path.iter().any(|&(y, x)| x == x1 && y == v.0) {
      stack.push(((v.0, x1), path.clone()));
    }
    if board[v.0][x2] && !path.iter().any(|&(y, x)| x == x2 && y == v.0) {
      stack.push(((v.0, x2), path.clone()));
    }
    if board[y1][v.1] && !path.iter().any(|&(y, x)| x == v.1 && y == y1)  {
      stack.push(((y1, v.1), path.clone()));
    }
    if board[y2][v.1] && !path.iter().any(|&(y, x)| x == v.1 && y == y2){
      stack.push(((y2, v.1), path.clone()));
    }
    // 次の移動場所がない時
    if num == stack.len() {
      res = max(res, path.len() as i64);
    }
  }
  return res;
}
 
fn main() {
  let n: usize = input();
  let m: usize = input();
  let mut board = vec![vec![false; n]; m];
  for i in 0..m {
    let s: String = input();
    let v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for j in 0..n {
      if v[j] == 1 {
        board[i][j] = true;
      }
    }
  }
  let mut ans: i64 = 0;
  for i in 0..m {
    for j in 0..n {
      if board[i][j] {
        let cmp: i64 = dfs(board.clone(), (i, j));
        ans = max(ans, cmp);
        // println!("{}", cmp);
      }
    }
  }
  println!("{}", ans);
}
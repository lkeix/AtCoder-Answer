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

fn bfs(mut board: Vec<Vec<(bool, i64)>>, sr: usize, sc: usize, er: usize, ec: usize) -> i64 {
  let mut now: (isize, isize) = (sr as isize, sc as isize);
  let mut que: VecDeque<(isize, isize)> = VecDeque::new();
  que.push_back(now);
  while que.len() != 0 {
    now = que.pop_front().unwrap();
    let vecs: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for v in vecs {
      if now.0 + v.0 >= 0 &&
      now.0 + v.0 < board.len() as isize &&
      now.1 + v.1 >= 0 &&
      now.1 + v.1 < board[now.0 as usize].len() as isize {
        if board[(now.0 + v.0) as usize][(now.1 + v.1) as usize].0 {
          board[(now.0 + v.0) as usize][(now.1 + v.1) as usize].1 = board[now.0 as usize][now.1 as usize].1 + 1;
          que.push_back((now.0 + v.0, now.1 + v.1));
          board[(now.0 + v.0) as usize][(now.1 + v.1) as usize].0 = false;
        }
      }
    }
  }
  return board[er][ec].1;
}

fn main() {
  let mut s: String = input();
  let mut tmp: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let r: usize = tmp[0];
  let c: usize = tmp[1];
  s = input();
  tmp = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let sr: usize = tmp[0] - 1;
  let sc: usize = tmp[1] - 1;
  s = input();
  tmp = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let er: usize = tmp[0] - 1;
  let ec: usize = tmp[1] - 1;
  let mut board: Vec<Vec<(bool, i64)>> = vec![];
  for _i in 0..r {
    s = input();
    let row: Vec<char> = s.chars().collect();
    let mut t: Vec<(bool, i64)> = vec![];
    for j in 0..row.len() {
      // 通路の場合
      if row[j] == '.' {
        t.push((true, 0));
      }
      if row[j] == '#' {
        t.push((false, -1));
      }
    }
    board.push(t);
  }
  println!("{}", bfs(board.clone(), sr, sc, er, ec));
}
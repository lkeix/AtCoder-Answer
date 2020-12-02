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

fn canreach(mut board: Vec<Vec<(bool, i64)>>, start: (usize, usize), end: (usize, usize)) -> (bool, i64) {
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  que.push_back(start);
  board[start.0][start.1] = (false, 0);
  while que.len() != 0 {
    let v: (usize, usize) = que.pop_front().unwrap();
    let direct: Vec<(isize, isize)> = [(-1, 0), (1, 0), (0, -1), (0, 1)].to_vec();
    for i in direct {
      let next: (isize, isize) = (v.0 as isize + i.0 as isize, v.1 as isize + i.1 as isize);
      if (next.0 >= 0 && next.0 < board.len() as isize) && (next.1 >= 0 && next.1 < board[v.0].len() as isize) {
        if board[next.0 as usize][next.1 as usize].0 {
          board[next.0 as usize][next.1 as usize].0 = false;
          board[next.0 as usize][next.1 as usize].1 = board[v.0 as usize][v.1 as usize].1 + 1;
          que.push_back((next.0 as usize, next.1 as usize));
        }
      }
    }
    if v.0 == end.0 && v.1 == end.1 {
      return (true, board[v.0][v.1].1 + 1)
    }
  }
  return (false, -1);
}

fn main() {
  let mut s: String = input();
  let mut hw: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut board: Vec<Vec<(bool, i64)>> = Vec::new();
  let start: (usize, usize) = (0, 0);
  let end: (usize, usize) = (hw[0], hw[1]);
  let mut fcnt: i64 = 0;
  for _i in 0..end.0 {
    s = input();
    let chars: Vec<char> = s.chars().collect();
    let mut tmp: Vec<(bool, i64)> = Vec::new();
    for j in chars {
      if j == '.' {
        tmp.push((true, 0));
      } else {
        tmp.push((false, 0));
        fcnt += 1;
      }
    }
    board.push(tmp);
  }
  let tmp: (bool, i64) = canreach(board.clone(), start, (end.0 - 1, end.1 - 1));
  if tmp.0 {
    let h: i64 = hw[0] as i64;
    let w: i64 = hw[1] as i64;
    let res: i64 = h * w - fcnt - tmp.1;
    println!("{}", res);
  } else {
    println!("{}", -1);
  }
}
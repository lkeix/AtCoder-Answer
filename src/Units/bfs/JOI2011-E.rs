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
*/

fn bfs(mut g: Vec<Vec<(bool, i64)>>, r: usize, c: usize, now: i64) -> (usize, usize, i64, i64) {
  g[r][c].0 = false;
  let mut target: i64 = now;
  let mut que: VecDeque<(usize, usize, i64, i64)> = VecDeque::new();
  que.push_back((r, c, 0, target));
  let maxr: usize = g.len();
  let maxc: usize = g[0].len();
  let mut v: (usize, usize, i64, i64) = que[0];
  while que.len() != 0 {
    v = que.pop_front().unwrap();
    if g[v.0][v.1].1 == target {
      return v;
    }
    let next: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for i in next {
      let tmpr: isize = v.0 as isize + i.0;
      let tmpc: isize = v.1 as isize + i.1;
      if (tmpr >= 0 && tmpr < maxr as isize) && (tmpc >= 0 && tmpc < maxc as isize) {
        if g[tmpr as usize][tmpc as usize].0 {
          g[tmpr as usize][tmpc as usize].0 = false;
          que.push_back((tmpr as usize, tmpc as usize, v.2 + 1, target));
        }
      }
    }
  }
  return v;
}


fn main() {
  let mut s: String = input();
  let mut v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let row: usize = v[0];
  let col: usize = v[1];
  let cheeze_num: i64 = v[2] as i64;
  let mut g: Vec<Vec<(bool, i64)>> = Vec::new();
  let mut sr: usize = 0;
  let mut sc: usize = 0;
  for i in 0..row {
    s = input();
    let mut col: usize = 0;
    let mut tmp: Vec<(bool, i64)> = Vec::new();
    for ch in s.chars() {
      let mut is_num: bool = false;
      for i in "123456789".chars() {
        if ch == i {
          is_num = true;
          let t: String = ch.to_string();
          let num: i64 = t.parse::<i64>().unwrap();
          tmp.push((true, num));
        }
      }
      if !is_num {
        if ch == '.' {
          tmp.push((true, 0));
        } else if ch == 'X' {
          tmp.push((false, 0));
        } else if ch == 'S' {
          sr = i;
          sc = col;
          tmp.push((true, 0));
        }
      }
      col += 1;
    }
    g.push(tmp);
  }
  let mut dst: i64 = 0;
  for i in 0..cheeze_num {
    let tmp: (usize, usize, i64, i64) = bfs(g.clone(), sr, sc, i + 1);
    sr = tmp.0;
    sc = tmp.1;
    dst += tmp.2;
  }
  println!("{}", dst);
}
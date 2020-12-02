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

// 建物に囲まれた範囲を列挙して埋めたものを返す
fn closedbfs(mut g: Vec<Vec<i64>>, j: usize, i: usize, w: usize, h: usize) -> Vec<Vec<i64>> {
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  que.push_back((j, i));
  let mut ok: bool = false;
  let cpw: isize = w as isize;
  let cph: isize = h as isize;
  let base: Vec<Vec<i64>> = g.clone();
  g[i][j] = 1;
  while que.len() != 0 {
    // 六近傍を取得する
    let mut v: (usize, usize) = que.pop_front().unwrap();
    let mut leftup: (isize, isize) = (v.0 as isize - 1, v.1 as isize - 1);
    let mut rightup: (isize, isize) = (v.0 as isize, v.1 as isize - 1);
    let mut left: (isize, isize) = (v.0 as isize - 1, v.1 as isize);
    let mut right: (isize, isize) = (v.0 as isize + 1, v.1 as isize);
    let mut leftdown: (isize, isize) = (v.0 as isize - 1, v.1 as isize + 1);
    let mut rightdown: (isize, isize) = (v.0 as isize, v.1 as isize + 1);
    if v.1 % 2 == 0 {
      leftup = (v.0 as isize, v.1 as isize - 1);
      rightup = (v.0 as isize + 1, v.1 as isize - 1);
      leftdown = (v.0 as isize, v.1 as isize + 1);
      rightdown = (v.0 as isize + 1, v.1 as isize + 1);
    }
    // 左上、右上が範囲外の場合
    if leftup.0 < 0 || leftup.1 < 0 || rightup.0 >= w as isize || rightup.1 < 0 {
      return base;
    }
    // 左右が範囲外の場合
    if left.0 < 0 || right.0 >= w as isize {
      return base;
    }
    // 左下、右下が範囲外の場合
    if leftdown.0 < 0 || leftdown.1 >= h as isize || rightdown.0 >= w as isize || rightdown.1 >= h as isize {
      return base;
    }
    if g[leftup.1 as usize][leftup.0 as usize] == 0 {
      g[leftup.1 as usize][leftup.0 as usize] = 1;
      que.push_back((leftup.0 as usize, leftup.1 as usize));
    }
    if g[rightup.1 as usize][rightup.0 as usize] == 0 {
      g[rightup.1 as usize][rightup.0 as usize] = 1;
      que.push_back((rightup.0 as usize, rightup.1 as usize));
    }
    if g[left.1 as usize][left.0 as usize] == 0 {
      g[left.1 as usize][left.0 as usize] = 1;
      que.push_back((left.0 as usize, left.1 as usize));
    }
    if g[right.1 as usize][right.0 as usize] == 0 {
      g[right.1 as usize][right.0 as usize] = 1;
      que.push_back((right.0 as usize, right.1 as usize));
    }
    if g[leftdown.1 as usize][leftdown.0 as usize] == 0 {
      g[leftdown.1 as usize][leftdown.0 as usize] = 1;
      que.push_back((leftdown.0 as usize, leftdown.1 as usize));
    }
    if g[rightdown.1 as usize][rightdown.0 as usize] == 0 {
      g[rightdown.1 as usize][rightdown.0 as usize] = 1;
      que.push_back((rightdown.0 as usize, rightdown.1 as usize));
    }
  }
  return g;
}

fn bfs(mut g: Vec<Vec<i64>>, j: usize, i: usize, w: usize, h: usize) -> (Vec<Vec<i64>>, i64) {
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  que.push_back((j, i));
  let mut ok: bool = false;
  let cpw: isize = w as isize;
  let cph: isize = h as isize;
  let base: Vec<Vec<i64>> = g.clone();
  g[i][j] = -1;
  let mut res: i64 = 0;
  while que.len() != 0 {
    // 六近傍を取得する
    let mut tmp: i64 = 0;
    let mut v: (usize, usize) = que.pop_front().unwrap();
    let mut leftup: (isize, isize) = (v.0 as isize - 1, v.1 as isize - 1);
    let mut rightup: (isize, isize) = (v.0 as isize, v.1 as isize - 1);
    let mut left: (isize, isize) = (v.0 as isize - 1, v.1 as isize);
    let mut right: (isize, isize) = (v.0 as isize + 1, v.1 as isize);
    let mut leftdown: (isize, isize) = (v.0 as isize - 1, v.1 as isize + 1);
    let mut rightdown: (isize, isize) = (v.0 as isize, v.1 as isize + 1);
    if v.1 % 2 == 0 {
      leftup = (v.0 as isize, v.1 as isize - 1);
      rightup = (v.0 as isize + 1, v.1 as isize - 1);
      leftdown = (v.0 as isize, v.1 as isize + 1);
      rightdown = (v.0 as isize + 1, v.1 as isize + 1);
    }
    tmp = 6;
    if leftup.0 >= 0 && leftup.1 >= 0 && leftup.0 < cpw && leftup.1 < cph {
      if g[leftup.1 as usize][leftup.0 as usize] == -1 {
        tmp -= 1;
      }
      if g[leftup.1 as usize][leftup.0 as usize] == 1 {
        g[leftup.1 as usize][leftup.0 as usize] = -1;
        tmp -= 1;
        que.push_back((leftup.0 as usize, leftup.1 as usize));
      }
    }
    if  rightup.0 < cpw && rightup.1 >= 0 && rightup.0 < cpw && rightup.1 < cph {
      if g[rightup.1 as usize][rightup.0 as usize] == -1 {
        tmp -= 1;
      }
      if g[rightup.1 as usize][rightup.0 as usize] == 1 {
        g[rightup.1 as usize][rightup.0 as usize] = -1;
        tmp -= 1;
        que.push_back((rightup.0 as usize, rightup.1 as usize));
      }
    }    
    if left.0 >= 0 && left.0 < cpw && left.1 >= 0 && left.1 < cph {
      if g[left.1 as usize][left.0 as usize] == -1 {
        tmp -= 1;
      }
      if g[left.1 as usize][left.0 as usize] == 1 {
        g[left.1 as usize][left.0 as usize] = -1;
        tmp -= 1;
        que.push_back((left.0 as usize, left.1 as usize));
      }
    }
    if right.0 < cpw && right.0 >= 0 && right.1 >= 0 && right.1 < cph {
      if g[right.1 as usize][right.0 as usize] == -1 {
        tmp -= 1;
      }
      if g[right.1 as usize][right.0 as usize] == 1 {
        g[right.1 as usize][right.0 as usize] = -1;
        tmp -= 1;
        que.push_back((right.0 as usize, right.1 as usize));
      }
    }
    if leftdown.0 >= 0 && leftdown.0 < cpw && leftdown.1 >= 0 && leftdown.1 < cph {
      if g[leftdown.1 as usize][leftdown.0 as usize] == -1 {
        tmp -= 1;
      }
      if g[leftdown.1 as usize][leftdown.0 as usize] == 1 {
        g[leftdown.1 as usize][leftdown.0 as usize] = -1;
        tmp -= 1;
        que.push_back((leftdown.0 as usize, leftdown.1 as usize));
      }
    }
    if rightdown.0 >= 0 && rightdown.0 < cpw && rightdown.1 >= 0 && rightdown.1 < cph {
      if g[rightdown.1 as usize][rightdown.0 as usize] == -1 {
        tmp -= 1;
      }
      if g[rightdown.1 as usize][rightdown.0 as usize] == 1 {
        g[rightdown.1 as usize][rightdown.0 as usize] = -1;
        tmp -= 1;
        que.push_back((rightdown.0 as usize, rightdown.1 as usize));
      }
    }
    res += tmp;
  }
  return (g, res);
}

fn main() {
  let mut s: String = input();
  let mut v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let w: usize = v[0];
  let h: usize = v[1];
  let mut g: Vec<Vec<i64>> = Vec::new();
  for _i in 0..h {
    s = input();
    let tmp: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    g.push(tmp);
  }
  g = closedbfs(g.clone(), 6, 1, w, h);
  for i in 0..h {
    for j in 0..w {
      if g[i][j] == 0 {
        g = closedbfs(g.clone(), j, i, w, h);
      }
    }
  }
  let mut res: i64 = 0;
  for i in 0..h {
    for j in 0..w {
      if g[i][j] == 1 {
        let tmp: (Vec<Vec<i64>>, i64) = bfs(g.clone(), j, i, w, h);
        g = tmp.0;
        res += tmp.1;
      }
    }
  }
  println!("{}", res);
}
use std::io::*;
use std::str::FromStr;
// use std::collections::*;

pub trait LexicalPermutation {
  /// Return `true` if the slice was permuted, `false` if it is already
  /// at the last ordered permutation.
  fn next_permutation(&mut self) -> bool;
  /// Return `true` if the slice was permuted, `false` if it is already
  /// at the first ordered permutation.
  fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
  /// Original author in Rust: Thomas Backman <serenity@exscape.org>
  fn next_permutation(&mut self) -> bool {
      // These cases only have 1 permutation each, so we can't do anything.
      if self.len() < 2 { return false; }

      // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
      let mut i = self.len() - 1;
      while i > 0 && self[i-1] >= self[i] {
          i -= 1;
      }

      // If that is the entire vector, this is the last-ordered permutation.
      if i == 0 {
          return false;
      }

      // Step 2: Find the rightmost element larger than the pivot (i-1)
      let mut j = self.len() - 1;
      while j >= i && self[j] <= self[i-1]  {
          j -= 1;
      }

      // Step 3: Swap that element with the pivot
      self.swap(j, i-1);

      // Step 4: Reverse the (previously) weakly decreasing part
      self[i..].reverse();

      true
  }

  fn prev_permutation(&mut self) -> bool {
      // These cases only have 1 permutation each, so we can't do anything.
      if self.len() < 2 { return false; }

      // Step 1: Identify the longest, rightmost weakly increasing part of the vector
      let mut i = self.len() - 1;
      while i > 0 && self[i-1] <= self[i] {
          i -= 1;
      }

      // If that is the entire vector, this is the first-ordered permutation.
      if i == 0 {
          return false;
      }

      // Step 2: Reverse the weakly increasing part
      self[i..].reverse();

      // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
      let mut j = self.len() - 1;
      while j >= i && self[j-1] < self[i-1]  {
          j -= 1;
      }

      // Step 4: Swap that element with the pivot
      self.swap(i-1, j);

      true
  }

}

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
  let n: i64 = input();
  let mut board: Vec<Vec<bool>> = vec![
    [false;8].to_vec(),
    [false;8].to_vec(),
    [false;8].to_vec(),
    [false;8].to_vec(),
    [false;8].to_vec(),
    [false;8].to_vec(),
    [false;8].to_vec(),
    [false;8].to_vec()
  ];
  for _i in 0..n {
    let s: String = input();
    let v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    board[v[0]][v[1]] = true;
  }
  let mut cmb: Vec<Vec<usize>> = vec![];
  let mut base: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7];
  cmb.push(base.clone());
  while base.next_permutation() {
    cmb.push(base.clone());
  }
  for i in cmb {
    let mut tmp: Vec<Vec<bool>> = board.clone();
    for j in 0..8 {
      for k in 0..i.len() {      
        if j == k {
          tmp[j][i[k]] = true;
        }
      }
    }
    if check(tmp.clone()) {
      display(tmp);
      return;
    }
  }
}

fn rowCheck(row: Vec<bool>) -> bool {
  let mut res: bool = false;
  for col in row {
    res = res | col;
  }
  return res
}

fn check(board: Vec<Vec<bool>>) -> bool {
  let mut res: bool = false;
  for row in 0..board.len() {
    let mut cnt: i64 = 1;
    for col in 0..board[row].len() {
      if board[row][col] {
        // 8近傍の延長をチェックする
        // 右上
        let mut m: usize = min(row as i64 , 7 - col as i64) as usize;
        let mut tmpr: usize = row;
        let mut tmpc: usize = col;
        for _i in 0..m {
          tmpr -= 1;
          tmpc += 1;
          cnt += if board[tmpr][tmpc] { 1 } else { 0 };
        }
        // 右下
        m = min(7 - row as i64, 7 - col as i64) as usize;
        tmpr = row;
        tmpc = col;
        for _i in 0..m {
          tmpr += 1;
          tmpc += 1;
          cnt += if board[tmpr][tmpc] { 1 } else { 0 };
        }
        // 左上
        m = min(row as i64, col as i64) as usize;
        tmpr = row;
        tmpc = col;
        for _i in 0..m {
          tmpr -= 1;
          tmpc -= 1;
          cnt += if board[tmpr][tmpc] { 1 } else { 0 };
        }
        // 左下
        m = min(7 - row as i64, col as i64) as usize;
        tmpr = row;
        tmpc = col;
        for _i in 0..m {
          tmpr += 1;
          tmpc -= 1;
          cnt += if board[tmpr][tmpc] { 1 } else { 0 };
        }
        // 上下
        for i in 0..8 {
          cnt += if board[i][col] && i != row { 1 } else { 0 };
        }
        // 左右
        for i in 0..8 {
          cnt += if board[row][i] && i != col { 1 } else { 0 };
        }
      }
    }
    if cnt > 1 {
      return false;
    }
  }
  return true;
}

fn display(board: Vec<Vec<bool>>) {
  for row in board {
    let mut s: String = "".to_string();
    for col in row {
      s = format!("{}{}", s, if col { "Q" } else { "." })
    }
    println!("{}", s);
  }
}
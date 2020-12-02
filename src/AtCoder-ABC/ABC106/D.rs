use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn min(a: i64, b:i64) -> i64 {
  if a > b {
    return b;
  }
  return a;
}

// Binary Tree
pub enum BinaryTree<T> {
  Nil,
  Node {
    val: T,
    left: Box<BinaryTree<T>>,
    right: Box<BinaryTree<T>>,
  }
}

fn main() {
  let mut s: String = input();
  let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut n: usize = v[0] as usize;
  let mut m: i64 = v[1] as i64;
  let mut q: i64 = v[2];
  let mut accs = vec![[0; 501]; 501];
  let mut lines: Vec<(usize, usize)> = vec![];
  let mut qlines: Vec<(usize, usize)> = vec![];
  for i in 0..m {
    s = input();
    let tmp: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let p = (tmp[0], tmp[1]);
    lines.push(p);
  }
  for i in 0..q {
    s = input();
    let tmp: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let q = (tmp[0], tmp[1]);
    qlines.push(q);
  }
  // 二次元累積和の元の繋がっている箇所を+1する
  for (l, r) in lines {
    accs[l][r] += 1;
  }
  // 二次元累積和を求める
  for i in 0..n {
    for j in 1..n+1 {
      accs[i][j] += accs[i][j - 1];
    }
  }
  for (l, r) in qlines {
    let mut res = 0;
    // クエリの範囲内に治っている区間を計算する
    for i in l..r+1 {
      res += accs[i][r] - accs[i][l - 1];
    }
    println!("{}", res);
  }
}
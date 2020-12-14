use std::io::*;
use std::collections::*;
use std::str::FromStr;

struct BIT {
  tree: Vec<i64>,
  n: usize,
}

impl BIT {
  pub fn init(n: usize) -> Self {
    return BIT {
      tree: vec![0; n+1],
      n: n + 1,
    }
  }

  pub fn add(&mut self, i: usize, v: i64) {
    let mut t: isize = i as isize;
    while t < self.n as isize {
      self.tree[t as usize] += v;
      t += t & (-t);
    }
  }
  
  pub fn sum(&mut self, i: usize) -> i64 {
    let mut t: isize = i as isize;
    let mut sum: i64 = 0;
    while t > 0 {
      sum += self.tree[t as usize];
      t -= t & (-t);
    }
    return sum;
  }
}

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn ok() {

}

fn main() {
  let mut n: usize = input();
  let N: usize = 2 * n + 10;
  let mut s: String = input();
  let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut l: usize = 0;
  let mut r: usize = 1000000000000;
  while r - l > 1 {
    let mid: usize = l + (r - l) / 2;
    let mut b = BIT::init(N);
    let mut num: i64 = 0;
    let mut sum: i64 = 0;
    b.add(n + 1, 1);
    // 反転数を求める
    for i in 0..n {
      if v[i] <= mid as i64 {
        sum += 1;
      } else {
        sum -= 1;
      }
      num += b.sum((sum + n as i64) as usize) - b.sum(0);
      b.add((sum + n as i64 + 1) as usize, 1);
    }
    if num > (n as i64) * (n as i64 + 1) / 4 {
      r = mid;
    } else {
      l = mid;
    }
  }
  println!("{}", r);
}

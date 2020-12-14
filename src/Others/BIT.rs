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
    if self.n < i {
      return;
    }
    let mut t: isize = i as isize;
    while t < self.n as isize {
      self.tree[t as usize] += v;
      t += t & (-t);
    }
  }
  
  pub fn sum(&mut self, i: usize) -> i64 {
    if i <= 0 || i > self.n {
      return 0;
    }
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

fn main() {
  // let v: Vec<i64> = vec![5,3,7,9,6,4,1,2];
  let v: Vec<i64> = vec![1,2,3,4,5];
  let mut b = BIT::init(v.len());
  let mut n: usize = v.len();
  for i in 0..v.len() {
    b.add(i + 1, v[i]);
  }
  println!("{}", b.sum(3));
  println!("{}", b.sum(5));
}

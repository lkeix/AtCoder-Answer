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
  let mut n: i64 = input();
  let mut xy: Vec<(i64, i64)> = vec![];
  for i in 0..n {
    let mut s: String = input();
    let v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    xy.push((v[0], v[1]));
  }
  let mut d: f64 = 0.;
  let mut routes: Vec<Vec<usize>> = vec![];
  let mut base: Vec<usize> =vec![];
  for i in 0..n {
    base.push(i as usize);
  }
  routes.push(base.clone());
  while base.next_permutation() {
    routes.push(base.clone());
  }
  let mut kaijon: i64 = 1;
  for i in 1..n+1 {
    kaijon *= i;
  }
  for route in routes {
    let mut lastxy: (i64, i64) = xy[route[0]];
    for i in route {
      d += calcd(lastxy, xy[i]) / kaijon as f64;
      lastxy = xy[i];
    }
  }
  println!("{}", d);
}

fn calcd(t1: (i64, i64), t2: (i64, i64)) -> f64 {
  let mut sx: f64 = (t1.0 - t2.0) as f64 * (t1.0 - t2.0) as f64;
  let mut sy: f64 = (t1.1 - t2.1) as f64 * (t1.1 - t2.1) as f64;
  return (sx + sy).sqrt();
}
use std::io::*;
use std::str::FromStr;

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

fn main() {
  let mut n: usize = input();
  let mut a: Vec<i64> = Vec::new();
  let mut v: Vec<bool> = vec![false; n];
  for i in 0..n {
    let tmp: i64 = input();
    a.push(tmp);
  }
  let mut f: bool = false;
  let mut next: usize = a[0] as usize - 1;
  let mut ans: i64 = 1;
  v[0] = true;
  while !f {
    if next == 1 {
      println!("{}", ans);
      return;
    }
    f = v[next];
    v[next] = true;
    next = a[next] as usize - 1;
    ans += 1;
  }
  println!("{}", -1);
}
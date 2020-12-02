use std::io::*;
use std::str::FromStr;

// use std::collections::*;

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
  let mut s: String = input();
  let mut tmp: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let n: i64 = tmp[0];
  let m: i64 = tmp[1];
  let mut g: Vec<Vec<bool>> = vec![];
  for _i in 0..12 {
    let mut t: Vec<bool> = vec![];
    for _j in 0..12 {
      t.push(false);
    }
    g.push(t.clone());
  }
  for _i in 0..m {
    s = input();
    tmp = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    g[tmp[0] as usize - 1][tmp[1] as usize - 1] = true;
    g[tmp[1] as usize - 1][tmp[0] as usize - 1] = true;
  }
  let mut ans: i64 = 1;
  for bit in 0..(1<<n) {
    let mut v: Vec<i64> = vec![];
    for i in 0..n {
      if 1 & (bit >> i) == 1 {
        v.push(i);
      }
    }
    let mut ok: bool = true;
    for i in &v {
      for j in &v {
        if *i == *j {
          continue;
        }
        if !g[*i as usize][*j as usize] {
          ok = false;
        }
      }
    }
    if ok {
      ans = max(ans, v.len() as i64);
    }
  }
  println!("{}", ans);
}

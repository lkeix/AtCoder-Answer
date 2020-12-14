use std::io::*;
use std::str::FromStr;
// use std::collections::*;

let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0;4];4];n+1];
 
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

fn solver(mut dp: Vec<Vec<Vec<i64>>>, x: usize, y: usize, z: usize, plan: Vec<i64>) -> i64 {
  let MOD: i64 = 10000;
  if dp[x][y][z] != -1 {
    return dp[x][y][z];
  }
  if x == 1 {
    if plan[x] != 0 {
      if y as i64 != plan[x] {
        return 0;
      }
    }
    return 1;
  }
  if plan[x] != 0 {
    if y as i64 != plan[x] {
      return 0;
    }
  }
  let mut res: i64 = 0;
  if x - 1 == 1 {
    res = solver(dp.clone(), x - 1, z, 1, plan.clone()) % MOD;
  } else {
    if !(y == z && y == 1) {
      res = solver(dp.clone(), x - 1, z, 1, plan.clone()) % MOD;
    }
    if !(y == z && y == 2) {
      res = solver(dp.clone(), x - 1, z, 2, plan.clone()) % MOD;
    }
    if !(y == z && y == 3) {
      res = solver(dp.clone(), x - 1, z, 3, plan.clone()) % MOD;
    }
  }
  return res % MOD;
}

fn main() {
  let mut s: String = input();
  let mut v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let n: usize = v[0];
  let k: usize = v[1];
  let mut plan: Vec<i64> = vec![0; n+1];
  for _i in 0..k {
    s = input();
    let mut tmp: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    plan[tmp[0] as usize] = tmp[1];
  }
  for i in 0..n+1 {
    for j in 0..4 {
      for k in 0..4 {
        dp[i][j][k] = -1;
      }
    }
  }
  let mut ans: i64 = 0;
  for i in 0..3 {
    for j in 0..3 {
      ans += solver(dp.clone(), n, i + 1, j + 1, plan.clone());
    }
  }
  println!("{}", ans);
}
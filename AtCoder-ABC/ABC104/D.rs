use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn main() {
  let s: String = input();
  let sa: Vec<char> = s.trim().chars().collect();
  let n: usize = s.len();
  let mut dp: Vec<i64> = vec![0,0,0,0];
  dp[0] = 1;
  const MOD: i64 = 1000000000 + 7;
  for i in 0..n {
    let mut next: Vec<i64> = vec![0,0,0,0];
    next = dp.clone();
    match sa[i] {
      'A' => {
        next[1] = dp[0] + dp[1];
      }
      'B' => {
        next[2] = dp[1] + dp[2];
      }
      'C' => {
        next[3] = dp[2] + dp[3];
      }
      _ => {
        next[0] = next[0] * 3 % MOD;
        next[1] = (dp[0] + dp[1] * 3) % MOD;
        next[2] = (dp[1] + dp[2] * 3) % MOD;
        next[3] = (dp[2] + dp[3] * 3) % MOD;
      }
    }
    dp = next;
  }
  println!("{}", dp[3] % MOD);
}
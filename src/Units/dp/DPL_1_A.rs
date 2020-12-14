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
  let mut s: String = input();
  let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let n: usize = v[0] as usize;
  let m: usize = v[1] as usize;
  s = input();
  let mut coins: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut dp: Vec<i64> = vec![n as i64;n+1];
  // 0円は0枚なので
  dp[0] = 0;
  for i in coins {
    for j in 1..n as i64 + 1 {
      // コインの金額以上になった時
      if j >= i {
        // 今の金額に達するまでのコインの枚数 = min(過去にその金額まで達したコインの枚数, 新たにコインを追加することで今の金額まで達する時のコインの枚数)
        dp[j as usize] = min(dp[j as usize], dp[(j - i) as usize] + 1);
      }
    }
  }
  println!("{}", dp[n]);
}
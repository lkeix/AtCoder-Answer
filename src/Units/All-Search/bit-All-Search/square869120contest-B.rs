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

fn main() {
  let mut s: String = input();
  let mut v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let n: i64 = v[0];
  let k: i64 = v[1];
  s = input();
  let mut a_i: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut ans: i64 = 1000000000000000000;
  for bit in 0..(1<<n) {
    let mut b: Vec<usize> = vec![];
    // 見える建物の組み合わせを全列挙する
    for i in 0..n {
      if bit & (1<<i) == 0 {
        b.push(i as usize);
      }
    }
    let mut last: i64 = if b.len() == 0 { 0 } else { a_i[0] + 1 };
    let mut tv: Vec<i64> = vec![];
    tv = a_i.clone();
    for i in  1..b.len() {
      if last < a_i[b[i]] { // 大きい場合は高さを変える必要がないため次へ
        last = a_i[b[i]] + 1;
        tv[b[i]] = last - 1;
        continue;
      } else { // 小さい場合は last+1まで高さを引き上げるため、差分を取る
        tv[b[i]] = last;
        last += 1;
      }
    }
    let mut cnt: i64 = 1;
    let mut mx: i64 = tv[0];
    let mut diff: i64 = 0;
    for i in 1..tv.len() {
      if mx < tv[i] {
        cnt += 1;
      }
      mx = max(mx, tv[i]);
    }
    for i in 0..a_i.len() {
      diff += tv[i] - a_i[i];
    }
    if cnt >= k {
      ans = min(ans, diff);
    }
  }
  println!("{}", ans);
}
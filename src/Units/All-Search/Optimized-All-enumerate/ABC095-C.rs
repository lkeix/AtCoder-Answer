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
  let mut n: String = input();
  let mut v: Vec<i64> = n.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut a: i64 = v[0];
  let mut b: i64 = v[1];
  let mut c: i64 = v[2] * 2;
  let mut x: i64 = v[3];
  let mut y: i64 = v[4];
  let mn: i64 = min(x, y);
  let mx: i64 = max(x, y);
  let mut res1: i64 = 0;
  let mut res2: i64 = 0;
  // ABピザを最小個枚数に合わせて買ったとき
  for _i in 0..mn {
    res1 += min(a + b, c);
  }
  for _i in 0..(x - mn) {
    res1 += a;
  }
  for _i in 0..(y - mn) {
    res1 += b;
  }
  // ABピザを最大枚数に合わせて買ったとき
  for _i in 0..mx {
    res2 += min(a + b, c);
  }
  println!("{}", min(res1, res2));
}
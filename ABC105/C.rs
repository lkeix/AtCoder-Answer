use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn min(a: i64, b:i64) -> i64 {
  if a > b {
    return b;
  }
  return a;
}

fn main() {
  let mut n: i64 = input();
  let mut out: String = "".to_string();
  if n == 0 {
    println!("0");
    return;
  }
  while n != 0 {
    let mut r: i64 = n % 2;
    if r == 0 {
      out = "0".to_string() + &out;
    } else {
      r = 1;
      out = "1".to_string() + &out;
    }
    n -= r;
    n /= (-2);
  }
  println!("{}", out);
}
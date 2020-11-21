use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn main() {
  let n: i64 = input();
  if n % 2 == 0 {
    println!("{}", n);
  } else {
    println!("{}", n * 2);
  }
}
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
  let mut num = 0;
  for cha in s.as_str().chars() {
    if cha == '+' {
      num += 1
    } else if cha == '-' {
      num -= 1
    }
  }
  println!("{}", num)
}
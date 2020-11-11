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
  let v: Vec<&str> = s.split(' ').collect();
  let n: i64 = (v[0].to_string()).parse::<i64>().unwrap();
  let k: i64 = (v[1].to_string()).parse::<i64>().unwrap();
  let _tmp: String = input();
  let mut left: i64 = k;
  let mut r: i64 = 1;
  while left < n {
    left += k - 1;
    r += 1;
  }
  println!("{}", r)
}
use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn main() {
  let n: usize = input();
  let s: String = input();
  let v: Vec<&str> = s.split(' ').collect();
  let mut numv: Vec<i64> = Vec::new();
  for i in 0..v.len() {
    let tmp: i64 = i as i64;
    numv.push(v[i].parse::<i64>().unwrap() - (tmp + 1));
  }
  numv.sort();
  let b: i64 = numv[n / 2];
  let mut res: i64 = 0;
  for i in 0..v.len() {
    res += i64::abs(numv[i] - b);
  }
  println!("{}", res);
}
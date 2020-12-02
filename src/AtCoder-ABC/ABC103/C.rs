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
  let t: String = input();
  let a: Vec<&str> = t.split(' ').collect();
  let mut v: Vec<i64> = vec![];
  for i in 0..n {
    v.push(a[i].parse::<i64>().unwrap());
  }
  let mut sum: i64 = 0;
  for i in 0..n {
    sum += v[i] - 1;
  }
  println!("{}", sum);
}
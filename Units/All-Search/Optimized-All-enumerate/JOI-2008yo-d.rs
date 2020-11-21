use std::io::*;
use std::str::FromStr;
use std::collections::*;

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
  let mut n: i64 = input();
  let mut map: HashMap<(i64, i64), bool> = HashMap::new();
  let mut s: String = input();
  let base: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut vecs: Vec<(i64, i64)> = vec![];
  for i in 1..n {
    s = input();
    let tmp: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    vecs.push((tmp[0] - base[0], tmp[1] - base[1]));
  }
  let mut m: i64 = input();
  for i in 0..m {
    s = input();
    let v: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    map.entry((v[0], v[1])).and_modify(|x| *x = true).or_insert(true);
  }
  for (key, val) in &map {
    let mut isSeiza: bool = true;
    for j in &vecs {
      if isSeiza {
        isSeiza = map.contains_key(&(key.0 + j.0, key.1 + j.1));
      }
    }
    if isSeiza {
      println!("{} {}", key.0 - base[0], key.1 - base[1]);
    }
  }
}
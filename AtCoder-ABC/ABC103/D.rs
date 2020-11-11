use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn main() {
  let tmp: String = input();
  let _tmp: Vec<&str> = tmp.split(' ').collect();
  let _n: usize = _tmp[0].parse::<usize>().unwrap();
  let m: usize = _tmp[1].parse::<usize>().unwrap();
  let mut v: Vec<(i64, i64)> = vec![];
  for _i in 0..m {
    let bridge: String = input();
    let btmp: Vec<&str> = bridge.split(' ').collect();
    let s: i64 = btmp[0].parse::<i64>().unwrap();
    let e: i64 = btmp[1].parse::<i64>().unwrap();
    v.push((s, e));
  }
  v.sort_by(|a, b| a.1.cmp(&b.1) );
  let mut e: i64 = 0;
  let mut res: i64 = 0;
  for i in v {
    println!("{}", e);
    if i.0 >= e {
      e = i.1;
      res += 1;
    }
  }
  println!("{}", res);
}
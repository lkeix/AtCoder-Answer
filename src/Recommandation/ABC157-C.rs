use std::io::*;
use std::str::FromStr;
use std::char::*;

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
  let mut ts: String = input();
  let mut tv: Vec<i64> = ts.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let mut hash: Vec<i64> = vec![];
  let n: usize = tv[0] as usize;
  let m: usize = tv[1] as usize;
  let mut s: Vec<i64> = vec![];
  let mut c: Vec<i64> = vec![];
  let mut res: i64 = 1000;
  for _i in 0..m {
    ts = input();
    tv = ts.split_whitespace().map(|x| x.parse().unwrap()).collect();
    s.push(tv[0]);
    c.push(tv[1]);
  }
  let mut pow: i64 = 1;
  for i in 0..1000 {
    let mut tmp: String = i.to_string();
    if tmp.len() == n {
      let mut ok: bool = true;
      for j in 0..m {
        let cmp = tmp.chars().nth(s[j as usize] as usize - 1).unwrap();
        let cmp1 = from_digit(c[j as usize] as u32, 10).unwrap();
        if cmp != cmp1 {
          ok = false;
        }
      }
      if ok {
        println!("{}", i);
        return;
      }
    }
  }
  println!("{}", -1);
}
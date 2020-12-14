use std::io::*;
use std::str::FromStr;
// use std::collections::*;
 
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
 
fn reverse(base: Vec<i64>) -> Vec<i64> {
  let mut res: Vec<i64> = vec![];
  for i in base {
    if i == 0 {
      res.push(1);
    } else {
      res.push(0);
    }
  }
  return res;
}
 
fn f(mut s: String) -> i64 {
  let mut res: i64 = 0;
  let chars: Vec<char> = s.chars().collect();
  let mut itr: usize = 0;
  while itr < chars.len() - 1 {
    if chars[itr] == chars[itr + 1] {
      res += 1;
      itr += 1;
    }
    itr += 1;
  }
  return res;
}

fn idx_of(mut s: &String, mut target: String) -> i64 {
  let cs: Vec<char> = s.chars().collect();
  let ts: Vec<char> = target.chars().collect();
  for i in 0..cs.len()-ts.len()+1 {
    let mut f: bool = true;
    for j in 0..ts.len() {
      if cs[i + j] != ts[j] {
        f = false;
      }
    }
    if f {
      return i as i64;
    }
  }
  return -1;
}
 
fn main() {
  let mut s: String = input();
  let cs: Vec<char> = s.chars().collect();
  let keyidx: i64 = idx_of(&s, "key".to_string());
  let enceidx: i64 = idx_of(&s, "ence".to_string());
  // 前の部分文字列を取り除く時
  if keyidx + 3 == enceidx && enceidx + 4 == cs.len() as i64 {
    println!("YES");
    return;
  }
  // 中間の部分文字列を取り除く時
  if keyidx == 0 && enceidx + 4 == cs.len() as i64 {
    println!("YES");
    return;
  }
  // 後ろの部分文字列を取り除く時
  if keyidx == 0 && enceidx + 4 == 7 {
    println!("YES");
    return;
  }
  println!("NO");
}
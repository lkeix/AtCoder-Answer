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

fn main() {
  let mut n: usize = input();
  let mut map: HashMap<String, bool> = HashMap::new();
  let mut s: String = input();
  let mut chars: Vec<char> = s.chars().collect();
  let mut last: char = chars[chars.len() - 1];
  let mut ok: bool = true;
  map.entry(s).or_insert(true);
  for _i in 0..n-1 {
    s = input();
    let mut chars: Vec<char> = s.chars().collect();
    if last != chars[0] {
      ok = false;
    }
    let mut f: bool = map.contains_key(&s);
    if f {
      ok = false;
    }
    last = chars[chars.len() - 1];
    map.entry(s).or_insert(true);
  }
  if ok {
    println!("Yes");
  } else {
    println!("No");
  }
}
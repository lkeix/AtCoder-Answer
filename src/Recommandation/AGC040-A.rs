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
  let chars: Vec<char> = s.chars().collect();
  let mut a: Vec<i64> = vec![0; s.len() + 1];
  let mut l: usize = 0;
  let mut r: usize = 0;
  while l < chars.len() {
    let base: usize = l;
    if chars[l] == '<' {
      r = l + 1;
      if r >= chars.len() {
        break;
      }
      while chars[r] == '<' {
        r += 1;
        if r >= chars.len() {
          break;
        }
      }
      r = if r + 1 >= chars.len() { r } else { r + 1 };
      let mut tmp: i64 = 0;
      a[l] = tmp;
      while l < r {
        tmp += 1;
        l += 1;
        a[l] = tmp;
      }
      if base == 0 {
        l = r - 1;
      }
    } else if chars[l] == '>' {
      r = l;
      let mut tmp: i64 = 0;
      while chars[r] == '>' {
        tmp += 1;
        r += 1;
        if r >= chars.len() {
          break;
        }
      }
      if a[base] - r as i64 >= 0 {
        tmp = a[base];
      }
      while l < r {
        a[l] = tmp;
        tmp -= 1;
        l += 1;
      }
    }
  }
  if chars[chars.len() - 1] == '<' {
    let end: usize = a.len() - 1;
    let tar: usize = if a.len() - 2 < 0 { 0 } else { a.len() - 2 };
    a[end] = (a[tar] + 1);
  }
  let mut sum: i64 = 0;
  for i in &a {
    sum += i;
  }
  /*
  let mut res: String = "".to_string();
  for i in 0..chars.len() {
    res += &format!("{} {} ", a[i], chars[i]);
  }
  res += &format!("{}", a[chars.len()]);
  println!("{}", res);
  println!("{:?}", &a);
  */
  println!("{}", sum);
}
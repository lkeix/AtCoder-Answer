use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn min(a: i64, b:i64) -> i64 {
  if a > b {
    return b;
  }
  return a;
}

fn main() {
  let mut s: String = input();
  let v: Vec<i64> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
  let h: usize = v[0] as usize;
  let w: usize = v[1] as usize;
  let mut hw: Vec<Vec<char>> = vec![];
  for i in 0..h {
    s = input();
    let tmp: Vec<char> = s.chars().collect();
    hw.push(tmp);
  }
  let mut rw: Vec<usize> = vec![];
  let mut rh: Vec<usize> = vec![];
  for i in 0..w {
    let mut skip: bool = true;
    for j in 0..h {
      if hw[j][i] == '#' {
        skip = false;
      }
    }
    if !skip {
      rw.push(i);
    }
  }
  for i in 0..h {
    let mut skip: bool = true;
    for j in 0..w {
      if hw[i][j] == '#' {
        skip = false;
      }
    }
    if !skip {
      rh.push(i);
    }
  }
  for &i in &rh {
    let mut output: String = "".to_string();
    for &j in &rw {
      output = format!("{}{}", output, hw[i][j]);
    }
    println!("{}", output);
  }
}
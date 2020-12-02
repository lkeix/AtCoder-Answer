use std::io::*;
use std::str::FromStr;

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
  let mut s: String = input();
  let mut ans: i64 = 0;
  let mut sa: Vec<char> = s.chars().collect();
  // 000 ~ 999 までを全列挙
  for i in 0..10 {
    let mut digit: i64 = 0;
    for j in 0..10 {
      for k in 0..10 {
        // 文字列を走査し当てはまる桁の値を更新
        for l in 0..n {
          let mut tmp: i64 = sa[l as usize].to_string().parse::<i64>().unwrap();
          if tmp == i && digit == 0 { // 1桁目
            digit += 1;
            continue;
          }
          if tmp == j && digit == 1 { // 2桁目
            digit += 1;
            continue;
          }
          if tmp == k && digit == 2 { // 3桁目
            ans += 1;
            digit = 0;
            break;
          }
        }
        // 桁を0に戻す
        digit = 0;
      }
    }
  }
  println!("{}", ans);
}
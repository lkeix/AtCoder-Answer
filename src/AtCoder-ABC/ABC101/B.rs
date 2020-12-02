use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn main() {
  let n: String = input();
  let mut digsum: i64 = 0;
  for cha in n.as_str().chars() {
    let num = (cha.to_string()).parse::<i64>().unwrap();
    digsum += num;
  }
  let num: i64 = n.parse::<i64>().unwrap();
  if num % digsum == 0 {
    println!("Yes");
  } else {
    println!("No");
  }
}
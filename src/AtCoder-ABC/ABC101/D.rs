use std::io::*;
use std::str::FromStr;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn digitsum(mut n: i64) -> i64 {
  let mut sum: i64 = 0;
  while n > 0 {
    sum += n % 10;
    n /= 10;
  }
  return sum;
}

fn snuke(n: i64) -> f64 {
  return n as f64 / digitsum(n) as f64;
}


fn main() {
  let s: String = input();
  let k: usize = (s.to_string()).parse::<usize>().unwrap();
  let mut base: Vec<i64> = vec![];
  let mut dig = 1;
  for _i in 0..15 {
    for j in 1..200 {
      let val: i64 = dig * (j + 1) - 1;
      base.push(val);
    }
    dig *= 10;
  }
  let mut i: usize = 0;
  base.sort();
  base.dedup();
  while i < base.len() {
    let mut j: usize = i + 1;
    while j < base.len() {
      if snuke(base[i]) > snuke(base[j]) {
        let tar: i64 = base[i];
        base.retain(|&x| x != tar);
        i -= 1;
        break;
      }
      j += 1;
    }
    i += 1;
  }
  base.sort();
  for i in 0..k {
    println!("{}", base[i]);
  }
}
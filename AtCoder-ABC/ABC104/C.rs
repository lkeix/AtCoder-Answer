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
  let n: String = input();
  let v: Vec<&str> = n.split(" ").collect();
  let d: i64 = v[0].parse::<i64>().unwrap();
  let g: i64 = v[1].parse::<i64>().unwrap();
  let mut prob: Vec<Vec<i64>> = vec![];
  for i in 0..d {
    let tmp: String = input();
    let tmpv: Vec<&str> = tmp.split(" ").collect();
    let mut p: i64 = tmpv[0].parse::<i64>().unwrap();
    let mut c: i64 = tmpv[1].parse::<i64>().unwrap();
    prob.push(vec![p, c]);
  }
  let mut res: i64 = 100000000000;
  let x: usize = 1<<d;
  for i in 0..x {
    let mut sum: i64 = 0;
    let mut num: i64 = 0;
    for j in 0..d {
      if i & (1<<(j)) != 0 {
        num += prob[j as usize][0];
        sum += prob[j as usize][1] + (j + 1) as i64 * 100 * prob[j as usize][0];
      }
    }
    if sum >= g {
      res = min(res, num);
    } else {
      for k in (0..d).rev() {
        // 全完した箇所は除外
        if i & (1<<(k)) != 0 {
          continue;
        }
        for _l in 0..prob[k as usize][0] {
          if sum >= g {
            break;
          }
          sum += (k + 1) as i64 * 100;
          num += 1;
        }
      }
      res = min(res, num);
    }
  }
  println!("{}", res);
}
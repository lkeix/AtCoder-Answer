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

fn main() {
  let mut n: String = input();
  let mut v: Vec<(i64, i64)> = vec![];
  let mut map: HashMap::<(i64, i64), bool> = HashMap::new();
  let num: i64 = n.parse::<i64>().unwrap();
  for i in 0..num {
    n = input();
    let tmpv: Vec<i64> = n.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut tmp: (i64, i64) = (tmpv[0], tmpv[1]);
    // let key: String = format!("{}-{}", tmp.0.to_string(), tmp.1.to_string());
    map.entry(tmp).and_modify(|x| *x = true).or_insert(true);
    v.push(tmp);
  }
  let mut res: i64 = 0;
  for i in 0..v.len() {
    for j in i+1..v.len() {
      let xyvec: (i64, i64) = (v[j].0 - v[i].0, v[j].1 - v[i].1);
      let mut p1: bool = map.contains_key(&((v[i].0 - xyvec.1), (v[i].1 + xyvec.0)));
      let mut p2: bool = map.contains_key(&((v[j].0 - xyvec.1), (v[j].1 + xyvec.0)));
      // 境界の右上に正方形があるか確かめる
      if p1 && p2{
        res = max(res, xyvec.0 * xyvec.0 + xyvec.1 * xyvec.1);
        continue;
      }
      p1 = map.contains_key(&((v[i].0 - xyvec.1), (v[i].1 - xyvec.0)));
      p2 = map.contains_key(&((v[j].0 - xyvec.1), (v[j].1 - xyvec.0)));
      // 境界の右下に正方形があるか確かめる
      if p1 && p2{
        res = max(res, xyvec.0 * xyvec.0 + xyvec.1 * xyvec.1);
        continue;
      }
      // 境界の左上に正方形があるか確かめる
      p1 = map.contains_key(&((v[i].0 + xyvec.1), (v[i].1 + xyvec.0)));
      p2 = map.contains_key(&((v[j].0 + xyvec.1), (v[j].1 + xyvec.0)));
      if p1 && p2 {
        res = max(res, xyvec.0 * xyvec.0 + xyvec.1 * xyvec.1);
        continue;
      }
      // 境界の左下に正方形があるか確かめる
      p1 = map.contains_key(&((v[i].0 + xyvec.1), (v[i].1 - xyvec.0)));
      p2 = map.contains_key(&((v[j].0 + xyvec.1), (v[j].1 - xyvec.0)));
      if p1 && p2{
        res = max(res, xyvec.0 * xyvec.0 + xyvec.1 * xyvec.1);
        continue;
      }
    }
  }
  println!("{}", res);
}
use std::io::*;
use std::str::FromStr;
use std::collections::HashMap;

// Quoted from https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn input<T: FromStr>() -> T {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn main() {
  let mut s: String = input();
  let sa: Vec<&str> = s.split(" ").collect();
  let n: usize = sa[0].parse::<usize>().unwrap();
  let m: i64 = sa[1].parse::<i64>().unwrap();
  s = input();
  let ni_s: Vec<&str> = s.split(" ").collect();
  let mut ni: Vec<i64> = vec![0];
  let mut sum: i64 = 0;
  let mut mp: HashMap<i64, i64> = HashMap::new();
  for i in ni_s {
    sum += i.parse::<i64>().unwrap();
    ni.push(sum);
  }
  let mut res: i64 = 0;
  for i in ni {
    mp.entry(i % m).and_modify(|x| *x += 1).or_insert(1);
    res += *mp.get(&(i % m)).unwrap() - 1;
  }
  println!("{}", res);
}
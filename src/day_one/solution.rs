use crate::day_one::input;
use std::{cmp, vec};

pub fn solve_p1() -> i32 {
  let mut max: i32 = 0;
  let mut curr: i32 = 0;

  const DATA: &str = input::DATA;
  let lines = DATA.split("\n");

  for line in lines {
    if line == "" {
      max = cmp::max(curr, max);
      curr = 0;
    } else {
      curr += line.parse::<i32>().expect("Line item not string");
    }
  }

  return max;
}

pub fn solve_p2() -> i32 {
  let mut top_three = vec![0, 0, 0];
  let mut curr: i32 = 0;

  const DATA: &str = input::DATA;
  let lines = DATA.split("\n");

  for line in lines {
    if line == "" {
      replace_max(curr, &mut top_three);
      curr = 0;
    } else {
      curr += line.parse::<i32>().expect("Line item not string");
    }
  }

  let mut total: i32 = 0;
  for val in top_three {
    total += val;
  }

  return total;
}

fn replace_max(curr: i32, top_three: &mut Vec<i32>) {
  let mut insert_idx: Option<usize> = None;
  if curr > top_three[0] {
    insert_idx = Some(0);
  } else if curr > top_three[1] {
    insert_idx = Some(1);
  } else if curr > top_three[2] {
    insert_idx = Some(2);
  }
  if insert_idx.is_some() {
    top_three.insert(insert_idx.unwrap(), curr);
    top_three.pop();
  }
}
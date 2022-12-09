use crate::day_three::input;

pub fn solve_p1() -> i32 {
  const DATA: &str = input::DATA;
  let lines = DATA.split("\n");

  let mut total: i32 = 0;

  for line in lines {
    println!("{:?}", line)
  }

  total
}
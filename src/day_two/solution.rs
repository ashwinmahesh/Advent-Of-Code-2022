use crate::day_two::input;
use std::collections::HashMap;


const OPP_ROCK: &str = &"A";
const OPP_PAPER: &str = &"B";
const OPP_SCISSORS: &str = &"C";

const ROCK: &str = &"X";
const PAPER: &str = &"Y";
const SCISSORS: &str = &"Z";

const LOSE: i32 = 0;
const TIE: i32 = 3;
const WIN: i32 = 6;

struct UserMove {
  beats: String,
  points: i32
}

pub fn solve_p1() -> i32 {
  const DATA: &str = input::DATA;
  let lines = DATA.split("\n");

  let mut score: i32 = 0;

  let mut move_map: HashMap<&str, UserMove> = HashMap::new();

  move_map.insert(
    ROCK,
    UserMove { 
    beats: String::from(OPP_SCISSORS),
    points: 1 
  });
  
  move_map.insert(
    PAPER,
    UserMove { 
    beats: String::from(OPP_ROCK),
    points: 2 
  });

  move_map.insert(
    SCISSORS,
    UserMove { 
    beats: String::from(OPP_PAPER),
    points: 3
  });

  for line in lines {
    let mut round = line.split(" ");
    let opp = round.next().unwrap();
    let user = round.next().unwrap();

    let user_move = move_map.get(user).expect("Invalid user move");
    score += user_move.points;

    if *opp == *user_move.beats {
      score += WIN;
    } else if *opp == *user {
      score += TIE;
    } else {
      score += LOSE;
    }
  }

  return score;
}


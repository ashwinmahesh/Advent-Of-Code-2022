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

const LOSE_ACTION: &str = &"X";
const TIE_ACTION: &str = &"Y";
const WIN_ACTION: &str = &"Z";

struct UserMove {
  beats: String,
  points: i32,
  opp_equal: String,
}

pub fn solve_p1() -> i32 {
  const DATA: &str = input::DATA;
  let lines = DATA.split("\n");

  let mut score: i32 = 0;

  let move_map = create_user_move_map();

  for line in lines {
    let mut round = line.split(" ");
    let opp = round.next().unwrap();
    let user = round.next().unwrap();

    let user_move = move_map.get(user).expect("Invalid user move");
    score += calc_points(user_move, opp);
  }

  score
}

struct OpponentMove {
  win: String,
  lose: String,
  tie: String,
}

pub fn solve_p2() -> i32 {
  let mut score: i32 = 0;

  const DATA: &str = input::DATA;
  let lines = DATA.split("\n");

  let move_map = create_user_move_map();
  let opp_move_map = create_opponent_move_map();


  for line in lines {
    let mut round = line.split(" ");
    let opp = round.next().unwrap();
    let action = round.next().unwrap();

    let opp_move = opp_move_map.get(opp).expect("invalid opponent move");

    let user_action: &str = match action {
      LOSE_ACTION => &opp_move.lose[..],
      TIE_ACTION => &opp_move.tie[..],
      WIN_ACTION => &opp_move.win[..],
      _ => &""[..]
    };

    let user_move = move_map.get(user_action).expect("invalid user action");
    score += calc_points(user_move, opp)
  }

  score
}

fn calc_points(user_move: &UserMove, opp: &str) -> i32 {
  let mut score: i32 = 0;
  score += user_move.points;

  if *opp == *user_move.beats {
    score += WIN;
  } else if opp.eq(&user_move.opp_equal[..]) {
    score += TIE;
  } else {
    score += LOSE;
  }

  score
}

fn create_user_move_map() -> HashMap<String, UserMove> {
  let mut move_map: HashMap<String, UserMove> = HashMap::new();

  move_map.insert(
    String::from(ROCK),
    UserMove { 
      beats: String::from(OPP_SCISSORS),
      points: 1,
      opp_equal: String::from(OPP_ROCK),
  });
  
  move_map.insert(
    String::from(PAPER),
    UserMove { 
      beats: String::from(OPP_ROCK),
      points: 2,
      opp_equal: String::from(OPP_PAPER),
  });

  move_map.insert(
    String::from(SCISSORS),
    UserMove { 
      beats: String::from(OPP_PAPER),
      points: 3,
      opp_equal: String::from(OPP_SCISSORS),
  });

  move_map
}

fn create_opponent_move_map() -> HashMap<String, OpponentMove> {
  let mut move_map: HashMap<String, OpponentMove> = HashMap::new();

  move_map.insert(
    String::from(OPP_ROCK),
    OpponentMove { 
      win: String::from(PAPER),
      lose:String::from(SCISSORS),
      tie: String::from(ROCK)
    }
  );

  move_map.insert(
    String::from(OPP_PAPER),
    OpponentMove { 
      win: String::from(SCISSORS),
      lose:String::from(ROCK),
      tie: String::from(PAPER)
    }
  );

  move_map.insert(
    String::from(OPP_SCISSORS),
    OpponentMove { 
      win: String::from(ROCK),
      lose:String::from(PAPER),
      tie: String::from(SCISSORS)
    }
  );

  move_map
}

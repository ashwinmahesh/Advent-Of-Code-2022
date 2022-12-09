pub mod day_one;
pub mod day_two;
pub mod day_three;

fn main() {
    // DAY 3
    // let max = day_one::solution::solve_p1();
    // println!("Max 1: {max}");

    // let top_three = day_one::solution::solve_p2();
    // println!("Final Top Three: {:?}", top_three);

    // DAY 2
    // let score1 = day_two::solution::solve_p1();
    // println!("Score Part 1: {:?}", score1);

    // let score2 = day_two::solution::solve_p2();
    // println!("Score Part 2: {:?}", score2);

    let p1 = day_three::solution::solve_p1();
    println!("Part 1: {:?}", p1);
}

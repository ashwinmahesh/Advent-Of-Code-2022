pub mod day_one;
pub mod day_two;

fn main() {
    // let max = day_one::solution::solve_p1();
    // println!("Max 1: {max}");

    // let top_three = day_one::solution::solve_p2();
    // println!("Final Top Three: {:?}", top_three);

    let score1 = day_two::solution::solve_p1();
    println!("Score: {:?}", score1)
}

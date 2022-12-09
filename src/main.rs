pub mod day_one;

fn main() {
    let max = day_one::solution::solve_p1();
    println!("Max 1: {max}");

    let top_three = day_one::solution::solve_p2();
    println!("Final Top Three: {:?}", top_three);
}

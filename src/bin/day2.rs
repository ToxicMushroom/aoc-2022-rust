extern crate core;

use aoc::read;

pub fn main() {
    let content = read("day2");
    let (a1, a2) = content.lines().rfold((0, 0), |(acc1, acc2): (i32, i32), x: &str| {
        let (opponent, you) = x.split_once(" ").unwrap();
        let points = get_points_q1(opponent, you);
        let points2 = get_points_q2(opponent, you);
        (acc1 + points, acc2 + points2)
    });

    // print results
    println!("Follow the moves {a1:?}");
    println!("Follow the results {a2:?}");
}

fn get_points_q2(opponent: &str, your_result: &str) -> i32 {
    let opponent_cp = get_choice_points(opponent);
    return match your_result {
        "X" => { (opponent_cp - 1).rem_euclid(3) },
        "Y" => 3 + opponent_cp,
        "Z" => 6 + (opponent_cp + 1).rem_euclid(3),
        _ => panic!("wrong input, expected X, Y or Z"),
    } + 1
}

// r p s
// 1,2,3 what you played
// 0 3 6 loss, draw, won
fn get_points_q1(opponent: &str, you: &str) -> i32 {
    let your_cp = get_choice_points(you);
    let opponent_cp = get_choice_points(opponent);
    let res = ((your_cp - opponent_cp + 3).rem_euclid(3) + 1).rem_euclid(3);
    res * 3 + your_cp + 1
}

fn get_choice_points(you: &str) -> i32 {
    match you {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("wrong input, expected X, Y or Z"),
    }
}
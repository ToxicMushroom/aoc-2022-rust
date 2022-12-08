#![feature(slice_group_by)]

use aoc::read;

pub fn main() {
    let content = read("day1");
    let mut calories: Vec<u32> = content
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();
    calories.sort_by(|el1, el2| el2.cmp(el1)); // reverse sort

    // take top 3 and top 1
    let first_three: &[u32] = &calories[0..3];
    let a1 = first_three[0];
    let a2: u32 = first_three.iter().sum();

    // print results
    println!("Highest calorie elve: {a1:?}");
    println!("top 3 calorie elves combined: {a2:?}");
}

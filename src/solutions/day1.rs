use crate::solutions::utils::read;

pub fn run_day1() {
    let content = read("day1");
    let elves: Vec<&str> = content.split("\n\n").collect();
    let mut calories = elves.iter().map(|x| get_line_sum(x)).collect::<Vec<u32>>();
    calories.sort_by(|el1, el2| el2.cmp(el1)); // reverse sort

    // take top 3 and top 1
    let first_three = calories.into_iter().take(3).collect::<Vec<u32>>();
    let a1 = first_three.get(0).unwrap();
    let a2: u32 = first_three.iter().sum();

    // print results
    println!("Highest calorie elve: {a1:?}");
    println!("top 3 calorie elves combined: {a2:?}");
}

fn get_line_sum(x: &str) -> u32 {
    x.lines().map(|x| x.parse::<u32>().unwrap()).sum()
}


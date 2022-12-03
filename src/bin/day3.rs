extern crate core;

fn main() {
    let day3 = aoc::read("day3");
    let (priorities1, priorities2): (i32, i32) = day3
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .fold((0, 0), |(acc1, acc2): (i32, i32), chunk: &[&str]| {
            let (b1, b2, b3) = (chunk[0], chunk[1], chunk[2]);
            let mut group_item_prio = 0;
            for l in b1.chars() {
                if b2.contains(l) && b3.contains(l) {
                    group_item_prio += char_to_priority(l);
                    break;
                }
            }
            let combined_prios_p1 = get_line_prio(b1) + get_line_prio(b2) + get_line_prio(b3);
            (acc1 + combined_prios_p1, acc2 + group_item_prio)
        });
    println!("{:?}", priorities1);
    println!("{:?}", priorities2)
}

fn get_line_prio(line: &str) -> i32 {
    let (first_half, second_half) = line.split_at(line.len() / 2);
    let mut prio_sum = 0;
    for l in first_half.chars() {
        if second_half.contains(l) {
            prio_sum += char_to_priority(l);
            break;
        }
    }
    prio_sum
}

fn char_to_priority(char: char) -> i32 {
    (match char {
        'a'..='z' => char as u32 - 96,
        'A'..='Z' => char as u32 - 65 + 27,
        _ => panic!("5")
    }) as i32
}
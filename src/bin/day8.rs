use std::cmp::max;

fn main() {
    let content = aoc::read("day8");
    let height = content.lines().count();
    let mut numbers = parse(content, height);
    let width = numbers.len() / height;

    // part 1
    for y in 1..(height - 1) {
        // run left -> right, run right -> left
        let (mut left_maxheight, _lsv, _) = numbers.get(y * width).unwrap();
        mark_visible_horizontal(&mut numbers, width, y, left_maxheight, 1..(width - 1));

        let (mut right_maxheight, _lsv, _) = numbers.get((y + 1) * width - 1).unwrap();
        mark_visible_horizontal(&mut numbers, width, y, right_maxheight, (1..(width - 1)).rev());
    }

    for x in 1..(width - 1) {
        // run top -> bottom
        let (mut top_maxheight, _lsv, _) = numbers.get(x).unwrap();
        mark_visible_vertical(&mut numbers, width, x, top_maxheight, 1..(height - 1));
        // run bottom -> top
        let (mut bottom_maxheight, _lsv, _) = numbers.get((height - 1) * width + x).unwrap();
        mark_visible_vertical(&mut numbers, width, x, bottom_maxheight, (1..(height - 1)).rev());
    }

    let a1 = numbers.iter().filter(|(_, b, _)| !*b).count();

    // part 2
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let (h, v, _) = numbers.get(index).unwrap().clone();
            let stuff = [width, x, y, h as usize];
            numbers[index] = (h, v,
                              run(&mut numbers, stuff, (0..x).rev(), false) *
                                  run(&mut numbers, stuff, x + 1..width, false) *
                                  run(&mut numbers, stuff, (0..y).rev(), true) *
                                  run(&mut numbers, stuff, y + 1..height, true)
            );
        }
    }

    let (_, _, a2) = numbers.iter().max_by_key(|(_, _, a)| a).unwrap();

    println!("{}", a1);
    println!("{}", a2);
}

fn run<I>(numbers: &mut Vec<(u32, bool, u32)>, stuff: [usize; 4], range: I, mod_y: bool) -> u32 where I: Iterator<Item=usize> {
    let [width, x, y, h] = stuff;
    let mut count = 0;
    for l in range {
        let index = if mod_y { (l) * width + x } else { y * width + l };
        count += 1;
        let (th, _, _) = numbers.get(index).unwrap().clone();
        if th >= h as u32 {
            break;
        }
    }
    count
}

fn mark_visible_vertical<I>(numbers: &mut Vec<(u32, bool, u32)>, width: usize, x: usize, mut max_state: u32, range: I) where I: Iterator<Item=usize> {
    for y in range { max_state = progress(numbers, width, x, max_state, y); }
}

fn mark_visible_horizontal<I>(numbers: &mut Vec<(u32, bool, u32)>, width: usize, y: usize, mut max_state: u32, range: I) where I: Iterator<Item=usize> {
    for x in range { max_state = progress(numbers, width, x, max_state, y); }
}

fn progress(numbers: &mut Vec<(u32, bool, u32)>, width: usize, x: usize, max_state: u32, y: usize) -> u32 {
    let index = y * width + x;
    let (ch, _cv, vc) = numbers.get(index).unwrap().clone();
    if max_state < ch { numbers[index] = (ch, false, vc); }
    max(max_state, ch)
}

fn parse(content: String, height: usize) -> Vec<(u32, bool, u32)> {
    content.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(|(x, t)| {
                let state = y != 0 && y != height - 1 && x != 0 && x != (line.len() - 1);
                (t.to_digit(10).unwrap(), state, 0)
            }).collect::<Vec<(u32, bool, u32)>>()
        }).collect::<Vec<(u32, bool, u32)>>()
}
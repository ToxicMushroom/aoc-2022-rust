use std::collections::HashMap;
use std::fmt::format;
use regex::Regex;

fn main() {
    let mut map: HashMap<i32, Vec<String>> = HashMap::new();
    let string1 = aoc::read("day5");
    let (boat, movements) = string1.split_once("\n\n").unwrap();
    let (boat, _stacks) = boat.rsplit_once("\n").unwrap();
    let mut cargo = boat.lines().collect::<Vec<&str>>();
    cargo.reverse();
    for line in cargo {
        let line_vec = line.chars().collect::<Vec<char>>();
        let containers = line_vec.chunks(4);
        for (index, container) in containers.enumerate() {
            let boxed_container: String = container.iter().collect();
            let trimmed = boxed_container.trim();
            let trimmed = trimmed.trim_start_matches("[").trim_end_matches("]");
            if trimmed != "" {
                let i = (index + 1) as i32;
                let vec: Vec<String> = Vec::new();
                let mut prev_stack = map.get(&i).unwrap_or(&vec).clone();
                prev_stack.push(trimmed.into());
                map.insert(i.clone(), prev_stack);
            }
        }
    }

    let mut map2 = map.clone();

    for movement in movements.lines() {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(movement).unwrap();

        let amount = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let from  = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let to  = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();

        let mut from_stack = map.get(&from).unwrap().clone();
        let mut from_stack2 = map2.get(&from).unwrap().clone();
        let mut to_stack = map.get(&to).unwrap().clone();
        let mut to_stack2 = map2.get(&to).unwrap().clone();
        println!("{:?}", from_stack2.len());
        println!("{amount:?}");
        from_stack2[from_stack2.len()-amount as usize..].iter().for_each(|s| to_stack2.push(s.clone()));

        for _i in 0..amount {
            let el = from_stack.pop().unwrap();
            to_stack.push(el);
            from_stack2.remove(from_stack2.len()-1);
        }
        map.insert(from, from_stack);
        map.insert(to, to_stack);

        map2.insert(from, from_stack2);
        map2.insert(to, to_stack2);
        println!("{map:?}");

    }
    let acc = acc_map(&mut map);
    println!("{acc:?}");
    let acc = acc_map(&mut map2);
    println!("{acc:?}");
}

fn acc_map(map: &mut HashMap<i32, Vec<String>>) -> String {
    let mut acc: String = String::new();
    for i in 1..=map.len() {
        let last = map[&(i as i32)].last().unwrap();
        acc = format!("{}{}", acc, last)
    }
    acc
}
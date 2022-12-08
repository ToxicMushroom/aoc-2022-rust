#![feature(map_many_mut)]

use std::collections::HashMap;

fn main() {
    let content = aoc::read("day7");
    let mut map: HashMap<usize, Vec<Dir>> = HashMap::new();

    let mut pwd: Vec<String> = Vec::new();
    parsing(content, &mut map, &mut pwd);

    let max = map.keys().max_by_key(|t| t.clone()).unwrap().clone();
    accumulate_sizes(&mut map, max);

    let a1: u32 = map.iter().map(|(_, v)| v.iter().map(|d| d.total_size).filter(|d| d <= &100_000u32).sum::<u32>()).sum();
    let full_usage: u32 = map.iter().map(|(_, v)| v.iter().map(|d| d.files.iter().sum::<u32>()).sum::<u32>()).sum();
    let available = 70_000_000 - full_usage;
    let missing = 30_000_000 - available;

    let a2 = map.iter().flat_map(|(_, dirs)| dirs).map(|d| d.total_size).filter(|t| t > &missing).min().unwrap();

    println!("{a1:?}");
    println!("{a2:?}");
}

fn accumulate_sizes(map: &mut HashMap<usize, Vec<Dir>>, max: usize) {
    for i in (0..=max).rev() {
        if i == max {
            let vec = map.get_mut(&max).unwrap();
            for dir in vec.iter_mut() {
                dir.total_size = dir.files.iter().sum::<u32>()
            }
        } else {
            let [vec, vec_next] = map.get_many_mut([&i, &(i + 1)]).unwrap();
            for dir in vec.iter_mut() {
                let mut vec1 = dir.pwd.clone();
                if dir.name != "" {
                    vec1.push((&dir.name).into());
                }
                dir.total_size = dir.files.iter().sum::<u32>() +
                    sum_sub_dirs(&vec1, &dir.directory_names, vec_next)
            }
        }
    }
}

fn sum_sub_dirs(pwd: &Vec<String>, names: &Vec<String>, opt: &mut Vec<Dir>) -> u32 {
    let sum = opt.iter().filter(|level_dir| {
        pwd == &level_dir.pwd && names.contains(&level_dir.name)
    }).map(|t| t.total_size).sum();
    return sum
}

fn parsing(content: String, map: &mut HashMap<usize, Vec<Dir>>, mut pwd: &mut Vec<String>) {
    let mut lines = content.lines();
    lines.next();
    for l in lines {
        let depth = pwd.len();
        if l.starts_with("$ ls") { // cool ?
            if map.get(&depth).is_none() { map.insert(depth, Vec::new()); }
            let mut current_vec = map.get_mut(&depth).unwrap();
            maybe_create_dir(&mut pwd, &mut current_vec);

        } else if l.starts_with("$ cd") { // change pwd
            let target = l.trim_start_matches("$ cd ");
            if target == ".." { // navigate up
                pwd.pop();
            } else { // navigate into
                pwd.push(target.into());
            }

        } else if l.starts_with("dir") { // ls output dir
            let current_vec = map.get_mut(&depth).unwrap();
            if let Some(dir) = current_vec.into_iter().find(|dir| {
                dir.clone().full_name() == Dir::join_pwd(&pwd)
            }) {
                dir.directory_names.push(l.trim_start_matches("dir ").into());
            }

        } else { // ls output file
            let current_vec = map.get_mut(&depth).unwrap();
            if let Some(dir) = current_vec.into_iter().find(|dir| {
                dir.clone().full_name() == Dir::join_pwd(&pwd)
            }) {
                let (size, _) = l.split_once(" ").unwrap();
                let file_size = size.parse::<u32>().unwrap();
                dir.files.push(file_size);
            }
        }
    }
}

fn maybe_create_dir(pwd: &mut Vec<String>, current_vec: &mut &mut Vec<Dir>) {
    if !current_vec.iter().any(|i| i.full_name() == Dir::join_pwd(pwd)) {
        let mut pwd = pwd.clone();
        let name = pwd.pop().unwrap_or("".to_string());
        current_vec.push(Dir {
            name: String::from(name),
            pwd,
            directory_names: vec![],
            files: vec![],
            total_size: 0,
        });
    }
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    pwd: Vec<String>,
    directory_names: Vec<String>,
    files: Vec<u32>,
    total_size: u32,
}

impl Dir {
    fn full_name(&self) -> String {
        let mut t = self.pwd.clone();
        t.push(self.name.clone());
        Dir::join_pwd(&t)
    }
    pub fn join_pwd(pwd: &Vec<String>) -> String {
        pwd.join("/") + "/"
    }
}
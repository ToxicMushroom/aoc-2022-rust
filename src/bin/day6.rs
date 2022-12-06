fn main() {
    let content = aoc::read("day6");
    let a1 = find_disjunct_sequence(&content, 4);
    let a2 = find_disjunct_sequence(&content, 14);
    println!("{:?}", a1);
    println!("{:?}", a2);
}

fn find_disjunct_sequence(content: &String, required_len: usize) -> usize {
    let mut i1 = 0;
    let mut active: Vec<char> = Vec::new();
    for (i, c) in content.char_indices() {
        active.insert(0, c);
        if active.len() < required_len { continue; }
        if active.len() > required_len { active.pop(); }
        if active.iter().all(|a| active.iter().filter(|b| a == *b).count() == 1) {
            i1 = i + 1;
            break;
        }
    }
    return i1
}
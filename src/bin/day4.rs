use std::ops::{Range};
use aoc::read;

fn main() {
    let content = read("day4");
    let (a1, a2) = content.lines().rfold((0, 0), |(mut acc1, mut acc2), line| {
        let (elve1, elve2) = line.split_once(',').unwrap();
        let range1 = parse_range(elve1);
        let range2 = parse_range(elve2);
        if range1.envelops_or_enveloped(&range2) { acc1 += 1 };
        if range1.intersects_range(&range2) { acc2 += 1; };
        (acc1, acc2)
    });
    println!("{}", a1);
    println!("{}", a2);
}

trait Comparing<R> {
    fn envelops_or_enveloped(&self, range: &Range<R>) -> bool;
    fn intersects_range(&self, range: &Range<R>) -> bool;
}

impl Comparing<i32> for Range<i32> {
    fn envelops_or_enveloped(&self, range: &Range<i32>) -> bool {
        (self.contains(&range.start) && self.contains(&(&range.end - 1))) ||
            (range.contains(&self.start) && range.contains(&(&self.end - 1)))
    }

    fn intersects_range(&self, range: &Range<i32>) -> bool {
        self.contains(&(&range.end - 1)) || self.contains(&range.start) ||
            range.contains(&self.start) || range.contains(&(&self.end - 1))
    }
}

fn parse_range(str: &str) -> Range<i32> {
    let (start, end) = str.split_once('-').unwrap();
    start.parse::<i32>().unwrap()..(end.parse::<i32>().unwrap() + 1)
}
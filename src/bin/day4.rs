use aoc::read;

fn main() {
    let content = read("day4");
    let (a1, a2) = content.lines().rfold((0, 0), |(mut acc1, mut acc2), line| {
        let (elve1, elve2) = line.split_once(',').unwrap();
        let (range1, range2): (IntRange, IntRange) = (elve1.into(), elve2.into());
        if range1.sub_or_superset(&range2) { acc1 += 1 };
        if range1.intersects_range(&range2) { acc2 += 1; };
        (acc1, acc2)
    });
    println!("{}", a1);
    println!("{}", a2);
}

/// Inclusive sane int-range implementation because rust's sucks
struct IntRange {
    pub start: i32,
    pub end: i32,
}

impl IntRange {
    fn contains(&self, i: &i32) -> bool { &self.start <= i && i <= &self.end }
    fn contains_range(&self, range: &Self) -> bool { self.contains(&range.start) && self.contains(&range.end) }

    fn sub_or_superset(&self, range: &IntRange) -> bool {
        self.contains_range(range) || range.contains_range(self)
    }

    fn intersects_range(&self, range: &IntRange) -> bool {
        self.contains(&range.end) || self.contains(&range.start) ||
            range.contains(&self.start) || range.contains(&self.end)
    }
}

impl<S: AsRef<str>> From<S> for IntRange {
    fn from(s: S) -> Self {
        let (start, end) = s.as_ref().split_once('-').unwrap();
        IntRange { start: start.parse::<i32>().unwrap(), end: (end.parse::<i32>().unwrap()) }
    }
}
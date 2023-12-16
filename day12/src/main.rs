const INPUT: &'static str = include_str!("./input");

use std::collections::{HashMap, VecDeque};

fn main() {
    part1();
    part2();
}

fn part1() {
    let rows: Vec<SpringRow> = INPUT.lines().map(|l| parse(l)).collect();

    let mut mem = HashMap::new();

    let sum: usize = rows.iter().map(|r| r.count_arrangements(&mut mem)).sum();
    println!("Part 1: {}", sum);
}

fn part2() {
    let rows: Vec<SpringRow> = INPUT.lines().map(|l| parse(l)).collect();

    let mut mem = HashMap::new();

    let sum: usize = rows
        .iter()
        .map(|r| r.unfold().count_arrangements(&mut mem))
        .sum();

    println!("Part 2: {}", sum);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpringCondition {
    Damaged,
    Operational,
    Unknown,
}

impl From<char> for SpringCondition {
    fn from(value: char) -> Self {
        match value {
            '#' => SpringCondition::Damaged,
            '.' => SpringCondition::Operational,
            '?' => SpringCondition::Unknown,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SpringGroup {
    length: usize,
}

impl From<&str> for SpringGroup {
    fn from(value: &str) -> Self {
        SpringGroup {
            length: usize::from_str_radix(value, 10).unwrap(),
        }
    }
}

fn parse(line: &str) -> SpringRow {
    let (conditions, groups) = line.split_once(" ").unwrap();
    let conditions = conditions.chars().map(|c| c.into()).collect();

    let groups = groups.split(",").map(|g| g.into()).collect();

    SpringRow { conditions, groups }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SpringRow {
    conditions: VecDeque<SpringCondition>,
    groups: VecDeque<SpringGroup>,
}

impl SpringRow {
    fn unfold(&self) -> SpringRow {
        let mut unfolded = self.clone();

        for _ in 1..5 {
            unfolded.conditions.push_back(SpringCondition::Unknown);

            for &c in &self.conditions {
                unfolded.conditions.push_back(c);
            }

            for &g in &self.groups {
                unfolded.groups.push_back(g);
            }
        }

        unfolded
    }

    fn count_arrangements(&self, memoized: &mut HashMap<SpringRow, usize>) -> usize {
        if let Some(count) = memoized.get(self) {
            return *count;
        }

        if self.complete() {
            memoized.insert(self.clone(), 1);
            return 1;
        }

        let mut next_pattern_count: Vec<(SpringRow, usize)> = vec![];

        'outer: for next in self.next_possible_rows() {
            for (other, count) in next_pattern_count.iter_mut() {
                if next == *other {
                    *count += 1;
                    continue 'outer;
                }
            }

            next_pattern_count.push((next, 1));
        }

        let mut total = 0;

        for (next, mul) in next_pattern_count.iter() {
            total += mul * next.count_arrangements(memoized);
        }

        memoized.insert(self.clone(), total);
        total
    }

    fn complete(&self) -> bool {
        !self
            .conditions
            .iter()
            .any(|&c| c == SpringCondition::Damaged)
            && self.groups.is_empty()
    }

    fn next_possible_rows(&self) -> Vec<SpringRow> {
        if self.groups.is_empty() {
            return vec![];
        }

        let row_length: usize =
            self.groups.iter().map(|g| g.length).sum::<usize>() + self.groups.len() - 1;

        if self.conditions.len() < row_length {
            return vec![];
        }

        let mut rows = vec![];

        'outer: for start in 0..=self.conditions.len() - row_length {
            let mut row = self.clone();
            let length = row.groups.pop_front().unwrap().length;

            for _ in 0..start {
                let condition = row.conditions.pop_front().unwrap();
                if condition == SpringCondition::Damaged {
                    continue 'outer;
                }
            }

            for _ in 0..length {
                let condition = row.conditions.pop_front().unwrap();
                if condition == SpringCondition::Operational {
                    continue 'outer;
                }
            }

            if let Some(next) = row.conditions.pop_front() {
                if next == SpringCondition::Damaged {
                    continue 'outer;
                }
            }

            let mut finished_triming = false;
            while !finished_triming {
                if let Some(next) = row.conditions.pop_front() {
                    if next != SpringCondition::Operational {
                        finished_triming = true;
                        row.conditions.push_front(next);
                    }
                } else {
                    finished_triming = true;
                }
            }

            rows.push(row)
        }

        rows
    }
}

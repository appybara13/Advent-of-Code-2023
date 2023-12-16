const INPUT: &'static str = include_str!("./input");

use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    let patterns = parse();

    let summary: usize = patterns.iter().map(|p| p.summary(false)).sum();

    println!("Part 1: {}", summary);
}

fn part2() {
    let patterns = parse();

    let summary: usize = patterns.iter().map(|p| p.summary(true)).sum();

    println!("Part 2: {}", summary);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn reflect_x(&self, width: usize, x: usize) -> Option<Pos> {
        if self.x > x {
            let dist = self.x - 1 - x;

            if dist > x {
                None
            } else {
                Some(Pos {
                    x: x - dist,
                    y: self.y,
                })
            }
        } else {
            let dist = x - self.x;

            if dist + 1 >= width - x {
                None
            } else {
                Some(Pos {
                    x: x + dist + 1,
                    y: self.y,
                })
            }
        }
    }

    fn reflect_y(&self, height: usize, y: usize) -> Option<Pos> {
        if self.y > y {
            let dist = self.y - 1 - y;

            if dist > y {
                None
            } else {
                Some(Pos {
                    x: self.x,
                    y: y - dist,
                })
            }
        } else {
            let dist = y - self.y;

            if dist + 1 >= height - y {
                None
            } else {
                Some(Pos {
                    x: self.x,
                    y: y + dist + 1,
                })
            }
        }
    }
}

#[derive(Debug)]
struct Pattern {
    width: usize,
    height: usize,
    rock: HashSet<Pos>,
}

impl Pattern {
    fn summary(&self, smudged: bool) -> usize {
        if let Some(x) = self.find_x_reflection(smudged) {
            x + 1
        } else if let Some(y) = self.find_y_reflection(smudged) {
            (y + 1) * 100
        } else {
            panic!()
        }
    }

    fn find_x_reflection(&self, smudged: bool) -> Option<usize> {
        for x in 0..self.width - 1 {
            if self.valid_reflection_x(x, smudged) {
                return Some(x);
            }
        }

        None
    }

    fn find_y_reflection(&self, smudged: bool) -> Option<usize> {
        for y in 0..self.height - 1 {
            if self.valid_reflection_y(y, smudged) {
                return Some(y);
            }
        }

        None
    }

    fn valid_reflection_x(&self, x: usize, smudged: bool) -> bool {
        let reflected: HashSet<_> = self
            .rock
            .iter()
            .filter_map(|r| r.reflect_x(self.width, x))
            .collect();

        let difference = reflected.difference(&self.rock);

        if !smudged {
            difference.count() == 0
        } else {
            difference.count() == 1
        }
    }

    fn valid_reflection_y(&self, y: usize, smudged: bool) -> bool {
        let reflected: HashSet<_> = self
            .rock
            .iter()
            .filter_map(|r| r.reflect_y(self.height, y))
            .collect();

        let difference = reflected.difference(&self.rock);

        if !smudged {
            difference.count() == 0
        } else {
            difference.count() == 1
        }
    }
}

fn parse() -> Vec<Pattern> {
    let mut patterns = vec![];

    let mut width = 0;
    let mut rock = HashSet::new();
    let mut height = 0;

    for line in INPUT.lines() {
        if line.is_empty() {
            patterns.push(Pattern {
                width,
                height,
                rock,
            });
            width = 0;
            rock = HashSet::new();
            height = 0;
            continue;
        }

        width = 0;
        for c in line.chars() {
            if c == '#' {
                rock.insert(Pos {
                    x: width,
                    y: height,
                });
            }

            width += 1;
        }

        height += 1;
    }

    patterns.push(Pattern {
        width,
        height,
        rock,
    });

    patterns
}

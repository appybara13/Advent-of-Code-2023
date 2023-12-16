const INPUT: &'static str = include_str!("./input");

use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    let image = Image::from(INPUT, 2);

    let sum = image.sum_all_distances();

    println!("Part 1: {}", sum);
}

fn part2() {
    let image = Image::from(INPUT, 1000000);

    let sum = image.sum_all_distances();

    println!("Part 2: {}", sum);
}

#[derive(Debug)]
struct Image {
    galaxies: Vec<(usize, usize)>,
}

impl Image {
    fn distance(&self, ga: usize, gb: usize) -> usize {
        let ga = self.galaxies[ga];
        let gb = self.galaxies[gb];

        let dx = ga.0.abs_diff(gb.0);
        let dy = ga.1.abs_diff(gb.1);

        dx + dy
    }

    fn sum_all_distances(&self) -> usize {
        let mut total = 0;

        for ga in 0..self.galaxies.len() {
            for gb in ga + 1..self.galaxies.len() {
                total += self.distance(ga, gb);
            }
        }

        total
    }

    fn from(value: &str, age: usize) -> Self {
        let mut non_empty_i = HashSet::new();

        for line in value.lines() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    non_empty_i.insert(i);
                }
            }
        }

        let mut galaxies = vec![];

        let mut y = 1;
        for (j, line) in value.lines().enumerate() {
            let mut j_not_empty = false;

            let mut x = 1;
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((x, y));
                    j_not_empty = true;
                } else if !non_empty_i.contains(&i) {
                    x += age - 1;
                }

                x += 1;
            }

            if !j_not_empty {
                y += age - 1;
            }

            y += 1;
        }

        Image { galaxies }
    }
}

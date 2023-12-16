const INPUT: &'static str = include_str!("./input");

use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut platform: Platform = INPUT.into();
    platform.tilt((0, -1));

    println!("Part 1: {}", platform.load());
}

fn part2() {
    let mut platform: Platform = INPUT.into();

    let mut cycles_remaining = 1000000000;

    let mut previous_cycles = HashMap::new();

    let mut period_found = false;
    previous_cycles.insert(platform.round_positions(), cycles_remaining);

    while cycles_remaining > 0 {
        platform.tilt((0, -1));
        platform.tilt((-1, 0));
        platform.tilt((0, 1));
        platform.tilt((1, 0));
        cycles_remaining -= 1;

        if !period_found {
            if let Some(previous_cycles_remaining) = previous_cycles.get(&platform.round_positions()) {
                let period = previous_cycles_remaining - cycles_remaining;
    
                while cycles_remaining > period {
                    cycles_remaining -= period;
                }

                period_found = true;
            }

            previous_cycles.insert(platform.round_positions(), cycles_remaining);
        }
    }

    println!("Part 2: {}", platform.load());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
    None
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            '.' => Rock::None,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Platform {
    width: isize,
    height: isize,
    rocks: HashMap<(isize, isize), Rock>
}

impl Platform {
    fn load(&self) -> usize {
        let mut total = 0;

        for x in 1..=self.width {
            for y in 1..=self.height {
                if self.rocks[&(x, y)] == Rock::Round {
                    total += self.height + 1 - y;
                }
            }
        }

        total as usize
    }

    fn round_positions(&self) -> Vec<(isize, isize)> {
        let mut positions = vec![];

        for x in 1..=self.width {
            for y in 1..=self.height {
                if self.rocks[&(x, y)] == Rock::Round {
                    positions.push((x, y))
                }
            }
        }

        positions
    }

    fn tilt(&mut self, direction: (isize, isize)) {
        let start = match direction {
            (0, -1) => (1, 1),
            (0, 1) => (1, self.height),
            (-1, 0) => (1, 1),
            (1, 0) => (self.width, 1),
            _ => panic!()
        };

        let direction_length = match direction {
            (0, -1) => self.height,
            (0, 1) => self.height,
            (-1, 0) => self.width,
            (1, 0) => self.width,
            _ => panic!()
        };

        let perpendicular = match direction {
            (0, -1) => (1, 0),
            (0, 1) => (1, 0),
            (-1, 0) => (0, 1),
            (1, 0) => (0, 1),
            _ => panic!()
        };

        let perpendicular_length = match direction {
            (0, -1) => self.width,
            (0, 1) => self.width,
            (-1, 0) => self.height,
            (1, 0) => self.height,
            _ => panic!()
        };

        for d_n in 0..direction_length {
            for p_n in 0..perpendicular_length {
                let mut pos = start;
                pos.0 -= direction.0 * d_n;
                pos.1 -= direction.1 * d_n;
                pos.0 += perpendicular.0 * p_n;
                pos.1 += perpendicular.1 * p_n;

                if self.rocks[&pos] != Rock::Round {
                    continue;
                }

                let mut scan = pos;
                while self.rocks[&(scan.0 + direction.0, scan.1 + direction.1)] == Rock::None {
                    scan.0 += direction.0;
                    scan.1 += direction.1;
                }

                if pos != scan {
                    self.rocks.insert(pos, Rock::None);
                    self.rocks.insert(scan, Rock::Round);
                }
            }
        }
    }
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len() as isize;
        let height = value.lines().count() as isize;

        let mut rocks = HashMap::new();

        for x in 0..=width + 1 {
            rocks.insert((x, 0), Rock::Cube);
            rocks.insert((x, height + 1), Rock::Cube);
        }

        let mut y = 1;
        for line in value.lines() {
            rocks.insert((0, y), Rock::Cube);
            rocks.insert((width + 1, y), Rock::Cube);

            let mut x = 1;
            for c in line.chars() {
                rocks.insert((x, y), c.into());

                x += 1;
            }

            y += 1;
        }

        Platform { width, height, rocks }
    }
}

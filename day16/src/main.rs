const INPUT: &'static str = include_str!("./input");

use std::collections::{HashMap, HashSet};

fn main() {
    part1();
    part2();
}

fn part1() {
    let contraption: Contraption = INPUT.into();

    let count = contraption.energized_from(BeamState { pos: (1, 1), dir: Direction::Right });

    println!("Part 1: {}", count);
}

fn part2() {
    let contraption: Contraption = INPUT.into();

    let mut max = 0;

    for x in 1..=contraption.width {
        let down = contraption.energized_from(BeamState{pos: (x, 1), dir: Direction::Down});
        let up = contraption.energized_from(BeamState{pos: (x, contraption.height), dir: Direction::Up});
    
        max = usize::max(max, down);
        max = usize::max(max, up);
    }

    for y in 1..=contraption.height {
        let right = contraption.energized_from(BeamState{pos: (1, y), dir: Direction::Right});
        let left = contraption.energized_from(BeamState{pos: (contraption.width, y), dir: Direction::Left});
    
        max = usize::max(max, right);
        max = usize::max(max, left);
    }

    println!("Part 2: {}", max);
}

#[derive(Debug)]
struct  Contraption {
    width: isize,
    height: isize,
    tiles: HashMap<(isize, isize), Tile>
}

impl Contraption {
    fn energized_from(&self, start: BeamState) -> usize {
        let mut all_next_states = vec![start];

        let mut visited = HashSet::new();
        let mut energized = HashSet::new();

        while let Some(state) = all_next_states.pop() {
            if visited.contains(&state) {
                continue;
            }

            if self.tiles[&state.pos] != Tile::Null {
                energized.insert(state.pos);
            }
            visited.insert(state);

            let mut next_states = state.next(self.tiles[&state.pos]);

            all_next_states.append(&mut next_states);
        }

        energized.len()
    }
}

impl From<&str> for Contraption {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len() as isize;
        let height = value.lines().count() as isize;

        let mut tiles = HashMap::new();

        for x in 0..=width + 1 {
            tiles.insert((x, 0), Tile::Null);
            tiles.insert((x, height + 1), Tile::Null);
        }

        let mut y = 1;
        for line in value.lines() {
            tiles.insert((0, y), Tile::Null);
            tiles.insert((width + 1, y), Tile::Null);

            let mut x = 1;
            for c in line.chars() {
                tiles.insert((x, y), c.into());

                x += 1;
            }

            y += 1;
        }

        Contraption { tiles, width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Null,
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BeamState {
    pos: (isize, isize),
    dir: Direction,
}

impl BeamState {
    fn next(&self, tile: Tile) -> Vec<BeamState> {
        match tile {
            Tile::Null => vec![],
            Tile::Empty => match self.dir {
                Direction::Up => vec![BeamState {
                    pos: (self.pos.0, self.pos.1 - 1),
                    dir: self.dir,
                }],
                Direction::Down => vec![BeamState {
                    pos: (self.pos.0, self.pos.1 + 1),
                    dir: self.dir,
                }],
                Direction::Left => vec![BeamState {
                    pos: (self.pos.0 - 1, self.pos.1),
                    dir: self.dir,
                }],
                Direction::Right => vec![BeamState {
                    pos: (self.pos.0 + 1, self.pos.1),
                    dir: self.dir,
                }],
            },
            Tile::ForwardMirror => match self.dir {
                Direction::Up => vec![BeamState {
                    pos: (self.pos.0 + 1, self.pos.1),
                    dir: Direction::Right,
                }],
                Direction::Down => vec![BeamState {
                    pos: (self.pos.0 - 1, self.pos.1),
                    dir: Direction::Left,
                }],
                Direction::Left => vec![BeamState {
                    pos: (self.pos.0, self.pos.1 + 1),
                    dir: Direction::Down,
                }],
                Direction::Right => vec![BeamState {
                    pos: (self.pos.0, self.pos.1 - 1),
                    dir: Direction::Up,
                }],
            },
            Tile::BackwardMirror => match self.dir {
                Direction::Up => vec![BeamState {
                    pos: (self.pos.0 - 1, self.pos.1),
                    dir: Direction::Left,
                }],
                Direction::Down => vec![BeamState {
                    pos: (self.pos.0 + 1, self.pos.1),
                    dir: Direction::Right,
                }],
                Direction::Left => vec![BeamState {
                    pos: (self.pos.0, self.pos.1 - 1),
                    dir: Direction::Up,
                }],
                Direction::Right => vec![BeamState {
                    pos: (self.pos.0, self.pos.1 + 1),
                    dir: Direction::Down,
                }],
            },
            Tile::VerticalSplitter => match self.dir {
                Direction::Up => vec![BeamState {
                    pos: (self.pos.0, self.pos.1 - 1),
                    dir: self.dir,
                }],
                Direction::Down => vec![BeamState {
                    pos: (self.pos.0, self.pos.1 + 1),
                    dir: self.dir,
                }],
                Direction::Left => vec![
                    BeamState {
                        pos: (self.pos.0, self.pos.1 - 1),
                        dir: Direction::Up,
                    },
                    BeamState {
                        pos: (self.pos.0, self.pos.1 + 1),
                        dir: Direction::Down,
                    },
                ],
                Direction::Right => vec![
                    BeamState {
                        pos: (self.pos.0, self.pos.1 - 1),
                        dir: Direction::Up,
                    },
                    BeamState {
                        pos: (self.pos.0, self.pos.1 + 1),
                        dir: Direction::Down,
                    },
                ],
            },
            Tile::HorizontalSplitter => match self.dir {
                Direction::Up => vec![
                    BeamState {
                        pos: (self.pos.0 - 1, self.pos.1),
                        dir: Direction::Left,
                    },
                    BeamState {
                        pos: (self.pos.0 + 1, self.pos.1),
                        dir: Direction::Right,
                    },
                ],
                Direction::Down => vec![
                    BeamState {
                        pos: (self.pos.0 - 1, self.pos.1),
                        dir: Direction::Left,
                    },
                    BeamState {
                        pos: (self.pos.0 + 1, self.pos.1),
                        dir: Direction::Right,
                    },
                ],
                Direction::Left => vec![BeamState {
                    pos: (self.pos.0 - 1, self.pos.1),
                    dir: self.dir,
                }],
                Direction::Right => vec![BeamState {
                    pos: (self.pos.0 + 1, self.pos.1),
                    dir: self.dir,
                }],
            },
        }
    }
}

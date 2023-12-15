const INPUT: &'static str = include_str!("./input");

use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    a: char,
    b: char,
    c: char,
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        Coord {
            a: chars.next().unwrap(),
            b: chars.next().unwrap(),
            c: chars.next().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    pos: Coord,
    left: Coord,
    right: Coord,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let (pos, next) = value.split_once(" = ").unwrap();
        let (left, right) = next
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(", ")
            .unwrap();

        Node {
            pos: pos.into(),
            left: left.into(),
            right: right.into(),
        }
    }
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<Coord, Node>,
}

impl Network {
    fn step(&self, start: Coord, direction: Direction) -> Coord {
        match direction {
            Direction::Left => self.nodes[&start].left,
            Direction::Right => self.nodes[&start].right,
        }
    }

    fn count_steps(
        &self,
        directions: &Vec<Direction>,
        direction_i: usize,
        start: Coord,
        end: Coord,
    ) -> usize {
        if start == end {
            return 0;
        }

        return 1 + self.count_steps(
            directions,
            (direction_i + 1) % directions.len(),
            self.step(start, directions[direction_i]),
            end,
        );
    }

    // fn ghost_step(
    //     &self,
    //     directions: &Vec<Direction>,
    //     step_i: usize,
    //     starts: Vec<Coord>,
    //     end: char,
    // ) -> usize {
    //     if starts.iter().all(|s| s.c == end) {
    //         return step_i;
    //     }

    //     return self.ghost_step(
    //         directions,
    //         step_i + 1,
    //         starts.into_iter().map(|s| self.step(s, directions[step_i % directions.len()])).collect(),
    //         end,
    //     );
    // }

    fn period(&self, directions: &Vec<Direction>, mut coord: Coord, end: char) -> RepeatedEvent {
        let mut history: HashMap<(Coord, usize), usize> = HashMap::new();
        let mut ends = vec![];

        let mut i = 0;

        while true {
            let dir_i = i % directions.len();

            if let Some(&last_time) = history.get(&(coord, dir_i)) {
                return RepeatedEvent {
                    loop_offset: last_time,
                    loop_length: i - last_time,
                    // event_indices: ends,
                    // event_index: ends[0]
                };
            }

            if coord.c == end {
                ends.push(i);
            }

            history.insert((coord, dir_i), i);
            coord = self.step(coord, directions[dir_i]);
            i += 1;
        }

        panic!()

        // 1 + self.count_steps(directions, 1, self.step(coord, directions[0]), coord)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }
}

fn parse() -> (Vec<Direction>, Network) {
    let mut lines = INPUT.lines();

    let directions = lines.next().unwrap().chars().map(|c| c.into()).collect();

    let nodes = lines
        .skip(1)
        .map(|l| l.into())
        .map(|n: Node| (n.pos, n))
        .collect();

    (directions, Network { nodes })
}

#[derive(Debug, Clone, Copy)]
struct RepeatedEvent {
    loop_offset: usize,
    loop_length: usize,
    // puzzle input keeps this simple (only one event index, and it happens on the loop_length)
    // event_indices: Vec<usize>
}

impl RepeatedEvent {
    fn coincide(&self, other: RepeatedEvent) -> RepeatedEvent {
        let coincide_offset = num::integer::lcm(self.loop_offset, other.loop_offset);

        let coincide_length = num::integer::lcm(self.loop_length, other.loop_length);

        RepeatedEvent {
            loop_offset: coincide_offset,
            loop_length: coincide_length,
        }
    }
}

fn part1() {
    let (directions, network) = parse();

    let steps = network.count_steps(
        &directions,
        0,
        Coord {
            a: 'A',
            b: 'A',
            c: 'A',
        },
        Coord {
            a: 'Z',
            b: 'Z',
            c: 'Z',
        },
    );

    println!("Part 1: {}", steps);
}

fn part2() {
    let (directions, network) = parse();

    let starts: Vec<_> = network
        .nodes
        .keys()
        .filter(|&n| n.c == 'A')
        .cloned()
        .collect();

    let periods: Vec<_> = starts
        .iter()
        .map(|&s| network.period(&directions, s, 'Z'))
        .collect();

    let mut coincide = periods[0];

    for i in 1..periods.len() {
        coincide = coincide.coincide(periods[i]);
    }

    println!("Part 2: {}", coincide.loop_length);
}

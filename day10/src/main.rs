const INPUT: &'static str = include_str!("./input");

use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let map: Map = INPUT.into();
    let (loop_start, loop_length) = map.find_loop();

    println!("Part 1: {}", loop_length / 2);
}

fn part2() {
    let map: Map = INPUT.into();
    
    let enclosed = map.count_enclosed();

    println!("Part 2: {}", enclosed);
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    connected: ((usize, usize), (usize, usize)),
}

impl From<(char, (usize, usize))> for Pipe {
    fn from(value: (char, (usize, usize))) -> Self {
        let pos = value.1;
        match value.0 {
            '|' => Pipe {connected: ((pos.0, pos.1 - 1), (pos.0, pos.1 + 1))},
            '-' => Pipe {connected: ((pos.0 - 1, pos.1), (pos.0 + 1, pos.1))},
            'L' => Pipe {connected: ((pos.0, pos.1 + 1), (pos.0 + 1, pos.1))},
            'J' => Pipe {connected: ((pos.0, pos.1 + 1), (pos.0 - 1, pos.1))},
            '7' => Pipe {connected: ((pos.0, pos.1 - 1), (pos.0 - 1, pos.1))},
            'F' => Pipe {connected: ((pos.0, pos.1 - 1), (pos.0 + 1, pos.1))},
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct Map {
    start: (usize, usize),
    width: usize,
    height: usize,
    pipes: HashMap<(usize, usize), Option<Pipe>>
}

impl Map {
    fn move_along_loop(&self, mut next: (usize, usize), distance: usize) -> (usize, usize) {
        let mut pos = self.start;

        for _ in 0..distance {
            if let Some(next_pipe) = self.pipes[&next] {
                if next_pipe.connected.0 == pos {
                    pos = next;
                    next = next_pipe.connected.1;
                } else if next_pipe.connected.1 == pos {
                    pos = next;
                    next = next_pipe.connected.0;
                } else {
                    panic!()
                }
            }
        }

        pos
    }

    fn enclosed(a_point: (usize, usize), pipes: &Vec<(usize, usize)>) -> bool {
        let mut angle_total = 0;

        for i in 0..pipes.len() {
            let b_point = pipes[i];
            let c_point = if i + 1 < pipes.len() { pipes[i + 1] } else { pipes[0]};

            let delta = Self::quadrant(a_point, c_point) - Self::quadrant(a_point, b_point);

            angle_total += Self::adjust_delta(delta, b_point, c_point, a_point);
        }

        angle_total != 0
    }

    fn adjust_delta(delta: isize, from: (usize, usize), to: (usize, usize), point: (usize, usize))  -> isize {
        
        //
        let dx = to.0 as isize - from.0 as isize;
        let dy = to.1 as isize - from.1 as isize;

        let t = (point.1 as f64 - from.1 as f64) / dy as f64;
        let x = from.0 as f64 + t * dx as f64;
        
        match delta {
            3 => -1,
            -3 => 1,
            2 => if x > point.0 as f64 { -2 } else { 2 },
            -2 => if x > point.0 as f64 { 2 } else { -2 },
            _ => delta
        }
    }

    fn quadrant(from: (usize, usize), to: (usize, usize)) -> isize {
        match (to.0 > from.0, to.1 > from.1) {
            (true, true) => 0,
            (true, false) => 3,
            (false, true) => 1,
            (false, false) => 2,
        }
    }
 
    fn count_enclosed(&self) -> usize {

        let (loop_start, loop_length) = self.find_loop();

        let pipes: Vec<_> = self.all_loop_points(loop_start, loop_length).into_iter().collect();

        let mut total = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if !pipes.contains(&(x, y)) {
                    if Self::enclosed((x, y), &pipes) {
                        total += 1;
                    }
                }
            }
        }

        total
    }

    fn all_loop_points(&self, mut next: (usize, usize), length: usize) -> Vec<(usize, usize)> {
        let mut pos = self.start;

        let mut points = vec![];

        for _ in 0..length {
            if let Some(next_pipe) = self.pipes[&next] {
                points.push(next);
                if next_pipe.connected.0 == pos {
                    pos = next;
                    next = next_pipe.connected.1;
                } else if next_pipe.connected.1 == pos {
                    pos = next;
                    next = next_pipe.connected.0;
                } else {
                    panic!()
                }
            }
        }

        points.push(self.start);

        points
    }

    fn find_loop(&self) -> ((usize, usize), usize) {
        let possible_neighbours = vec![
            (self.start.0 - 1, self.start.1),
            (self.start.0 + 1, self.start.1),
            (self.start.0, self.start.1 - 1),
            (self.start.0, self.start.1 + 1)
        ];

        for possible_neighbour in possible_neighbours {
            let mut pos = self.start;
            let mut next = possible_neighbour;

            let mut length = 0;

            while let Some(next_pipe) = self.pipes[&next] {
                if next_pipe.connected.0 == pos {
                    pos = next;
                    next = next_pipe.connected.1;
                } else if next_pipe.connected.1 == pos {
                    pos = next;
                    next = next_pipe.connected.0;
                }
                 else {
                    pos = next;
                    next = (0, 0);
                }

                length += 1;
            }

            if next == self.start {
                return (possible_neighbour, length + 1);
            }
        }

        panic!()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().chars().count();
        let height = value.lines().count();
    
        let mut pipes = HashMap::new();
    
        let mut start = (0, 0);
    
        for x in 0..=width + 1 {
            pipes.insert((x, 0), None);
        }
    
        let mut y = height;
        for line in value.lines() {
            pipes.insert((0, y), None);
    
            let mut x = 1;
            for c in line.chars() {
    
                match c {
                    '.' => pipes.insert((x, y), None),
                    'S' => {
                        start = (x, y);
                        pipes.insert((x, y), None)
                    },
                    _ => pipes.insert((x, y), Some((c, (x, y)).into()))
                };
    
                x += 1;
            }
    
            pipes.insert((x, y), None);
    
            y -= 1;
        }
    
        for x in 0..=width + 1 {
            pipes.insert((x, y), None);
        }

        Map { start, width: width + 2, height: height + 2, pipes }
    }
}
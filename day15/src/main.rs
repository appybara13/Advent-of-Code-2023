use std::collections::HashMap;

const INPUT: &'static str = include_str!("./input");

fn main() {
    part1();
    part2();
}

fn part1() {
    let sum: usize = INPUT.split(",").map(|s| hash(s)).sum();

    println!("Part 1: {}", sum);
}

fn part2() {
    let mut boxes = Boxes {boxes: HashMap::new()};

    INPUT.split(",").map(|s| Op::from(s)).for_each(|op| boxes.perform_operation(op));

    let power = boxes.focusing_power();

    println!("Part 2: {}", power);
}

struct Boxes {
    boxes: HashMap<usize, BoxContent>
}

impl Boxes {
    fn perform_operation(&mut self, op: Op) {
        match op {
            Op::Remove { box_id, label } => self.remove(box_id, label),
            Op::Add { box_id, label, focal_length } => self.add(box_id, label, focal_length),
        }
    }

    fn add(&mut self, box_id: usize, label: String, focal_length: usize) {
        if !self.boxes.contains_key(&box_id) {
            self.boxes.insert(box_id, BoxContent { focal_lengths: HashMap::new(), positions: HashMap::new() });
        }

        self.boxes.get_mut(&box_id).unwrap().add(label, focal_length);
    }

    fn remove(&mut self, box_id: usize, label: String) {
        if let Some(content) = self.boxes.get_mut(&box_id) {
            content.remove(label);

            if content.is_empty() {
                self.boxes.remove(&box_id);
            }
        }
    }

    fn focusing_power(&self) -> usize {
        let mut total = 0;
        
        for (id, content) in &self.boxes {
            total += (id + 1) * content.focusing_power();
        } 

        return total;
    }
}

struct BoxContent {
    focal_lengths: HashMap<String, usize>,
    positions: HashMap<String, usize>
}

impl BoxContent {
    fn add(&mut self, label: String, focal_length: usize) {
        if let Some(fl) = self.focal_lengths.get_mut(&label) {
            *fl = focal_length;
        } else {
            self.focal_lengths.insert(label.clone(), focal_length);
            self.positions.insert(label, self.positions.len());
        }
    }

    fn remove(&mut self, label: String) {
        if let Some(position) = self.positions.remove(&label) {
            self.focal_lengths.remove(&label);

            for (_, other_position) in self.positions.iter_mut() {
                if *other_position > position {
                    *other_position -= 1;
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.focal_lengths.len() == 0
    }

    fn focusing_power(&self) -> usize {
        let mut total = 0;

        for label in self.focal_lengths.keys() {
            total += (self.positions[label] + 1) * self.focal_lengths[label];
        }

        return total;
    }
}

enum Op {
    Remove{box_id: usize, label: String},
    Add{box_id: usize, label: String, focal_length: usize},
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        if value.contains("-") {
            let (label, _) = value.split_once("-").unwrap();

            Op::Remove { box_id: hash(label), label: label.into() }
        }
        else if value.contains("=") {
            let (label, focal_length) = value.split_once("=").unwrap();

            Op::Add { box_id: hash(label), label: label.into(), focal_length: usize::from_str_radix(focal_length, 10).unwrap() }
        } else {
            panic!()
        }
    }
}

fn hash(input: &str) -> usize {
    let mut current_value = 0;

    for c in input.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

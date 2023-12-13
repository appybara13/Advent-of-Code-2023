use std::collections::HashSet;

const INPUT: &'static str = include_str!("./input");

#[derive(Debug, Clone, Copy)]
enum Value {
    Digit { value: usize, num_id: usize },
    Symbol(char),
    Empty,
}

#[derive(Debug)]
struct Schematic {
    width: usize,
    height: usize,
    numbers: Vec<usize>,
    values: Vec<Vec<Value>>,
}

fn parse() -> Schematic {
    let width = INPUT.lines().next().unwrap().chars().count();
    let height = INPUT.lines().count();
    let mut numbers = vec![];

    let values = INPUT
        .lines()
        .map(|l| {
            let mut current_number = None;

            let v = l.chars()
                .map(|c| {

                    if let Some(n) = c.to_digit(10) {
                        if let Some(cn) = current_number {
                            current_number = Some(cn * 10 + n as usize);
                        } else {
                            current_number = Some(n as usize);
                        }

                        Value::Digit {
                            value: n as usize,
                            num_id: numbers.len(),
                        }
                    } else if c == '.' {
                        if let Some(cn) = current_number {
                            numbers.push(cn);
                            current_number = None;
                        }

                        Value::Empty
                    } else {
                        if let Some(cn) = current_number {
                            numbers.push(cn);
                            current_number = None;
                        }

                        Value::Symbol(c)
                    }
                })
                .collect();

            if let Some(cn) = current_number {
                numbers.push(cn);
            }

            v
        })
        .collect();

    Schematic {
        width,
        height,
        numbers,
        values,
    }
}

fn part1() {
    let schematic = parse();

    let mut num_ids = HashSet::new();

    for i in 0..schematic.width {
        for j in 0..schematic.height {
            if let Value::Symbol(_) = schematic.values[i][j] {

                if i > 0 && j > 0 {
                    if let Value::Digit { num_id, .. } = schematic.values[i - 1][j - 1] {
                        num_ids.insert(num_id);
                    }
                }
                if i > 0 {
                    if let Value::Digit { num_id, .. } = schematic.values[i - 1][j] {
                        num_ids.insert(num_id);
                    }
                }
                if i > 0 && j < schematic.height - 1 {
                    if let Value::Digit { num_id, .. } = schematic.values[i - 1][j + 1] {
                        num_ids.insert(num_id);
                    }
                }

                if j > 0 {
                    if let Value::Digit { num_id, .. } = schematic.values[i][j - 1] {
                        num_ids.insert(num_id);
                    }
                }
                if j < schematic.height - 1 {
                    if let Value::Digit { num_id, .. } = schematic.values[i][j + 1] {
                        num_ids.insert(num_id);
                    }
                }

                if i < schematic.width - 1 && j > 0 {
                    if let Value::Digit { num_id, .. } = schematic.values[i + 1][j - 1] {
                        num_ids.insert(num_id);
                    }
                }
                if i < schematic.width - 1 {
                    if let Value::Digit { num_id, .. } = schematic.values[i + 1][j] {
                        num_ids.insert(num_id);
                    }
                }
                if i < schematic.width - 1 && j < schematic.height - 1 {
                    if let Value::Digit { num_id, .. } = schematic.values[i + 1][j + 1] {
                        num_ids.insert(num_id);
                    }
                }

            }
        }
    }

    let sum: usize = num_ids.iter().map(|&id|schematic.numbers[id]).sum();

    println!("Part 1: {}", sum);
}

fn part2() {
    let schematic = parse();

    let mut sum = 0;

    for i in 0..schematic.width {
        for j in 0..schematic.height {
            if let Value::Symbol('*') = schematic.values[i][j] {
                let mut num_ids = HashSet::new();

                if i > 0 && j > 0 {
                    if let Value::Digit { num_id, .. } = schematic.values[i - 1][j - 1] {
                        num_ids.insert(num_id);
                    }
                }
                if i > 0 {
                    if let Value::Digit { num_id, .. } = schematic.values[i - 1][j] {
                        num_ids.insert(num_id);
                    }
                }
                if i > 0 && j < schematic.height - 1 {
                    if let Value::Digit { num_id, .. } = schematic.values[i - 1][j + 1] {
                        num_ids.insert(num_id);
                    }
                }

                if j > 0 {
                    if let Value::Digit { num_id, .. } = schematic.values[i][j - 1] {
                        num_ids.insert(num_id);
                    }
                }
                if j < schematic.height - 1 {
                    if let Value::Digit { num_id, .. } = schematic.values[i][j + 1] {
                        num_ids.insert(num_id);
                    }
                }

                if i < schematic.width - 1 && j > 0 {
                    if let Value::Digit { num_id, .. } = schematic.values[i + 1][j - 1] {
                        num_ids.insert(num_id);
                    }
                }
                if i < schematic.width - 1 {
                    if let Value::Digit { num_id, .. } = schematic.values[i + 1][j] {
                        num_ids.insert(num_id);
                    }
                }
                if i < schematic.width - 1 && j < schematic.height - 1 {
                    if let Value::Digit { num_id, .. } = schematic.values[i + 1][j + 1] {
                        num_ids.insert(num_id);
                    }
                }

                if num_ids.len() == 2 {
                    let num_ids: Vec<_> = num_ids.iter().collect();
                    sum += schematic.numbers[*num_ids[0]] * schematic.numbers[*num_ids[1]];
                }
            }
        }
    }

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}

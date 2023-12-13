const INPUT: &'static str = include_str!("./input");

fn part1() {
    let mut total = 0;

    for line in INPUT.lines() {
        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                if first == None {
                    first = Some(n);
                }

                last = Some(n);
            }
        }

        total += first.unwrap() * 10 + last.unwrap();
    }

    println!("Part 1: {}", total);
}

fn part2() {
    let mut total = 0;

    for mut line in INPUT.lines() {
        let mut first = None;
        let mut last = None;

        while line.len() > 0 {
            if let Some(n) = line.chars().next().unwrap().to_digit(10) {
                if first == None {
                    first = Some(n);
                }

                last = Some(n);
            } else if line.starts_with("one") {
                if first == None {
                    first = Some(1);
                }
                last = Some(1);
            } else if line.starts_with("two") {
                if first == None {
                    first = Some(2);
                }
                last = Some(2);
            } else if line.starts_with("three") {
                if first == None {
                    first = Some(3);
                }
                last = Some(3);
            } else if line.starts_with("four") {
                if first == None {
                    first = Some(4);
                }
                last = Some(4);
            } else if line.starts_with("five") {
                if first == None {
                    first = Some(5);
                }
                last = Some(5);
            } else if line.starts_with("six") {
                if first == None {
                    first = Some(6);
                }
                last = Some(6);
            } else if line.starts_with("seven") {
                if first == None {
                    first = Some(7);
                }
                last = Some(7);
            } else if line.starts_with("eight") {
                if first == None {
                    first = Some(8);
                }
                last = Some(8);
            } else if line.starts_with("nine") {
                if first == None {
                    first = Some(9);
                }
                last = Some(9);
            }

            let (left, right) = line.split_at(1);
            line = right;
        }

        total += first.unwrap() * 10 + last.unwrap();
    }

    println!("Part 2: {}", total);
}

fn main() {
    part1();
    part2();
}

const INPUT: &'static str = include_str!("./input");

#[derive(Debug)]
struct Card {
    id: usize,
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Card {
    fn count_wins(&self) -> usize {
        let mut count = 0;

        for r in &self.right {
            if self.left.contains(r) {
                count += 1;
            }
        }

        count
    }

    fn count_rec(list: &Vec<Card>, id: usize) -> usize {
        let wins = list[id - 1].count_wins();

        if wins == 0 {
            return 1;
        }

        let mut total = 1;

        for next_id in id + 1..=id + wins {
            total += Card::count_rec(list, next_id);
        }

        total
    }

    fn points(&self) -> usize {
        let count = self.count_wins();

        if count == 0 {
            0
        } else {
            1 << (count - 1)
        }
    }
}

fn parse() -> Vec<Card> {
    INPUT
        .lines()
        .map(|l| {
            let (id, cards) = l.split_once(":").unwrap();
            let id =
                usize::from_str_radix(id.split_whitespace().skip(1).next().unwrap(), 10).unwrap();
            let (left, right) = cards.split_once("|").unwrap();
            Card {
                id,
                left: left
                    .split_whitespace()
                    .filter_map(|n| usize::from_str_radix(n, 10).ok())
                    .collect(),
                right: right
                    .split_whitespace()
                    .filter_map(|n| usize::from_str_radix(n, 10).ok())
                    .collect(),
            }
        })
        .collect()
}

fn part1() {
    let total: usize = parse().iter().map(|c| c.points()).sum();

    println!("Part 1: {}", total);
}

fn part2() {
    let cards = parse();
    let total: usize = (1..=cards.len())
        .map(|i| Card::count_rec(&parse(), i))
        .sum();

    println!("Part 2: {}", total);
}

fn main() {
    part1();
    part2();
}

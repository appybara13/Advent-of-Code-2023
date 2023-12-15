const INPUT: &'static str = include_str!("./input");

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u128,
    record_distance: u128,
}

impl Race {
    fn distance(&self, charge_time: u128) -> u128 {
        let speed = charge_time;
        let time_remaining = self.time - charge_time;
        speed * time_remaining
    }

    fn beats_record(&self, charge_time: u128) -> bool {
        self.record_distance < self.distance(charge_time)
    }
}

fn part1() {
    let times = INPUT
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|t| u128::from_str_radix(t, 10).unwrap());
    let record_distances = INPUT
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|d| u128::from_str_radix(d, 10).unwrap());

    let races: Vec<_> = times
        .zip(record_distances)
        .map(|(t, rd)| Race {
            time: t,
            record_distance: rd,
        })
        .collect();

    let score: usize = races
        .into_iter()
        .map(|r| {
            (1..r.time)
                .into_iter()
                .filter(|&ct| r.beats_record(ct))
                .count() as usize
        })
        .product();

    println!("Part 1: {}", score);
}

fn part2() {
    let time: u128 = u128::from_str_radix(
        INPUT
            .lines()
            .nth(0)
            .unwrap()
            .split_once(" ")
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .as_str(),
        10,
    )
    .unwrap();

    let record_distance: u128 = u128::from_str_radix(
        INPUT
            .lines()
            .nth(1)
            .unwrap()
            .split_once(" ")
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .as_str(),
        10,
    )
    .unwrap();

    let race = Race{ time, record_distance };

    let score = (1..race.time)
    .into_iter()
    .filter(|&ct| race.beats_record(ct))
    .count();

    println!("Part 2: {}", score);
}

fn main() {
    part1();
    part2();
}

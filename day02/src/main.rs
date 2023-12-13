const INPUT: &'static str = include_str!("./input");

#[derive(Debug, Copy, Clone)]
struct Cubeset {
    red: usize,
    green: usize,
    blue: usize,
}

impl Cubeset {
    fn parse(str: &str) -> Cubeset {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for s in str.split(", ") {
            let (n, c) = s.split_once(" ").unwrap();

            let v = usize::from_str_radix(n, 10).unwrap();

            match c {
                "red" => red += v,
                "green" => green += v,
                "blue" => blue += v,
                _ => {
                    panic!()
                }
            }
        }

        Cubeset { red, green, blue }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    sets: Vec<Cubeset>,
}

impl Game {
    fn parse(str: &str) -> Game {
        let (game_id, sets) = str.split_once(": ").unwrap();

        let id = usize::from_str_radix(game_id.split_once(" ").unwrap().1, 10).unwrap();

        Game {
            id,
            sets: sets.split("; ").map(Cubeset::parse).collect(),
        }
    }

    fn possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.sets
            .iter()
            .all(|set| set.red <= red && set.green <= green && set.blue <= blue)
    }

    fn smallest(&self) -> Cubeset {
        Cubeset {
            red: self.sets.iter().map(|s| s.red).max().unwrap(),
            green: self.sets.iter().map(|s| s.green).max().unwrap(),
            blue: self.sets.iter().map(|s| s.blue).max().unwrap(),
        }
    }
}

fn part1() {
    let mut total = 0;
    for line in INPUT.lines() {
        let game = Game::parse(line);

        if game.possible(12, 13, 14) {
            total += game.id;
        }
    }
    println!("Part 1: {}", total);
}

fn part2() {
    let mut total = 0;
    for line in INPUT.lines() {
        let game = Game::parse(line);

        total += game.smallest().power();
    }
    println!("Part 2: {}", total);
}

fn main() {
    part1();
    part2();
}

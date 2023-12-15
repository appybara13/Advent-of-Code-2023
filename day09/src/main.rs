const INPUT: &'static str = include_str!("./input");

fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct SensorModel {
    known: usize,
    order_start: Vec<isize>
}

impl SensorModel {
    fn get(&self, n: usize, o: usize) -> isize {
        if o >= self.order_start.len() {
            return 0;
        }

        if n == 0 {
            return self.order_start[o];
        }

        return self.get(n - 1, o)  + self.get(n - 1, o + 1);
    }

    fn get_next(&self) -> isize {
        self.get(self.known, 0)
    }

    fn get_prev(&self) -> isize {
        let mut value = 0;

        let mut os = self.order_start.clone();
        os.reverse();

        for o in os {
            value = o - value;
        }

        value
    }

    fn from_history(history: Vec<isize>) -> Self {
        let mut model = SensorModel {
            known: history.len(),
            order_start: vec![history[0]]
        };

        model.add_next_orders(history);

        model
    }

    fn add_next_orders(&mut self, differences: Vec<isize>) {
        let next_differences: Vec<_> = (0..differences.len()-1).map(|i| differences[i + 1] - differences[i]).collect();
    
        if next_differences.iter().all(|&d| d == 0) {
            return;
        }

        self.order_start.push(next_differences[0]);

        self.add_next_orders(next_differences);
    }
}

impl From<&str> for SensorModel {
    fn from(value: &str) -> Self {
        SensorModel::from_history(value.split_whitespace().map(|v| isize::from_str_radix(v, 10).unwrap()).collect())
    }
}

fn part1() {
    let sensors: Vec<SensorModel> = INPUT.lines().map(|l| l.into()).collect();
    let next: Vec<_> = sensors.iter().map(|s|s.get_next()).collect();

    let sum: isize = next.iter().sum();

    println!("Part 1: {}", sum);
}

fn part2() {
    let sensors: Vec<SensorModel> = INPUT.lines().map(|l| l.into()).collect();
    let prev: Vec<_> = sensors.iter().map(|s|s.get_prev()).collect();

    let sum: isize = prev.iter().sum();

    println!("Part 2: {}", sum);
}


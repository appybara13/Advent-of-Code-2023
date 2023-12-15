const INPUT: &'static str = include_str!("./input");

#[derive(Debug, Clone)]
struct RangeSet {
    ranges: Vec<(usize, usize)>
}
impl RangeSet {
    fn min(&self) -> usize {
        self.ranges.iter().map(|r|r.0).min().unwrap()
    }

    fn shift(&mut self, positive: usize, negative: usize) {
        for range in &mut self.ranges {
            range.0 += positive;
            range.0 -= negative;

            range.1 += positive;
            range.1 -= negative;
        }
    }

    fn combine(&mut self, other: &mut RangeSet) {
        self.ranges.append(&mut other.ranges);
    }

    fn overlap(&self, start: usize, end: usize) -> (RangeSet, RangeSet)  {
        let mut overlaps = RangeSet {ranges: vec![]};
        let mut remaining = RangeSet {ranges: vec![]};

        for &range in &self.ranges {
            if range.1 <= start {
                remaining.ranges.push(range);
            }
            else if range.0 >= end {
                remaining.ranges.push(range);
            }
            else if range.0 >= start && range.1 <= end {
                overlaps.ranges.push(range);
            } 
            else if range.0 < start && range.1 > end {
                overlaps.ranges.push((start, end));
                remaining.ranges.push((range.0, start));
                remaining.ranges.push((end, range.1));
            }
            else if range.0 >= start  && range.1 > end {
                overlaps.ranges.push((range.0, end));
                remaining.ranges.push((end, range.1));


            } else if range.0 < start  && range.1 <= end {
                overlaps.ranges.push((start, range.1));
                remaining.ranges.push((range.0, start));
            } else  {
                panic!()
            }
        }

        (overlaps, remaining)
    }
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl From<&str> for MapRange {
    fn from(value: &str) -> Self {
        let mut iter = value
            .split_whitespace()
            .map(|n| usize::from_str_radix(n, 10).unwrap());

        Self {
            destination_start: iter.next().unwrap(),
            source_start: iter.next().unwrap(),
            length: iter.next().unwrap(),
        }
    }
}

impl MapRange {
    fn map(&self, source: usize) -> Option<usize> {
        if source >= self.source_start && source - self.source_start < self.length {
            return Some(source - self.source_start + self.destination_start);
        }

        None
    }

    fn map_range(&self, source: RangeSet) -> (RangeSet, RangeSet) {
        let (mut overlap, remaining) = source.overlap(self.source_start, self.source_start + self.length);
        overlap.shift(self.destination_start, self.source_start);

        (overlap, remaining)
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self {
            ranges: value.lines().map(|l| l.into()).collect(),
        }
    }
}

impl Map {
    fn map(&self, source: usize) -> usize {
        for range in &self.ranges {
            if let Some(destination) = range.map(source) {
                return destination;
            }
        }

        return source;
    }

    fn map_range(&self, source: RangeSet) -> RangeSet {
        let mut remaining = source;
        let mut mapped = RangeSet{ ranges: vec![] };

        for range in &self.ranges {
            let mut result = range.map_range(remaining);
            remaining = result.1;
            mapped.combine(&mut result.0);
        }

        mapped.combine(&mut remaining);

        mapped
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let (seeds, value) = value.split_once("\nseed-to-soil map:\n").unwrap();
        let (seed_to_soil, value) = value.split_once("\nsoil-to-fertilizer map:\n").unwrap();
        let (soil_to_fertilizer, value) = value.split_once("\nfertilizer-to-water map:\n").unwrap();
        let (fertilizer_to_water, value) = value.split_once("\nwater-to-light map:\n").unwrap();
        let (water_to_light, value) = value.split_once("\nlight-to-temperature map:\n").unwrap();
        let (light_to_temperature, value) = value
            .split_once("\ntemperature-to-humidity map:\n")
            .unwrap();
        let (temperature_to_humidity, humidity_to_location) =
            value.split_once("\nhumidity-to-location map:\n").unwrap();

        Almanac {
            seeds: seeds.split_once(":").unwrap().1.split_whitespace().map(|n|usize::from_str_radix(n, 10).unwrap()).collect(),
            seed_to_soil: seed_to_soil.into(),
            soil_to_fertilizer: soil_to_fertilizer.into(),
            fertilizer_to_water: fertilizer_to_water.into(),
            water_to_light: water_to_light.into(),
            light_to_temperature: light_to_temperature.into(),
            temperature_to_humidity: temperature_to_humidity.into(),
            humidity_to_location: humidity_to_location.into(),
        }
    }
}

fn part1() {
    let almanac = Almanac::from(INPUT);

    let min = almanac.seeds.iter().cloned()
    .map(|n| almanac.seed_to_soil.map(n))
    .map(|n| almanac.soil_to_fertilizer.map(n))
    .map(|n| almanac.fertilizer_to_water.map(n))
    .map(|n| almanac.water_to_light.map(n))
    .map(|n| almanac.light_to_temperature.map(n))
    .map(|n| almanac.temperature_to_humidity.map(n))
    .map(|n| almanac.humidity_to_location.map(n))
    .min().unwrap();

    println!("Part 1: {}", min);
}

fn part2() {
    let almanac = Almanac::from(INPUT);

    let mut min: usize = 999999999999999999;

    for seeds in almanac.seeds.chunks(2) {
        let mut range = RangeSet{ranges: vec![(seeds[0], seeds[0] + seeds[1])]};

        range = almanac.seed_to_soil.map_range(range);
        range = almanac.soil_to_fertilizer.map_range(range);
        range = almanac.fertilizer_to_water.map_range(range);
        range = almanac.water_to_light.map_range(range);
        range = almanac.light_to_temperature.map_range(range);
        range = almanac.temperature_to_humidity.map_range(range);
        range = almanac.humidity_to_location.map_range(range);

        let range_min = range.min();

        min = min.min(range_min);
    }

    println!("Part 2: {}", min);
}

fn main() {
    part1();
    part2();
}

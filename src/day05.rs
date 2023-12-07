use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

pub fn run(contents: String) {
    let mut parts = contents.split("\n\n");
    let seed_part: Vec<&str> = parts.next().unwrap().split(":").collect();
    let seeds: Vec<usize> = seed_part[1]
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let maps: Vec<Layer> = parts
        .filter(|p| !p.trim().is_empty())
        .map(|p| Layer::parse(p.trim()))
        .collect();

    let l = maps.len();
    println!("{l} length");

    let mut inputs = seeds.clone();
    for m in maps.iter() {
        inputs = inputs.iter().map(|i| m.map(*i)).collect();
    }
    let min = inputs.iter().min().unwrap();
    println!("Min is {min}");

    println!("Part 2----");
    let size: usize = seeds.iter().tuples().map(|(_, e)| e).sum();
    println!("To test {size}");
    let min2 = seeds
        .into_iter()
        .tuples()
        .par_bridge()
        .map(|(s, l)| {
            let range = s..(s + l);
            println!("Calc for {range:?}...");
            let mapped = range.clone().into_iter().map(|s| {
                let mut i = s;
                for m in maps.iter() {
                    i = m.map(i);
                }
                i
            });
            let min = mapped.min().unwrap();
            println!("End for {range:?}");
            min
        })
        .min()
        .unwrap();
    println!("Min 2 is {min2}");
}
#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

struct RangeMapping {
    d_start: usize,
    s_start: usize,
    length: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start: start,
            end: end,
        }
    }
}

impl RangeMapping {
    fn parse(line: &str) -> Self {
        let r: Vec<usize> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        RangeMapping {
            d_start: r[0],
            s_start: r[1],
            length: r[2],
        }
    }

    fn map(&self, input: usize) -> Option<usize> {
        if input >= self.s_start && input < self.s_start + self.length {
            let offset = input - self.s_start;
            Some(self.d_start + offset)
        } else {
            None
        }
    }
}

struct Layer {
    from: String,
    to: String,
    ranges: Vec<RangeMapping>,
}

impl Layer {
    fn parse(str: &str) -> Self {
        let re: Regex = Regex::new(r"([a-z]+)-to-([a-z]+) map:").unwrap();
        let mut lines = str.lines();
        let names = lines.next().unwrap();
        let captures = re.captures(names).unwrap();
        let from = captures.get(1).unwrap().as_str();
        let to = captures.get(2).unwrap().as_str();
        let ranges: Vec<RangeMapping> = lines
            .map(RangeMapping::parse)
            .sorted_by_key(|r| r.s_start)
            .collect();
        Self {
            from: String::from(from),
            to: String::from(to),
            ranges: ranges,
        }
    }

    fn map(&self, input: usize) -> usize {
        let dest = self.ranges.iter().find_map(|r| r.map(input));
        dest.unwrap_or(input)
    }
}

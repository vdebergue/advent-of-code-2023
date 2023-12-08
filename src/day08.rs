use std::collections::HashMap;

use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use regex::Regex;

pub fn run(contents: String) {
    let mut lines = contents.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    let nodes: HashMap<&str, (&str, &str)> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let matches = re.captures(l).unwrap();
            let label = matches.get(1).unwrap().as_str();
            let left = matches.get(2).unwrap().as_str();
            let right = matches.get(3).unwrap().as_str();
            (label, (left, right))
        })
        .collect();

    let p1 = part1(&instructions, &nodes);
    println!("Steps p1 {p1}");
    let p2 = part2(&instructions, &nodes);
    println!("Steps p2 {p2}");
}

fn part1(instructions: &Vec<char>, nodes: &HashMap<&str, (&str, &str)>) -> usize {
    let mut curr = "AAA";
    let mut steps = 0;
    while curr != "ZZZ" {
        let instr = instructions[steps % instructions.len()];
        let &pair = nodes.get(curr).unwrap();
        let next = match instr {
            'L' => pair.0,
            'R' => pair.1,
            _ => panic!("Error"),
        };
        curr = next;
        steps += 1;
    }
    steps
}

fn part2(instructions: &Vec<char>, nodes: &HashMap<&str, (&str, &str)>) -> usize {
    let starts: Vec<&str> = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|&c| c)
        .collect();

    let steps: Vec<usize> = starts
        .par_iter()
        .map(|s| calc_steps(instructions, nodes, s))
        .collect();
    steps.iter().fold(1, |acc, &n| num::integer::lcm(acc, n))
}

fn calc_steps(instructions: &Vec<char>, nodes: &HashMap<&str, (&str, &str)>, start: &str) -> usize {
    let mut steps = 0;
    let mut curr = start;
    while !curr.ends_with("Z") {
        let instr = instructions[steps % instructions.len()];
        let &pair = nodes.get(curr).unwrap();
        let next = match instr {
            'L' => pair.0,
            'R' => pair.1,
            _ => panic!("Error"),
        };
        curr = next;
        steps += 1;
    }
    steps
}

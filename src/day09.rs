pub fn run(contents: String) {
    let inputs: Vec<Vec<i32>> = contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" ").map(|n| n.trim().parse().unwrap()).collect())
        .collect();

    let sum = inputs
        .iter()
        .fold(0, |acc, series| acc + predict_last(series));
    println!("Part 1: {sum}");

    let sum2 = inputs
        .iter()
        .fold(0, |acc, series| acc + predict_first(series));
    println!("Part 2: {sum2}");
}

fn derivative(input: &Vec<i32>) -> Vec<i32> {
    input.windows(2).map(|t| t[1] - t[0]).collect()
}

fn predict_last(series: &Vec<i32>) -> i32 {
    let mut d = derivative(series);
    let mut last: Vec<i32> = vec![series.last().unwrap().clone()];
    while !d.iter().all(|&n| n == 0) {
        last.push(d.last().unwrap().clone());
        d = derivative(&d);
    }
    last.iter().sum()
}

fn predict_first(series: &Vec<i32>) -> i32 {
    let mut d = derivative(series);
    let mut first: Vec<i32> = vec![series.first().unwrap().clone()];
    while !d.iter().all(|&n| n == 0) {
        first.push(d.first().unwrap().clone());
        d = derivative(&d);
    }
    first
        .iter()
        .enumerate()
        .map(|(i, n)| n * (-1i32).pow(i as u32))
        .sum()
}

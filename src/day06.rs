pub fn run(contents: String) {
    let lines: Vec<&str> = contents.lines().collect();
    let times = parse_line(lines[0]);
    let distances = parse_line(lines[1]);

    let res_p1 = part1(&times, &distances);
    println!("part1: {res_p1}");

    let time = parse_line2(lines[0]);
    let distance = parse_line2(lines[1]);
    let res_p2: u64 = part2(time, distance);
    let res_p2_bis = part2_bis(time, distance);
    println!("From calc: {res_p2_bis}");
    println!("part2: {res_p2}");
}

fn part1(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let mut total: u64 = 1;
    for idx in 0..times.len() {
        let race_time = times[idx];
        let record = distances[idx];
        let mut win = 0;
        for t in 1..race_time {
            let moved = distance_moved(t, race_time);
            if moved > record {
                win += 1;
            }
        }
        total = win * total
    }
    total
}

fn part2(time: u64, distance: u64) -> u64 {
    part1(&vec![time], &vec![distance])
}

// Equation is x^2 - Tx + D = 0
// Distance between solutions is sqrt(delta)
// Delta = b^2 -4ac
fn part2_bis(time: u64, distance: u64) -> f64 {
    let delta: f64 = (time.pow(2) - 4 * distance) as f64;
    let r = delta.sqrt();
    r.ceil()
}

fn distance_moved(pressed_time: u64, race_time: u64) -> u64 {
    let speed = pressed_time;
    let movement_time = race_time - pressed_time;
    movement_time * speed
}

fn parse_line(line: &str) -> Vec<u64> {
    line.split(" ")
        .map(|c| c.trim())
        .filter(|s| !s.is_empty())
        .skip(1)
        .map(|c| c.parse().unwrap())
        .collect()
}

fn parse_line2(line: &str) -> u64 {
    let x = line.replace(" ", "");
    let digits: String = x.chars().filter(|c| c.is_digit(10)).collect();
    digits.parse().unwrap()
}

use std::collections::HashSet;

pub fn run(contents: String) {
    let games: Vec<Game> = contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(Game::parse)
        .collect();

    let mut sum1 = 0;
    for game in &games {
        sum1 += game.points();
    }
    println!("Sum is {sum1}");

    let mut scores = vec![1; games.len()];
    for (idx, game) in games.iter().enumerate() {
        let n = game.matches();
        let curr = scores[idx];
        for i in idx..(idx + n) {
            scores[i + 1] = scores[i + 1] + curr;
        }
    }
    let sum2: u32 = scores.iter().sum();
    println!("Sum part2 is {sum2}");
}

struct Game {
    wins: HashSet<u32>,
    nums: HashSet<u32>,
}

impl Game {
    fn matches(&self) -> usize {
        self.wins.intersection(&self.nums).count()
    }

    fn points(&self) -> i32 {
        let matches: u32 = self.matches().try_into().unwrap();
        if matches > 0 {
            2_i32.pow(matches - 1)
        } else {
            0
        }
    }

    fn parse(line: &str) -> Self {
        let p1: Vec<&str> = line.split(":").collect();
        let p2: Vec<&str> = p1[1].split("|").collect();
        let wins: HashSet<u32> = p2[0]
            .split(" ")
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let nums: HashSet<u32> = p2[1]
            .split(" ")
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        Game {
            wins: wins,
            nums: nums,
        }
    }
}

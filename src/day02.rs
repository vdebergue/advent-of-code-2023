use std::{cmp::max, ops::Add};

pub fn run(contents: String) {
    let games = parse_games(contents);

    let target = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum: usize = 0;
    for (idx, game) in games.iter().enumerate() {
        let game_possible = game.iter().all(|d| target.is_possible(d));
        if game_possible {
            sum += idx + 1;
        }
    }
    println!("Result is {sum}");

    let mut sum2: u32 = 0;
    for game in games.iter() {
        let min = game.iter().fold(Cubes::default(), |acc, c| acc.min(c));
        sum2 += min.power();
    }
    println!("Result part 2 is {sum2}");
}

fn parse_games(contents: String) -> Vec<Vec<Cubes>> {
    contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let pa: Vec<&str> = line.split(":").collect();
            let draws: Vec<Cubes> = pa[1]
                .split(";")
                .map(|draw| {
                    let cubes = draw.split(",").map(parse_cube);
                    cubes.fold(Cubes::default(), |acc, c| acc + c)
                })
                .collect();
            draws
        })
        .collect()
}

fn parse_cube(str: &str) -> Cubes {
    let mut parts = str.trim().split(" ");
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let color = parts.next().unwrap();
    match color {
        "red" => Cubes {
            red: n,
            green: 0,
            blue: 0,
        },
        "green" => Cubes {
            red: 0,
            green: n,
            blue: 0,
        },
        "blue" => Cubes {
            red: 0,
            green: 0,
            blue: n,
        },
        _ => panic!("unknown color {color}"),
    }
}

#[derive(Default, Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Add for Cubes {
    type Output = Cubes;
    fn add(self, rhs: Self) -> Self::Output {
        Cubes {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Cubes {
    fn is_possible(&self, draw: &Cubes) -> bool {
        self.red >= draw.red && self.green >= draw.green && self.blue >= draw.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn min(&self, rhs: &Cubes) -> Cubes {
        Cubes {
            red: max(self.red, rhs.red),
            green: max(self.green, rhs.green),
            blue: max(self.blue, rhs.blue),
        }
    }
}

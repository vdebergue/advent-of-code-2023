use regex::Regex;

pub fn run(contents: String) {
    let score = contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .fold(0, |acc, line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            let first: u32 = (*digits.first().unwrap()).to_digit(10).unwrap();
            let last: u32 = (*digits.last().unwrap()).to_digit(10).unwrap();
            let line_score = first * 10 + last;
            // println!("line score {first} {last} {line_score}");
            acc + line_score
        });

    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[\d]").unwrap();
    let score2 = contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .fold(0, |acc, l| {
            let mut first: Option<u32> = Option::None;
            let mut last: Option<u32> = Option::None;

            let mut line = l;
            loop {
                match re.find(line) {
                    Some(m) => {
                        let ds = m.as_str();
                        let di = str_to_digit(ds);
                        first = first.or(Some(di));
                        last = Some(di).or(last);
                        line = &line[m.start() + 1..];
                    }
                    None => {
                        break;
                    }
                }
            }
            let calibration_value = first.unwrap() * 10 + last.unwrap();
            acc + calibration_value
        });

    println!("The score is {score}");
    println!("The score part 2 is {score2}");
}

fn str_to_digit(str: &str) -> u32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str.chars().next().unwrap().to_digit(10).unwrap(),
    }
}

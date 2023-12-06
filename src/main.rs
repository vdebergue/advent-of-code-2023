use std::env;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).expect("Args should contain day");
    let filename = args.get(2).expect("Args should contain a filename");
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    match day.as_str() {
        "1" => day01::run(contents),
        "2" => day02::run(contents),
        "3" => day03::run(contents),
        "4" => day04::run(contents),
        "5" => day05::run(contents),
        "6" => day06::run(contents),
        _ => panic!("Not implemented yet"),
    };
}

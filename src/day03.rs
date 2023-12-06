use std::collections::HashMap;

pub fn run(contents: String) {
    let grid: Vec<Vec<char>> = contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut sum = 0;
    let mut current_number: Vec<char> = vec![];
    let mut should_count = false;
    let mut gear: Option<(usize, usize)> = None;
    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let current = grid[y][x];
            if current.is_digit(10) {
                current_number.push(current);
                let neighbours = get_neighbours(&grid, x, y);
                let has_symbol = neighbours.iter().any(|c| c.0 != '.' && !c.0.is_digit(10));
                let is_gear = neighbours.iter().find(|c| c.0 == '*');
                match is_gear {
                    Some((_, xg, yg)) => gear = Some((*xg, *yg)),
                    None => (),
                };
                if has_symbol {
                    should_count = true;
                }
            } else {
                // reset
                if should_count {
                    let s: String = current_number.into_iter().collect();
                    let n: u32 = s.parse().unwrap();
                    sum += n;
                    match gear {
                        Some(pos) => match gear_map.get_mut(&pos) {
                            Some(nums) => {
                                nums.push(n);
                                ()
                            }
                            None => {
                                gear_map.insert(pos, vec![n]);
                                ();
                            }
                        },
                        None => (),
                    }
                }

                current_number = vec![];
                should_count = false;
                gear = None;
            }
        }
        // reset
        if should_count {
            let s: String = current_number.into_iter().collect();
            let n: u32 = s.parse().unwrap();
            println!(" => {n}");
            sum += n;
            match gear {
                Some(pos) => match gear_map.get_mut(&pos) {
                    Some(nums) => {
                        nums.push(n);
                        ()
                    }
                    None => {
                        gear_map.insert(pos, vec![n]);
                        ();
                    }
                },
                None => (),
            }
        }
        current_number = vec![];
        should_count = false;
        gear = None;
    }
    println!("Res is {sum}");

    let mut sum2: u32 = 0;
    for (_, nums) in gear_map.iter() {
        if nums.len() == 2 {
            sum2 += nums[0] * nums[1];
        }
    }
    println!("Res part 2 is {sum2}");
}

fn get_neighbours(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(char, usize, usize)> {
    let mut out: Vec<(char, usize, usize)> = vec![];
    let y_max = grid.len() - 1;
    let x_max = grid[0].len() - 1;

    if x > 0 && y > 0 {
        out.push((grid[y - 1][x - 1], x - 1, y - 1));
    }
    if x > 0 {
        out.push((grid[y][x - 1], x - 1, y));
    }
    if y > 0 {
        out.push((grid[y - 1][x], x, y - 1));
    }
    if x < x_max && y < y_max {
        out.push((grid[y + 1][x + 1], x + 1, y + 1));
    }
    if x < x_max {
        out.push((grid[y][x + 1], x + 1, y));
    }
    if y < y_max {
        out.push((grid[y + 1][x], x, y + 1));
    }
    if y > 0 && x < x_max {
        out.push((grid[y - 1][x + 1], x + 1, y - 1));
    }
    if x > 0 && y < y_max {
        out.push((grid[y + 1][x - 1], x - 1, y + 1));
    }
    out
}

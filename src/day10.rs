// .....
// .S-7.
// .|.|.
// .L-J.
// .....

// w = 5 h = 5
// S p=6 x=1 y=1

use std::collections::HashSet;

use num::Integer;

pub fn run(contents: String) {
    let grid = Grid::parse(&contents);
    let start = grid.start();
    let start_neighbors = grid.neighbors_position(start);
    // dbg!(&grid);
    println!("Grid w={} h={}", grid.width, grid.height);
    let mut boucle: HashSet<usize> = HashSet::new();
    for n in start_neighbors {
        let mut previous = start;
        let mut current = n;
        boucle.clear();
        boucle.insert(current);
        while let Some(next) = grid.follow_pipe(current, previous) {
            boucle.insert(next);
            if grid.chars[next] == 'S' {
                boucle.insert(next);
                break;
            }
            previous = current;
            current = next;
        }
    }
    println!("Size is {}, length = {}", boucle.len(), boucle.len() / 2);

    // S might be included, depends on the input...
    let side_chars: HashSet<char> = "LJ|".chars().collect();
    let sum = grid
        .chars
        .iter()
        .enumerate()
        .filter(|(pos, &c)| {
            if !boucle.contains(pos) {
                let (y, _) = pos.div_rem(&grid.width);
                let left = (y * grid.width)..(*pos);
                let sides = left
                    .filter(|n| {
                        let char = grid.chars[*n];
                        side_chars.contains(&char) && boucle.contains(n)
                    })
                    .count();
                sides % 2 == 1
            } else {
                false
            }
        })
        .count();

    println!("Sum is {sum}");
}

#[derive(Debug)]
struct Grid {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(str: &str) -> Self {
        let chars: Vec<char> = str.chars().filter(|&c| c != ' ' && c != '\n').collect();
        let h = str.lines().filter(|l| !l.is_empty()).count();
        let w = str.lines().next().unwrap().len();
        Self {
            chars: chars,
            width: w,
            height: h,
        }
    }
    fn start(&self) -> usize {
        self.chars.iter().position(|&c| c == 'S').unwrap()
    }
    fn neighbors_position(&self, pos: usize) -> Vec<usize> {
        let mut out = vec![];
        let (y, x) = pos.div_rem(&self.width);
        if x > 0 {
            out.push(pos - 1)
        }
        if x < self.width {
            out.push(pos + 1)
        }
        if y > 0 {
            out.push(pos - self.width);
        }
        if y < self.height {
            out.push(pos + self.width);
        }
        out
    }
    fn follow_pipe(&self, current: usize, previous: usize) -> Option<usize> {
        let char = self.chars[current];
        let next = match char {
            '|' => {
                if current == previous + self.width {
                    Some(current + self.width)
                } else if current == previous - self.width {
                    Some(current - self.width)
                } else {
                    None
                }
            }
            '-' => {
                if current == previous + 1 {
                    Some(current + 1)
                } else if current == previous - 1 {
                    Some(current - 1)
                } else {
                    None
                }
            }
            'L' => {
                if current == previous + self.width {
                    Some(current + 1)
                } else if current == previous - 1 {
                    Some(current - self.width)
                } else {
                    None
                }
            }
            'J' => {
                if current == previous + self.width {
                    Some(current - 1)
                } else if current == previous + 1 {
                    Some(current - self.width)
                } else {
                    None
                }
            }
            '7' => {
                if current == previous + 1 {
                    Some(current + self.width)
                } else if current == previous - self.width {
                    Some(current - 1)
                } else {
                    None
                }
            }
            'F' => {
                if current == previous - 1 {
                    Some(current + self.width)
                } else if current == previous - self.width {
                    Some(current + 1)
                } else {
                    None
                }
            }
            _ => None,
        };
        let neighbors = self.neighbors_position(current);
        next.filter(|n| neighbors.contains(n))
    }
}

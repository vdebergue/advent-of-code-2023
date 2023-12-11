// .....
// .S-7.
// .|.|.
// .L-J.
// .....

// w = 5 h = 5
// S p=6 x=1 y=1

pub fn run(contents: String) {
    let grid = Grid::parse(&contents);
    let start = grid.start();
    let n = grid.neighbors_position(start);
    // dbg!(&grid);
    dbg!(n);
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
        let x = pos % self.width;
        let y = pos - (pos / self.width) * self.width;
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
}

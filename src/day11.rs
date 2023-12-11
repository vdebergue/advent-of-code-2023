use itertools::Itertools;

pub fn run(contents: String) {
    let universe = Universe::parse(&contents);
    println!("Universe size {}", universe.galaxies.len());
    let s: usize = universe
        .galaxies
        .iter()
        .combinations(2)
        .map(|v| universe.distance(v[0], v[1], 2))
        .sum();
    println!("Sum is {s}");

    let s2: usize = universe
        .galaxies
        .iter()
        .combinations(2)
        .map(|v| universe.distance(v[0], v[1], 1000000))
        .sum();
    println!("Sum is {s2}")
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }
}

struct Universe {
    galaxies: Vec<Pos>,
}

impl Universe {
    fn parse(str: &str) -> Self {
        let galaxies: Vec<Pos> = str
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(y, line)| {
                let g: Vec<Pos> = line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(|(x, _)| Pos::new(x, y))
                    .collect();
                g
            })
            .collect();
        Self { galaxies: galaxies }
    }

    fn distance(&self, p1: &Pos, p2: &Pos, factor: usize) -> usize {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);
        let x_range = (x1.min(x2))..(x1.max(x2));
        let dx: usize = x_range
            .map(|x| {
                let has_galaxy = self.galaxies.iter().any(|g| g.x == x);
                if has_galaxy {
                    1
                } else {
                    factor
                }
            })
            .sum();
        let y_range = (y1.min(y2))..(y1.max(y2));
        let dy: usize = y_range
            .map(|y| {
                let has_galaxy = self.galaxies.iter().any(|g| g.y == y);
                if has_galaxy {
                    1
                } else {
                    factor
                }
            })
            .sum();
        dx + dy
    }
}

use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

type Coordinate = (isize, isize);

fn parse_coordinate(line: &str) -> Result<Coordinate> {
    let cs = line
        .split(",")
        .map(|s| s.trim().parse::<isize>())
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if cs.len() != 2 {
        return Err("Expected two numbers per line".into());
    }
    Ok((cs[0], cs[1]))
}

fn read_coordinates(filename: &Path) -> Result<Vec<Coordinate>> {
    let file = File::open(filename).map_err(|e| {
        format!("Unable to open input file '{}': {}", filename.display(), e).to_string()
    })?;
    let file = BufReader::new(file);
    let mut coordinates = Vec::new();
    for line in file.lines() {
        let line = line?;

        match parse_coordinate(&line) {
            Ok(n) => coordinates.push(n),
            Err(e) => return Err(e.into()),
        }
    }
    return Ok(coordinates);
}

fn distance(x1: isize, x2: isize) -> isize {
    (x1 as isize - x2 as isize).abs()
}

fn manhattan_dist(c1: Coordinate, c2: Coordinate) -> isize {
    distance(c1.0, c2.0) + distance(c1.1, c2.1)
}

struct GridInfo {
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
}

impl GridInfo {
    fn new(cs: &Vec<Coordinate>) -> GridInfo {
        let x0 = cs.iter().min_by_key(|c| c.0).unwrap().0;
        let x1 = cs.iter().max_by_key(|c| c.0).unwrap().0;
        let y0 = cs.iter().min_by_key(|c| c.1).unwrap().1;
        let y1 = cs.iter().max_by_key(|c| c.1).unwrap().1;

        GridInfo { x0, y0, x1, y1 }
    }

    fn width(&self) -> usize {
        (self.x1 - self.x0) as usize
    }
    fn height(&self) -> usize {
        (self.y1 - self.y0) as usize
    }
    fn size(&self) -> usize {
        self.width() * self.height()
    }
    fn on_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || x == (self.width() - 1) || y == 0 || y == (self.height() - 1)
    }
}

fn find_closest_coord(cs: &Vec<Coordinate>, coord: Coordinate) -> Option<usize> {
    let distances = cs
        .iter()
        .map(|&c| manhattan_dist(c, coord))
        .collect::<Vec<_>>();

    let (min_index, min_dist) = distances.iter().enumerate().min_by_key(|x| x.1).unwrap();

    if distances.iter().filter(|&d| d == min_dist).count() > 1 {
        return None;
    }
    Some(min_index)
}

fn calc_distance_sums(cs: &Vec<Coordinate>, coord: Coordinate) -> isize {
    cs.iter().map(|&c| manhattan_dist(c, coord)).sum()
}

type GridFunc<T> = dyn Fn(&Vec<Coordinate>, Coordinate) -> T;

fn calc_grid<T: Default>(cs: &Vec<Coordinate>, gi: &GridInfo, func: &GridFunc<T>) -> Vec<T> {
    let mut grid = Vec::with_capacity(gi.size());
    grid.resize_with(gi.size(), Default::default);
    for y in 0..gi.height() {
        for x in 0..gi.width() {
            grid[y * gi.width() + x] = func(cs, (x as isize + gi.x0, y as isize + gi.y0));
        }
    }
    grid
}

fn task_1(cs: &Vec<Coordinate>) -> usize {
    let gi = GridInfo::new(cs);
    println!("Grid size: {}x{}", gi.width(), gi.height());

    let grid = calc_grid(cs, &gi, &find_closest_coord);
    let mut counts = vec![0i32; cs.len()];
    for (index, closest) in grid.into_iter().enumerate() {
        if let Some(c) = closest {
            let x = index % gi.width();
            let y = index / gi.width();
            if gi.on_edge(x, y) {
                // On grid edge => infinite area
                counts[c] = -1;
            }
            if counts[c] >= 0 {
                counts[c] += 1;
            }
        }
    }
    counts.into_iter().max().unwrap() as usize
}

fn task_2(cs: &Vec<Coordinate>) -> usize {
    const SUM_THRESHOLD: isize = 10000;

    let average_x = cs.iter().map(|&c| c.0).sum::<isize>() / cs.len() as isize;
    let average_y = cs.into_iter().map(|c| c.1).sum::<isize>() / cs.len() as isize;

    let mut total_area = if calc_distance_sums(cs, (average_x, average_y)) < SUM_THRESHOLD {
        1
    } else {
        0
    };

    let mut r = 1isize;
    loop {
        let mut sums: Vec<isize> = Vec::with_capacity(4 * r as usize + 4);
        for i in 0..(2 * r + 1) {
            sums.push(calc_distance_sums(cs, (average_x - r + i, average_y - r)));
            sums.push(calc_distance_sums(cs, (average_x - r + i, average_y + r)));
        }
        for i in 1..(2 * r) {
            sums.push(calc_distance_sums(cs, (average_x - r, average_y - r + i)));
            sums.push(calc_distance_sums(cs, (average_x + r, average_y - r + i)));
        }

        let area_expansion = sums.iter().filter(|&&c| c < SUM_THRESHOLD).count();
        //println!("AE: {}, {}, {}", r, sums.len(), area_expansion);
        if area_expansion == 0 {
            break;
        }
        total_area += area_expansion;

        r += 1;
    }

    total_area
}

fn run() -> Result<()> {
    let mut input_fn = std::env::current_exe()?.parent().unwrap().to_path_buf();
    input_fn.push("../../data/input.txt");

    let coords = read_coordinates(&input_fn)?;

    let max_area = task_1(&coords);
    println!("Max area: {:?}", max_area);
    assert_eq!(max_area, 3604);

    let r = task_2(&coords);
    println!("Total area: {:?}", r);
    assert_eq!(r, 46563);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}

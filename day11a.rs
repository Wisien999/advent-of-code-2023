use std::ops::Add;
use std::cmp::min;
use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}


fn precompute(space: &Vec<Vec<char>>) -> (Vec<i64>, Vec<i64>, Vec<Point>) {
    let rs = space.len();
    let cs = space[0].len();
    let mut rows: Vec<i64> = vec![1_000_000; rs];
    let mut cols: Vec<i64> = vec![1_000_000; cs];
    let mut points: Vec<Point> = Vec::new();

    for (i, row) in space.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '#' {
                points.push(Point { x: j, y: i });
                rows[i] = 1;
                cols[j] = 1;
            }
        }
    }

    (rows, cols, points)
}

fn cumsum<T>(x: &[T]) -> Vec<T>
where
    T: Clone,
    for<'r> &'r T: Add<&'r T, Output=T>,
{
    let mut y = Vec::with_capacity(x.len());

    if !x.is_empty() {
        y.push(x[0].clone());

        for i in 1..x.len() {
            y.push(&y[i - 1] + &x[i]);
        }
    }

    y
}

fn distance_cumsum(cr: &Vec<i64>, cc: &Vec<i64>, p1: &Point, p2: &Point) -> i64 {
    let xs = min(p1.x, p2.x);
    let xl = max(p1.x, p2.x);

    let ys = min(p1.y, p2.y);
    let yl = max(p1.y, p2.y);

    let xd = cc[xl] - cc[xs];
    let yd = cr[yl] - cr[ys];

    xd + yd
}


fn calc_sum_of_distances(space: &Vec<Vec<char>>) -> i64 {
    let (row_w, col_w, points) = precompute(space);
    let cumsum_r = cumsum(&row_w);
    let cumsum_c = cumsum(&col_w);

    let mut s = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            s += distance_cumsum(&cumsum_r, &cumsum_c, &p1, &p2);
        }
    }

    
    s
}


fn main() {
    let sketch = 
        include_str!("day11a_data.txt")
//"...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#....."
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .filter(|line| line.len() > 0)
        .collect::<Vec<Vec<char>>>();

    println!("{:?}", sketch);

    let steps = calc_sum_of_distances(&sketch);

    println!("Suma: {}", steps);
}


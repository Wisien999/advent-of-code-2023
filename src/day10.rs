use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn move_in(&self, direction: &Direction, x_size: usize, y_size: usize) -> Option<Point> {
        let new_x = match direction {
            Direction::East => self.x.checked_add(1).filter(|&x| x < x_size),
            Direction::West => self.x.checked_sub(1),
            _ => Some(self.x),
        };

        let new_y = match direction {
            Direction::North => self.y.checked_sub(1),
            Direction::South => self.y.checked_add(1).filter(|&y| y < y_size),
            _ => Some(self.y),
        };

        match (new_x, new_y) {
            (Some(x), Some(y)) => Some(Point { x, y }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East  => Direction::West,
            Direction::South => Direction::North,
            Direction::West  => Direction::East,
        }
    }
}

fn possible_moves(square: char) -> Vec<Direction> {
    match square {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::West, Direction::East],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::West, Direction::North],
        '7' => vec![Direction::West, Direction::South],
        'F' => vec![Direction::South, Direction::East],
        'S' => vec![
            Direction::North,
            Direction::East, 
            Direction::South, 
            Direction::West, 
        ],
         _  => vec![],
    }
}

fn can_be_entered_going(square: char, dir: &Direction) -> bool {
    let can_be_entered: Vec<Direction> = possible_moves(square)
        .iter()
        .map(|d| d.opposite())
        .collect();

    can_be_entered.contains(dir)
}


fn farthest_point_distance(grid: &[Vec<char>], start: Point) -> usize {
    let mut distances: HashMap<Point, usize> = HashMap::new();
    let mut parents: HashMap<Point, Point> = HashMap::new();
    let mut queue: VecDeque<Point> = VecDeque::new();

    println!("Start: {:?}", start);

    queue.push_back(start.clone());
    distances.insert(start, 0);

    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];

        let neighbours: Vec<Point> = possible_moves(grid[current.y][current.x])
            .iter()
            .map(|d| (d, current.move_in(d, grid[current.y].len(), grid.len())))
            .filter(|(d, point)| point.map_or(false, |p| can_be_entered_going(grid[p.y][p.x], d)))
            .map(|(_, p)| p)
            .flatten()
            .collect();

        for next in neighbours {
            let next_distance = current_distance + 1;

            if !distances.contains_key(&next) || next_distance < distances[&next] {
                distances.insert(next.clone(), next_distance);
                parents.insert(next.clone(), current.clone());
                queue.push_back(next);
            } else if parents[&current] != next {
                return distances[&next];
            }
        }
    }

    distances.values().cloned().max().unwrap_or(0)
}

fn main() {
    let sketch = include_str!("day10_data.txt")
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .filter(|line| line.len() > 0)
        .collect::<Vec<Vec<char>>>();

    println!("{:?}", sketch);

    let mut start_point = Point { x: 0, y: 0 };

    // Find the starting position
    for (i, row) in sketch.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start_point = Point { x: j, y: i };
                break;
            }
        }
    }

    let steps = farthest_point_distance(&sketch, start_point);

    println!("Steps along the loop to farthest point: {}", steps);
}


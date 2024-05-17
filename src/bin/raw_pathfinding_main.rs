use ndarray::Array2;
use pathfinding::prelude::*;

// Define the function to convert the grid to a pathfinding-compatible format
fn neighbors(pos: (usize, usize), grid: &Array2<i32>) -> Vec<((usize, usize), i32)> {
    let (x, y) = pos;
    let mut result = Vec::new();

    let directions = [
        (1, 0),  // Right
        (0, 1),  // Down
        (-1, 0), // Left
        (0, -1), // Up
    ];

    for &(dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < grid.nrows() as isize && ny >= 0 && ny < grid.ncols() as isize {
            let cost = grid[(nx as usize, ny as usize)];
            result.push(((nx as usize, ny as usize), cost));
        }
    }

    result
}

// Define the heuristic function using the Manhattan distance
fn heuristic(a: (usize, usize), b: (usize, usize)) -> i32 {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as i32
}

fn main() {
    // Create a 2D ndarray grid
    let grid: Array2<i32> = Array2::from_shape_vec(
        (5, 5),
        vec![
            1, 1, 1, 1, 1, //
            1, 5, 5, 5, 1, //
            1, 5, 1, 1, 1, //
            1, 1, 1, 5, 1, //
            1, 1, 1, 1, 1, //
        ],
    )
    .unwrap();

    // Define the start and end points
    let start = (0, 0);
    let goal = (4, 4);

    // Run A-star algorithm to find the shortest path
    let result = astar(
        &start,
        |&pos| neighbors(pos, &grid),
        |&pos| heuristic(pos, goal),
        |&pos| pos == goal,
    );

    match result {
        Some((path, cost)) => {
            println!("Found a path of cost {}: {:?}", cost, path);
        }
        None => {
            println!("No path found");
        }
    }
}

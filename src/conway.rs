use std::collections::HashSet;

fn step(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    if grid.is_empty() || grid[0].is_empty() {
        return grid.clone();
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut next_grid = vec![vec![false; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let mut neighbor_count = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                        if grid[ni as usize][nj as usize] {
                            neighbor_count += 1;
                        }
                    }
                }
            }
            next_grid[i][j] = (grid[i][j] && (neighbor_count == 2 || neighbor_count == 3))
                || (!grid[i][j] && neighbor_count == 3);
        }
    }

    next_grid
}

pub fn game_of_life(grid: &Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let mut current_grid = grid.clone();
    let mut seen = HashSet::new();
    let mut history = Vec::new();

    while seen.insert(current_grid.clone()) {
        history.push(current_grid.clone());
        current_grid = step(&current_grid);
    }

    history
}
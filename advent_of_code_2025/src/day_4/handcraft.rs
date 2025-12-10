pub struct SolutionDay4;
use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Cell {
    x: usize, // i
    y: usize, // j
    cell_type: char,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}): {}", self.x, self.y, self.cell_type)
    }
}

impl SolutionDay4 {
    fn read_input(filename: &str) -> Vec<String> {
        let input = fs::read_to_string(filename).unwrap();
        input.lines().map(|s| s.to_string()).collect()
    }

    fn build_adjacency_list(grid: &Vec<Vec<char>>) -> HashMap<Cell, Vec<Cell>> {
        let mut adjacency_list: HashMap<Cell, Vec<Cell>> = HashMap::new();

        let grid_height: i32 = grid.len() as i32;
        let grid_width: i32 = grid[0].len() as i32;

        for (row, row_line) in grid.iter().enumerate() {
            for (col, c) in row_line.into_iter().enumerate() {
                // put current cell into the adjacency list
                let current_cell = Cell {
                    x: row,
                    y: col,
                    cell_type: *c,
                };
                if adjacency_list.get(&current_cell).is_none() {
                    adjacency_list.insert(current_cell, Vec::new());
                }

                match c {
                    '.' => continue,
                    '@' => {
                        for i in 0..=2 {
                            for j in 0..=2 {
                                let row_index: i32 = row as i32 + i as i32 - 1;
                                let col_index: i32 = col as i32 + j as i32 - 1;

                                if row_index < 0
                                    || col_index < 0
                                    || row_index >= grid_height
                                    || col_index >= grid_width
                                    || (i == 1 && j == 1)
                                {
                                    // out of bounds
                                    continue;
                                }

                                match adjacency_list.get_mut(&Cell {
                                    x: row_index as usize,
                                    y: col_index as usize,
                                    cell_type: grid[row_index as usize][col_index as usize].clone(),
                                }) {
                                    Some(cells) => cells.push(current_cell),
                                    None => {
                                        adjacency_list.insert(
                                            Cell {
                                                x: row_index as usize,
                                                y: col_index as usize,
                                                cell_type: grid[row_index as usize]
                                                    [col_index as usize]
                                                    .clone(),
                                            },
                                            vec![current_cell],
                                        );
                                    }
                                };
                            }
                        }
                    }
                    _ => panic!("Invalid character: {}", c),
                }
            }
        }

        adjacency_list
    }

    pub fn solve_1(filename: &str) -> usize {
        let input = Self::read_input(filename);

        let grid = input
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let adjacency_list: HashMap<Cell, Vec<Cell>> = Self::build_adjacency_list(&grid);

        // find out all the cells that have less than 4 neighbors
        let mut accessible_cells_count = 0;
        for (cell, neighbors) in adjacency_list.iter() {
            if cell.cell_type == '@' && neighbors.len() < 4 {
                accessible_cells_count += 1;
            }
        }

        accessible_cells_count
    }

    pub fn solve_2(filename: &str) -> u32 {
        let input = Self::read_input(filename);

        let grid = input
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let grid_height: i32 = grid.len() as i32;
        let grid_width: i32 = grid[0].len() as i32;

        let mut adjacency_list: HashMap<Cell, Vec<Cell>> = Self::build_adjacency_list(&grid);

        let mut accessible_cells = Vec::new();
        // find out all the cells that have less than 4 neighbors
        for (cell, neighbors) in adjacency_list.iter() {
            if cell.cell_type == '@' && neighbors.len() < 4 {
                accessible_cells.push(cell.clone());
            }
        }

        let mut total_removed_cells = 0;
        loop {
            let current_accessible_cells = accessible_cells.pop();
            if current_accessible_cells.is_none() {
                break;
            }
            let current_accessible_cell = current_accessible_cells.unwrap();

            if current_accessible_cell.cell_type != '@' {
                continue;
            }

            // remove myself from the adjacency list
            // check if this cell has been removed already
            if adjacency_list.get(&current_accessible_cell).is_none() {
                continue;
            }

            adjacency_list.remove(&current_accessible_cell);
            total_removed_cells += 1;

            // remove this cell from all the 8 neighbors
            for i in 0..=2 {
                for j in 0..=2 {
                    let row_index: i32 = current_accessible_cell.x as i32 + i as i32 - 1;
                    let col_index: i32 = current_accessible_cell.y as i32 + j as i32 - 1;

                    if row_index < 0
                        || col_index < 0
                        || row_index >= grid_height
                        || col_index >= grid_width
                        || (i == 1 && j == 1)
                    {
                        // out of bounds
                        continue;
                    }

                    let neighbor_cell = Cell {
                        x: row_index as usize,
                        y: col_index as usize,
                        cell_type: grid[row_index as usize][col_index as usize].clone(),
                    };

                    if let Some(neighbors) = adjacency_list.get_mut(&neighbor_cell) {
                        // remove current cell from the neighbor's adjacency list
                        neighbors.retain(|cell| cell != &current_accessible_cell);

                        // check the number of neighbors of the neighbor
                        if neighbors.len() < 4 {
                            accessible_cells.push(neighbor_cell);
                        }
                    }
                }
            }
        }

        total_removed_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay4::solve_1("src/day_4/input_test.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay4::solve_2("src/day_4/input_test.txt");
        assert_eq!(result, 43);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay4::solve_1("src/day_4/input.txt");
        assert_eq!(result, 1533);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay4::solve_2("src/day_4/input.txt");
        assert_eq!(result, 9206);
    }
}

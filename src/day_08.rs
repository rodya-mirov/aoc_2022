fn input() -> String {
    std::fs::read_to_string("input/input_08.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let height_grid: Grid<i32> = parse_tree_map(input);

    let num_rows = height_grid.num_rows;
    let num_cols = height_grid.num_cols;

    let mut seen_grid: Grid<bool> = Grid::new(vec![false; num_rows * num_cols], num_rows, num_cols);

    fn maybe_tag_seen(
        running_max: &mut i32,
        height_grid: &Grid<i32>,
        seen_grid: &mut Grid<bool>,
        col: usize,
        row: usize,
    ) {
        let height = height_grid.get(col, row);
        if height > *running_max {
            *running_max = height;
            seen_grid.set(col, row, true);
        }
    }

    // it hurts me that i wrote out each direction manually
    // but i feel lazy about making more reusable code? it only took like 10 minutes :thinking:

    // sweep left to right in each row
    for row in 0..num_rows {
        seen_grid.set(0, row, true);
        let mut running_max = height_grid.get(0, row);
        for col in 1..num_cols {
            maybe_tag_seen(&mut running_max, &height_grid, &mut seen_grid, col, row);
        }
    }

    // sweep right to left in each row
    for row in 0..num_rows {
        seen_grid.set(num_cols - 1, row, true);
        let mut running_max = height_grid.get(num_cols - 1, row);
        for col in (0..num_cols - 1).rev() {
            maybe_tag_seen(&mut running_max, &height_grid, &mut seen_grid, col, row);
        }
    }

    // sweep top to bottom in each column
    for col in 0..num_cols {
        seen_grid.set(col, 0, true);
        let mut running_max = height_grid.get(col, 0);
        for row in 1..num_rows {
            maybe_tag_seen(&mut running_max, &height_grid, &mut seen_grid, col, row);
        }
    }

    // sweep bottom to top in each column
    for col in 0..num_cols {
        seen_grid.set(col, num_rows - 1, true);
        let mut running_max = height_grid.get(col, num_rows - 1);
        for row in (0..num_rows - 1).rev() {
            maybe_tag_seen(&mut running_max, &height_grid, &mut seen_grid, col, row);
        }
    }

    seen_grid.data.iter().copied().filter(|b| *b).count()
}

struct Grid<T> {
    num_rows: usize,
    num_cols: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn new(data: Vec<T>, num_rows: usize, num_cols: usize) -> Grid<T> {
        assert_eq!(num_rows * num_cols, data.len());
        Grid {
            num_cols,
            num_rows,
            data,
        }
    }
}

impl<T: Copy> Grid<T> {
    fn get(&self, col: usize, row: usize) -> T {
        assert!(col < self.num_cols);
        assert!(row < self.num_rows);

        self.data[row * self.num_cols + col]
    }

    fn set(&mut self, col: usize, row: usize, val: T) {
        assert!(col < self.num_cols);
        assert!(row < self.num_rows);

        self.data[row * self.num_cols + col] = val;
    }
}

fn char_to_int(c: char) -> i32 {
    return ((c as usize) - ('0' as usize)) as i32;
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let height_grid = parse_tree_map(input);

    let mut best = 0;

    // O(n^3) where n is the side of the grid (assuming grid is nearly square)
    for row in 1..height_grid.num_rows - 1 {
        for col in 1..height_grid.num_cols - 1 {
            let score = scenic_score(&height_grid, col, row);
            best = best.max(score);
        }
    }

    best
}

// O(n) where n is the side of the grid (assuming grid is nearly square)
fn scenic_score(height_map: &Grid<i32>, center_col: usize, center_row: usize) -> usize {
    let left_view = left_view(&height_map, center_col, center_row);
    let right_view = right_view(&height_map, center_col, center_row);
    let up_view = up_view(&height_map, center_col, center_row);
    let down_view = down_view(&height_map, center_col, center_row);

    left_view * right_view * up_view * down_view
}

// yes, i split this into its own function to play "find the typo" with unit tests
fn left_view(height_map: &Grid<i32>, center_col: usize, center_row: usize) -> usize {
    let starting_height = height_map.get(center_col, center_row);

    let last_left = (0..center_col)
        .rev()
        .take_while(|col| height_map.get(*col, center_row) < starting_height)
        .last()
        .unwrap_or(center_col);
    (center_col - last_left + 1).min(center_col)
}

// yes, i split this into its own function to play "find the typo" with unit tests
fn right_view(height_map: &Grid<i32>, center_col: usize, center_row: usize) -> usize {
    let starting_height = height_map.get(center_col, center_row);
    let right_edge = height_map.num_cols - 1;

    let last_right = (center_col + 1..height_map.num_cols)
        .take_while(|col| height_map.get(*col, center_row) < starting_height)
        .last()
        .unwrap_or(center_col);
    (last_right + 1 - center_col).min(right_edge - center_col)
}

// this one worked first try but the code felt asymmetric without making it a function too
fn up_view(height_map: &Grid<i32>, center_col: usize, center_row: usize) -> usize {
    let starting_height = height_map.get(center_col, center_row);

    let last_up = (0..center_row)
        .rev()
        .take_while(|row| height_map.get(center_col, *row) < starting_height)
        .last()
        .unwrap_or(center_row);
    (center_row - last_up + 1).min(center_row)
}

fn down_view(height_map: &Grid<i32>, center_col: usize, center_row: usize) -> usize {
    let starting_height = height_map.get(center_col, center_row);
    let bottom_edge = height_map.num_rows - 1;

    let last_down = (center_row + 1..height_map.num_rows)
        .take_while(|row| height_map.get(center_col, *row) < starting_height)
        .last()
        .unwrap_or(center_row);
    (last_down + 1 - center_row).min(bottom_edge - center_row)
}

fn parse_tree_map(input: &str) -> Grid<i32> {
    let num_cols = input.lines().next().unwrap().len();

    let heights: Vec<i32> = input
        .trim()
        .chars()
        // skip whitespace
        .filter(|c| c.is_ascii_digit())
        .map(char_to_int)
        .collect();

    let num_rows = heights.len() / num_cols;

    Grid::new(heights, num_rows, num_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE_INPUT_STR), 21);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE_INPUT_STR), 8);
    }

    #[test]
    fn scenic_view_test() {
        let grid = parse_tree_map(SAMPLE_INPUT_STR);

        assert_eq!(left_view(&grid, 0, 1), 0);
        assert_eq!(left_view(&grid, 1, 1), 1);
        assert_eq!(left_view(&grid, 2, 1), 1);
        assert_eq!(left_view(&grid, 3, 1), 1);
        assert_eq!(left_view(&grid, 4, 1), 2);

        assert_eq!(right_view(&grid, 0, 1), 1);
        assert_eq!(right_view(&grid, 1, 1), 1);
        assert_eq!(right_view(&grid, 2, 1), 2);
        assert_eq!(right_view(&grid, 3, 1), 1);
        assert_eq!(right_view(&grid, 4, 1), 0);

        assert_eq!(scenic_score(&grid, 2, 1), 4);
    }

    #[test]
    fn char_to_int_tests() {
        assert_eq!(char_to_int('0'), 0);
        assert_eq!(char_to_int('1'), 1);
        assert_eq!(char_to_int('2'), 2);
        assert_eq!(char_to_int('3'), 3);
        assert_eq!(char_to_int('4'), 4);
        assert_eq!(char_to_int('5'), 5);
        assert_eq!(char_to_int('6'), 6);
        assert_eq!(char_to_int('7'), 7);
        assert_eq!(char_to_int('8'), 8);
        assert_eq!(char_to_int('9'), 9);
    }
}

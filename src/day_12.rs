use std::collections::VecDeque;

fn input() -> String {
    std::fs::read_to_string("input/input_12.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let grid = parse_grid(input);
    let best_scores = dijkstra(&grid);

    let start = grid.start;
    let best_length = best_scores[start.1 as usize][start.0 as usize];

    assert!(best_length < usize::MAX);

    best_length
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let grid = parse_grid(input);
    let best_scores = dijkstra(&grid);

    (0..grid.height)
        .map(|y| {
            (0..grid.width)
                .filter(|x| grid.height_at((*x, y)) == to_height('a'))
                .map(|x| best_scores[y as usize][x as usize])
                .min()
        })
        .filter_map(|best_in_row| best_in_row)
        .min()
        .unwrap()
}

/// Given a grid, return a Vec<Vec<usize>> where out[y][x] is the length of the shortest path
/// from (x, y) to the end point
fn dijkstra(grid: &Grid) -> Vec<Vec<usize>> {
    let end = grid.end;

    let mut best_scores = vec![vec![usize::MAX; grid.width as usize]; grid.height as usize];
    let mut seen = vec![vec![false; grid.width as usize]; grid.height as usize];

    let mut to_process = VecDeque::new();
    to_process.push_back((0, end));

    while let Some((path_length, pos)) = to_process.pop_front() {
        if !seen[pos.1 as usize][pos.0 as usize] {
            best_scores[pos.1 as usize][pos.0 as usize] = path_length;
            seen[pos.1 as usize][pos.0 as usize] = true;

            let candidates = [
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
            ];
            for candidate_pos in candidates {
                if grid.contains_pos(candidate_pos) && grid.can_move_from(candidate_pos, pos) {
                    to_process.push_back((path_length + 1, candidate_pos));
                }
            }
        }
    }

    best_scores
}

fn parse_grid(input: &str) -> Grid {
    let mut lines = input.trim().lines().peekable();

    let first_line = lines.peek().unwrap();
    let width = first_line.len() as i32;

    let mut data = Vec::new();

    let mut start = None;
    let mut end = None;

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            if c == 'S' {
                start = Some((x, y));
                data.push(to_height('a'));
            } else if c == 'E' {
                end = Some((x, y));
                data.push(to_height('z'));
            } else {
                data.push(to_height(c));
            }
            x += 1;
        }
        y += 1;
    }

    Grid::new(width, y, data, start.unwrap(), end.unwrap())
}

#[inline(always)]
fn to_height(c: char) -> i32 {
    (c as i32) - ('a' as i32)
}

struct Grid {
    width: i32,
    height: i32,
    start: (i32, i32),
    end: (i32, i32),
    // guaranteed to have length width * height
    data: Vec<i32>,
}

impl Grid {
    fn new(
        width: i32,
        height: i32,
        // always from 0 to 25, actually
        data: Vec<i32>,
        start: (i32, i32),
        end: (i32, i32),
    ) -> Grid {
        assert!(width > 0);
        assert!(height > 0);
        assert_eq!(width * height, data.len() as i32);
        Grid {
            width,
            height,
            data,
            start,
            end,
        }
    }

    #[inline(always)]
    fn height_at(&self, pos: (i32, i32)) -> i32 {
        self.data[(pos.1 * self.width + pos.0) as usize]
    }

    #[inline(always)]
    fn can_move_from(&self, start_pos: (i32, i32), end_pos: (i32, i32)) -> bool {
        let height_start = self.height_at(start_pos);
        let height_end = self.height_at(end_pos);

        height_start + 1 >= height_end
    }

    fn contains_pos(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 31);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 29);
    }
}

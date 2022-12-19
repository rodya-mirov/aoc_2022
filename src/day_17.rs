use std::collections::HashMap;

fn input() -> String {
    std::fs::read_to_string("input/input_17.txt").expect("Should be able to read the file")
}

fn grab_next<T: Copy>(arr: &[T], ind: &mut usize) -> T {
    let out = arr[*ind];
    *ind = (*ind + 1) % arr.len();
    out
}

const NUM_COLUMNS: usize = 7;

type Columns = columns::Columns;

mod columns {
    use std::collections::{HashSet, VecDeque};

    use super::NUM_COLUMNS;

    const COLUMN_HEIGHT: usize = 127;

    pub(super) struct Columns {
        columns: [u128; NUM_COLUMNS],
        // everything at or below this number is known to be inaccessible, so we don't even
        // bother tracking it
        culled_height: usize,
    }

    impl Columns {
        pub(super) fn new() -> Self {
            Columns {
                // start with a rock at 0 for every column
                columns: [1; NUM_COLUMNS],
                culled_height: 0,
            }
        }

        pub(super) fn add_rock(&mut self, x: usize, y: usize) {
            let y = y - self.culled_height;
            if y >= COLUMN_HEIGHT {
                unimplemented!("Data model doesn't allow our columns to track > 64 things");
            }

            self.columns[x] = self.columns[x] | (1 << y);
        }

        pub(super) fn rock_top(&self) -> usize {
            (0..NUM_COLUMNS)
                .map(|x| {
                    (0..COLUMN_HEIGHT)
                        .filter(|y| self.columns[x] & (1 << y) != 0)
                        .last()
                        .unwrap()
                })
                .max()
                .unwrap()
                + self.culled_height
        }

        pub(super) fn is_legal(&self, x: usize, y: usize) -> bool {
            let y = y - self.culled_height;
            if y > COLUMN_HEIGHT {
                return true;
            }
            let column = self.columns[x];
            (column & (1 << y)) == 0
        }

        pub(super) fn as_hashable_data(&self) -> ([u128; NUM_COLUMNS]) {
            self.columns
        }

        pub(super) fn clean_up(&mut self) {
            // y here is in EXTERNAL coordinates (i.e. not culled down)
            let air = self.rock_top() + 1;

            let mut bottom = air;
            let mut to_process = VecDeque::new();
            let mut seen = HashSet::new();

            for x in 0..NUM_COLUMNS {
                to_process.push_back((x, air));
            }

            while let Some((x, y)) = to_process.pop_front() {
                if !self.is_legal(x, y) {
                    continue;
                }
                // deduping
                if !seen.insert((x, y)) {
                    continue;
                }

                bottom = bottom.min(y);

                if x > 0 {
                    to_process.push_back((x - 1, y));
                }
                if x < NUM_COLUMNS - 1 {
                    to_process.push_back((x + 1, y));
                }
                if y < air {
                    to_process.push_back((x, y + 1));
                }
                if y == 0 {
                    unimplemented!("No way should we have gotten to zero");
                }
                to_process.push_back((x, y - 1));
            }

            // we keep the bottom row
            self.cull(bottom - 1);
        }

        fn cull(&mut self, bottom_kept: usize) {
            if bottom_kept > self.culled_height {
                let diff = bottom_kept - self.culled_height;
                for x in 0..NUM_COLUMNS {
                    let row = self.columns.get_mut(x).unwrap();
                    *row = *row >> diff;
                }
                self.culled_height = bottom_kept;
            }
        }
    }
}

fn all_legal(columns: &Columns, arr: &[(usize, usize)]) -> bool {
    arr.iter().copied().all(|(x, y)| columns.is_legal(x, y))
}

pub fn a() -> String {
    let contents = input();

    let val = height_after_num_rocks(&contents, 2022);

    val.to_string()
}

fn make_new_shape(rock_top: usize, shape: RockShape) -> Vec<(usize, usize)> {
    match shape {
        RockShape::HorizontalLine => vec![
            (2, rock_top + 4),
            (3, rock_top + 4),
            (4, rock_top + 4),
            (5, rock_top + 4),
        ],
        RockShape::VerticalLine => vec![
            (2, rock_top + 4),
            (2, rock_top + 5),
            (2, rock_top + 6),
            (2, rock_top + 7),
        ],
        RockShape::Square => vec![
            (2, rock_top + 4),
            (2, rock_top + 5),
            (3, rock_top + 4),
            (3, rock_top + 5),
        ],
        RockShape::Angle => vec![
            (2, rock_top + 4),
            (3, rock_top + 4),
            (4, rock_top + 4),
            (4, rock_top + 5),
            (4, rock_top + 6),
        ],
        RockShape::Plus => vec![
            (2, rock_top + 5),
            (3, rock_top + 4),
            (3, rock_top + 5),
            (3, rock_top + 6),
            (4, rock_top + 5),
        ],
    }
}

fn shift_positions(
    rock_positions: &[(usize, usize)],
    next_side_direction: Dir,
) -> Option<Vec<(usize, usize)>> {
    match next_side_direction {
        Dir::Left => {
            if rock_positions.iter().copied().any(|(x, _)| x == 0) {
                None
            } else {
                Some(
                    rock_positions
                        .iter()
                        .copied()
                        .map(|(x, y)| (x - 1, y))
                        .collect(),
                )
            }
        }
        Dir::Right => {
            if rock_positions
                .iter()
                .copied()
                .any(|(x, _)| x == NUM_COLUMNS - 1)
            {
                None
            } else {
                Some(
                    rock_positions
                        .iter()
                        .copied()
                        .map(|(x, y)| (x + 1, y))
                        .collect(),
                )
            }
        }
    }
}

pub fn b() -> String {
    let contents = input();

    let val = height_after_num_rocks(&contents, 1000000000000);

    val.to_string()
}

fn height_after_num_rocks(input: &str, num_rocks: usize) -> usize {
    let directions = parse_input(input);
    let mut next_dir: usize = 0;

    let shapes = [
        RockShape::HorizontalLine,
        RockShape::Plus,
        RockShape::Angle,
        RockShape::VerticalLine,
        RockShape::Square,
    ];
    let mut shape_ind: usize = 0;

    // true means "has rock", false means "no rock"
    // start with a floor, then some empty space
    let mut columns: Columns = Columns::new();

    #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
    struct InputState {
        next_dir: usize,
        shape_ind: usize,
        column_tops: [u128; NUM_COLUMNS],
    }

    #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
    struct OutputState {
        rocks_so_far: usize,
        rock_top: usize,
    }

    let mut seen: HashMap<InputState, OutputState> = HashMap::new();
    // everything is gonna be a repeat once we find one, so jump to the end and don't jump again
    let mut cycle_finished = false;

    let mut rocks_so_far = 0;
    let mut skipped_rock_top = 0;

    while rocks_so_far < num_rocks {
        rocks_so_far += 1;
        let rock_top = columns.rock_top();

        columns.clean_up();

        if !cycle_finished {
            let input_state = InputState {
                next_dir,
                shape_ind,
                column_tops: columns.as_hashable_data(),
            };
            let output_state = OutputState {
                rock_top,
                rocks_so_far,
            };

            if let Some(old_output_state) = seen.insert(input_state, output_state) {
                let cycle_length = output_state.rocks_so_far - old_output_state.rocks_so_far;
                let cycle_value = output_state.rock_top - old_output_state.rock_top;

                let remaining_full_cycles = ((num_rocks - 1) - rocks_so_far) / cycle_length;
                rocks_so_far += remaining_full_cycles * cycle_length;
                skipped_rock_top = remaining_full_cycles * cycle_value;

                cycle_finished = true;
            }
        }

        let shape = grab_next(&shapes, &mut shape_ind);

        // x, y
        let mut rock_positions: Vec<(usize, usize)> = make_new_shape(rock_top, shape);

        loop {
            // first, do a side-shift
            let next_side_direction = grab_next(&directions, &mut next_dir);

            let shifted: Option<Vec<(usize, usize)>> =
                shift_positions(&rock_positions, next_side_direction);

            // failure is legal here
            if let Some(shifted) = shifted {
                if all_legal(&columns, &shifted) {
                    rock_positions = shifted;
                }
            }

            // then shift down
            let shifted: Vec<(usize, usize)> = rock_positions
                .iter()
                .copied()
                .map(|(x, y)| (x, y - 1))
                .collect();

            if all_legal(&columns, &shifted) {
                rock_positions = shifted;
            } else {
                break;
            }
        }

        for (x, y) in rock_positions {
            columns.add_rock(x, y);
        }
    }

    columns.rock_top() + skipped_rock_top
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum RockShape {
    HorizontalLine,
    Plus,
    Angle,
    VerticalLine,
    Square,
}

fn parse_input(input: &str) -> Vec<Dir> {
    input
        .chars()
        .map(|c| match c {
            '>' => Dir::Right,
            '<' => Dir::Left,
            _ => unimplemented!("Bad input: {}", c),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn sample_a() {
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 1), 1);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 2), 4);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 3), 6);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 4), 7);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 5), 9);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 6), 10);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 7), 13);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 8), 15);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 9), 17);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 10), 17);
        assert_eq!(height_after_num_rocks(SAMPLE_INPUT_STR, 2022), 3068);
    }
}

fn input() -> String {
    std::fs::read_to_string("input/input_17.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents, 2022);

    val.to_string()
}

fn a_with_input(input: &str, num_rocks: usize) -> usize {
    fn peek<T: Copy>(v: &[T]) -> T {
        assert!(!v.is_empty());
        v[v.len() - 1]
    }

    fn rock_top<T: Copy + Ord>(v: &[Vec<T>]) -> T {
        let len = v.len();
        (0..len).map(|i| peek(&v[i])).max().unwrap()
    }

    fn grab_next<T: Copy>(arr: &[T], ind: &mut usize) -> T {
        let out = arr[*ind];
        *ind = (*ind + 1) % arr.len();
        out
    }

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
    let mut columns: [Vec<usize>; 7] = [(); 7].map(|_| vec![0]);

    for _ in 0..num_rocks {
        let rock_top = rock_top(&columns);

        let shape = grab_next(&shapes, &mut shape_ind);

        // x, y
        let mut rock_positions: Vec<(usize, usize)> = match shape {
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
        };

        let is_legal = |x: usize, y: usize| {
            let column = &columns[x];
            for column_y in column.iter().copied().rev() {
                if column_y == y {
                    return false;
                } else if column_y < y {
                    return true;
                }
            }
            unreachable!("Somehow fucked up here, idk, I'm watching this show while I type")
        };

        let all_legal = |arr: &[(usize, usize)]| arr.iter().copied().all(|(x, y)| is_legal(x, y));

        loop {
            // first, do a side-shift
            let next_side_direction = grab_next(&directions, &mut next_dir);

            let shifted: Option<Vec<(usize, usize)>> = match next_side_direction {
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
                    if rock_positions.iter().copied().any(|(x, _)| x == 6) {
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
            };

            // failure is legal here
            if let Some(shifted) = shifted {
                if all_legal(&shifted) {
                    rock_positions = shifted;
                }
            }

            // then shift down
            let shifted: Vec<(usize, usize)> = rock_positions
                .iter()
                .copied()
                .map(|(x, y)| (x, y - 1))
                .collect();

            if all_legal(&shifted) {
                rock_positions = shifted;
            } else {
                break;
            }
        }

        for (x, y) in rock_positions {
            columns[x].push(y);
            // note this is essential -- sometimes the y you push isn't actually the highest value
            // like if you cram a plus sign in somehow
            columns[x].sort();
        }
    }

    rock_top(&columns)
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> u32 {
    unimplemented!()
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
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input, 2022);
        assert_eq!(actual, 3068);

        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 1), 1);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 2), 4);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 3), 6);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 4), 7);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 5), 9);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 6), 10);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 7), 13);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 8), 15);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 9), 17);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 10), 17);
        assert_eq!(a_with_input(SAMPLE_INPUT_STR, 2022), 3068);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, unimplemented!());
    }
}

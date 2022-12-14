use std::collections::HashSet;

fn input() -> String {
    std::fs::read_to_string("input/input_14.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let (max_y_depth, mut occupancy_grid) = to_grid(input);

    let start_count = occupancy_grid.len();

    let mut into_the_void = false;
    while !into_the_void {
        let mut x = 500;
        let mut y = 0;

        if occupancy_grid.contains(&(x, y)) {
            unimplemented!("Did not handle the case where the start position gets clogged, not sure what ACs are there");
        }

        let mut done_falling = false;
        while !done_falling {
            // needed because of overflow problems
            assert!(y < u32::MAX);
            assert!(x > 0);
            assert!(x < u32::MAX);

            // if we're into the void, we're done; exit the whole outer loop and be done
            if y > max_y_depth {
                into_the_void = true;
                done_falling = true;
            } else {
                // otherwise, attempt to move D/DL/DR and go on to another loop
                if !occupancy_grid.contains(&(x, y + 1)) {
                    y += 1;
                } else if !occupancy_grid.contains(&(x - 1, y + 1)) {
                    x -= 1;
                    y += 1;
                } else if !occupancy_grid.contains(&(x + 1, y + 1)) {
                    x += 1;
                    y += 1;
                } else {
                    // otherwise, we can't move; settle and move to the next grain of sand
                    occupancy_grid.insert((x, y));
                    done_falling = true;
                }
            }
        }
    }

    occupancy_grid.len() - start_count
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let (max_y_depth, map_grid) = to_grid(input);
    let mut actual_grid = Grid::new(map_grid);

    while !actual_grid.contains(500, 0) {
        let mut x = 500;
        let mut y = 0;

        loop {
            // needed because of overflow problems
            assert!(y < u32::MAX);
            assert!(x > 0);
            assert!(x < u32::MAX);

            // if we're into the void, we're done; exit the whole outer loop and be done
            if y > max_y_depth {
                actual_grid.try_mark_filled(x, y);
                break;
            } else {
                // otherwise, attempt to move D/DL/DR and go on to another loop
                if !actual_grid.contains(x, y + 1) {
                    y += 1;
                } else if !actual_grid.contains(x - 1, y + 1) {
                    x -= 1;
                    y += 1;
                } else if !actual_grid.contains(x + 1, y + 1) {
                    x += 1;
                    y += 1;
                } else {
                    // otherwise, we can't move; settle and move to the next grain of sand
                    actual_grid.try_mark_filled(x, y);
                    break;
                }
            }
        }
    }

    actual_grid.amt_added
}

mod parse {
    use nom::character::complete::multispace0;
    use nom::multi::separated_list1;
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, space1},
        combinator::{eof, map},
        sequence::tuple,
        IResult,
    };

    fn parse_num(input: &str) -> IResult<&str, u32> {
        let (input, val) = map(digit1, |digits: &str| digits.parse::<u32>().unwrap())(input)?;

        Ok((input, val))
    }

    fn parse_pos(input: &str) -> IResult<&str, (u32, u32)> {
        let (input, (x, _, y)) = tuple((parse_num, tag(","), parse_num))(input)?;

        Ok((input, (x, y)))
    }

    fn parse_arrow(input: &str) -> IResult<&str, ()> {
        let (input, _) = tuple((space1, tag("->"), space1))(input)?;

        Ok((input, ()))
    }

    fn parse_line_helper(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
        let (input, _) = multispace0(input)?;
        let (input, pos) = separated_list1(parse_arrow, parse_pos)(input)?;
        let (input, _) = multispace0(input)?;
        let (_, _) = eof(input)?;

        Ok(("", pos))
    }

    pub(super) fn parse_line(input: &str) -> Vec<(u32, u32)> {
        parse_line_helper(input).unwrap().1
    }

    #[cfg(test)]
    mod tests {
        use super::parse_line;

        #[test]
        fn sample_lines() {
            assert_eq!(
                parse_line("498,4 -> 498,6 -> 496,6"),
                vec![(498, 4), (498, 6), (496, 6)]
            );
            assert_eq!(
                parse_line("503,4 -> 502,4 -> 502,9 -> 494,9"),
                vec![(503, 4), (502, 4), (502, 9), (494, 9)]
            );
        }
    }
}

// returns (max_y_depth, occupancy_grid)
fn to_grid(input: &str) -> (u32, HashSet<(u32, u32)>) {
    let mut occupancy_grid = HashSet::new();
    let mut max_y_depth = 0;

    for line in input.lines() {
        let walls = parse::parse_line(line);
        assert!(!walls.is_empty());
        let mut pos_iter = walls.into_iter();
        let mut last_pos = pos_iter.next().unwrap();
        max_y_depth = max_y_depth.max(last_pos.1);

        while let Some(next_pos) = pos_iter.next() {
            if last_pos == next_pos {
                occupancy_grid.insert(last_pos);
            } else if last_pos.0 != next_pos.0 {
                assert_eq!(last_pos.1, next_pos.1);
                let x_min = last_pos.0.min(next_pos.0);
                let x_max = last_pos.0.max(next_pos.0);
                let y = last_pos.1;

                for x in x_min..x_max + 1 {
                    occupancy_grid.insert((x, y));
                }
            } else {
                assert_eq!(last_pos.0, next_pos.0);
                let y_min = last_pos.1.min(next_pos.1);
                let y_max = last_pos.1.max(next_pos.1);
                let x = last_pos.0;

                for y in y_min..y_max + 1 {
                    occupancy_grid.insert((x, y));
                }
            }

            last_pos = next_pos;
            max_y_depth = max_y_depth.max(last_pos.1);
        }
    }

    (max_y_depth, occupancy_grid)
}

struct Grid {
    x_min: u32,
    x_max: u32,
    y_max: u32,
    amt_added: usize,
    occupancy: Vec<bool>,
    width: usize,
}

impl Grid {
    fn new(set: HashSet<(u32, u32)>) -> Grid {
        let mut y_max = 0;

        for (_, y) in set.iter().copied() {
            y_max = y_max.max(y);
        }

        // maximum occupy-able position is 1 more than observed occupied position
        y_max += 1;

        // we don't actually care about tracking rocks we can never interact with; sand can only
        // go left or right as far as the down allows
        let x_min = 500 - y_max;
        let x_max = 500 + y_max;

        let height = (y_max + 1) as usize;
        let width = (x_max - x_min + 1) as usize;
        let data_size = width * height;

        let mut grid = Grid {
            x_min,
            x_max,
            y_max,
            occupancy: vec![false; data_size],
            amt_added: 0,
            width,
        };

        for (x, y) in set {
            grid.try_mark_filled(x, y);
        }

        grid.amt_added = 0;

        grid
    }

    fn is_in_grid(&self, x: u32, y: u32) -> bool {
        x >= self.x_min && x <= self.x_max && y <= self.y_max
    }

    // turn a position into an index into the occupancy array
    // PRE: self.is_in_grid(x, y) is true
    fn make_ind(&self, x: u32, y: u32) -> usize {
        self.width * (y as usize) + ((x - self.x_min) as usize)
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        self.is_in_grid(x, y) && self.occupancy[self.make_ind(x, y)]
    }

    fn try_mark_filled(&mut self, x: u32, y: u32) -> bool {
        if self.is_in_grid(x, y) {
            let ind = self.make_ind(x, y);
            self.occupancy[ind] = true;
            self.amt_added += 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 24);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 93);
    }
}

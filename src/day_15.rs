use std::collections::HashSet;

fn input() -> String {
    std::fs::read_to_string("input/input_15.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents, 2000000);

    val.to_string()
}

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn a_with_input(input: &str, row: i32) -> usize {
    let input = parse::parse_input(input);

    let mut blocked = occlusion::Occlusion::new(i32::MIN, i32::MAX);

    for (sensor_x, sensor_y, beacon_x, beacon_y) in input.iter().copied() {
        let sensor_pos = (sensor_x, sensor_y);
        let beacon_pos = (beacon_x, beacon_y);

        // anything with distance at or under this is NOT a beacon
        let min_dist = dist(sensor_pos, beacon_pos);

        let y_dist = (row - sensor_y).abs();

        if y_dist > min_dist {
            continue;
        }

        let rem_x_dist = min_dist - y_dist;
        let x_min = sensor_x - rem_x_dist;
        let x_max = sensor_x + rem_x_dist;

        blocked.block_interval(x_min, x_max + 1);
    }

    let sensors_in_row: HashSet<i32> = input
        .iter()
        .filter(|(_, _, _, y)| *y == row)
        .map(|(_, _, x, _)| *x)
        .collect();

    blocked.blocked_len() - sensors_in_row.len()
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents, 0, 4000000 + 1);

    val.to_string()
}

// true_x_min: inclusive
// true_x_max: exclusive
fn b_with_input(input: &str, true_x_min: i32, true_x_max: i32) -> usize {
    assert!(true_x_min >= 0);
    assert!(true_x_max > true_x_min);

    let mut occlusions = Vec::with_capacity(true_x_max as usize);
    for _ in 0..true_x_max {
        occlusions.push(occlusion::Occlusion::new(true_x_min, true_x_max));
    }

    let input = parse::parse_input(input);

    for (sensor_x, sensor_y, beacon_x, beacon_y) in input.iter().copied() {
        let sensor_pos = (sensor_x, sensor_y);
        let beacon_pos = (beacon_x, beacon_y);

        let base_dist = dist(sensor_pos, beacon_pos);

        debug_assert!(base_dist >= 0);

        let y_min = (sensor_y - base_dist).max(true_x_min);
        let y_max = (sensor_y + base_dist).min(true_x_max - 1);

        for y in y_min..y_max + 1 {
            let x_dist_max = base_dist - (sensor_y - y).abs();
            debug_assert!(x_dist_max >= 0);
            let x_min = sensor_x - x_dist_max;
            let x_max = sensor_x + x_dist_max;

            occlusions[y as usize].block_interval(x_min, x_max + 1);
        }
    }

    for y in 0..true_x_max + 1 {
        let occ = occlusions.get(y as usize);
        if let Some(x) = occ.unwrap().any_unblocked_space() {
            return (x as usize * 4000000) + (y as usize);
        }
    }

    unreachable!("No solution found; all rows fully blocked");
}

mod occlusion {
    pub(super) struct Occlusion {
        // CONTRACT: if i is a valid index, then blocks[i].0 < blocks[i].1
        // CONTRACT: if i and i+1 are valid indices, then blocks[i].1 < blocks[i+1].0
        // interpretation: if i is a valid index, then blocks[i] is a half-open interval [x_min, x_max)
        //      which indicates that the (half-open) range x_min .. x_max is blocked
        blocks: Vec<(i32, i32)>,
        // reusable temp space. Do whatever you want with it.
        // CONTRACT: after execution of any function, this should be empty
        backup_data: Vec<(i32, i32)>,

        // only x >= this is considered
        true_x_min: i32,
        // only x < this is considered
        true_x_max: i32,
    }

    impl Occlusion {
        pub fn new(true_x_min: i32, true_x_max: i32) -> Occlusion {
            Self {
                blocks: vec![],
                backup_data: vec![],
                true_x_min,
                true_x_max,
            }
        }

        pub fn block_interval(&mut self, mut x_min_new: i32, mut x_max_new: i32) {
            assert!(x_min_new < x_max_new);

            // clamp the arguments to the bounds of interest, so we don't waste time + space
            // by checking on territory we don't care about
            x_min_new = x_min_new.max(self.true_x_min);
            x_max_new = x_max_new.min(self.true_x_max);

            // we'll construct the new state of self.blocks into this vector, then swap them
            self.backup_data.clear();

            let mut running_interval = (x_min_new, x_max_new);

            for (x_min_old, x_max_old) in self.blocks.iter().copied() {
                let (x_min_new, x_max_new) = running_interval;

                // if the old interval is completely before the running interval,
                // then insert the old interval and keep the running interval unchanged
                if x_max_old < x_min_new {
                    self.backup_data.push((x_min_old, x_max_old));
                }
                // if the running interval is completely before the old interval, insert the
                // running interval and make the old interval the running interval
                else if x_max_new < x_min_old {
                    self.backup_data.push((x_min_new, x_max_new));
                    running_interval = (x_min_old, x_max_old);
                }
                // otherwise the intervals overlap; let the running interval be their union
                else {
                    running_interval = (x_min_old.min(x_min_new), x_max_old.max(x_max_new));
                }
            }

            // we ALWAYS have a running interval left over
            self.backup_data.push(running_interval);

            std::mem::swap(&mut self.blocks, &mut self.backup_data);

            self.backup_data.clear();
        }

        pub fn blocked_len(&self) -> usize {
            self.blocks
                .iter()
                .map(|(x_min, x_max)| (x_max - x_min) as usize)
                .sum()
        }

        pub fn any_unblocked_space(&self) -> Option<i32> {
            let mut iter = self.blocks.iter().copied();

            let first = iter.next();

            if first.is_none() {
                return Some(self.true_x_min);
            }

            let (xmin_a, xmax_a) = first.unwrap();

            if self.true_x_min < xmin_a {
                Some(self.true_x_min)
            } else if xmax_a < self.true_x_max {
                Some(xmax_a)
            } else {
                None
            }
        }
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{eof, map},
        IResult,
    };

    fn parse_num(input: &str) -> IResult<&str, i32> {
        if input.starts_with("-") {
            let (input, _) = tag("-")(input)?;
            let (input, num) = map(digit1, |digits: &str| digits.parse::<i32>().unwrap())(input)?;
            Ok((input, -num))
        } else {
            let (input, val) = map(digit1, |digits: &str| digits.parse::<i32>().unwrap())(input)?;

            Ok((input, val))
        }
    }

    fn parse_line_helper(input: &str) -> IResult<&str, (i32, i32, i32, i32)> {
        let (input, _) = tag("Sensor at x=")(input)?;
        let (input, x1) = parse_num(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, y1) = parse_num(input)?;
        let (input, _) = tag(": closest beacon is at x=")(input)?;
        let (input, x2) = parse_num(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, y2) = parse_num(input)?;
        let (_, _) = eof(input)?;

        Ok(("", (x1, y1, x2, y2)))
    }

    fn parse_line(input: &str) -> (i32, i32, i32, i32) {
        parse_line_helper(input).unwrap().1
    }

    pub(super) fn parse_input(input: &str) -> Vec<(i32, i32, i32, i32)> {
        input.lines().map(parse_line).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::parse_line;

        #[test]
        fn sample_lines() {
            assert_eq!(
                parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
                (2, 18, -2, 15)
            );
            assert_eq!(
                parse_line("Sensor at x=14, y=17: closest beacon is at x=10, y=16"),
                (14, 17, 10, 16)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input, 10);
        assert_eq!(actual, 26);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input, 0, 20 + 1);
        assert_eq!(actual, 56000011);
    }
}

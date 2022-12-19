use std::collections::{HashSet, VecDeque};

fn input() -> String {
    std::fs::read_to_string("input/input_18.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let input = parse::parse_input(input);

    let mut seen = HashSet::new();

    for (x, y, z) in input.iter().copied() {
        seen.insert((x, y, z));
    }

    let mut faces_exposed = 0;
    for (x, y, z) in input.iter().copied() {
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let new_pos = (x + dx, y + dy, z + dz);
            if !seen.contains(&new_pos) {
                faces_exposed += 1;
            }
        }
    }

    faces_exposed
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let input = parse::parse_input(input);

    let mut filled_voxels = HashSet::new();

    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    let mut z_min = i32::MAX;
    let mut z_max = i32::MIN;

    // track the filled voxel points AND the convex hull of the whole thing
    for (x, y, z) in input.iter().copied() {
        filled_voxels.insert((x, y, z));
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
        z_min = z_min.min(z);
        z_max = z_max.max(z);
    }

    // add the most obvious OOB hull (just a cube shell) around the droplet
    // then we'll spread inward until we hit the rocks
    let is_oob =
        |x, y, z| x < x_min || x > x_max || y < y_min || y > y_max || z < z_min || z > z_max;

    let mut is_exposed_to_air = HashSet::new();

    let mut to_process = VecDeque::new();
    for x in x_min - 1..x_max + 2 {
        for y in y_min - 1..y_max + 2 {
            for z in z_min - 1..z_max + 2 {
                if is_oob(x, y, z) {
                    to_process.push_back((x, y, z));
                }
            }
        }
    }

    while let Some((x, y, z)) = to_process.pop_front() {
        if is_exposed_to_air.contains(&(x, y, z)) {
            continue;
        }

        is_exposed_to_air.insert((x, y, z));

        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let new_pos = (x + dx, y + dy, z + dz);
            if !is_oob(new_pos.0, new_pos.1, new_pos.2) && !filled_voxels.contains(&new_pos) {
                to_process.push_back(new_pos);
            }
        }
    }

    // now we know which voxel points are exposed to the outer air, so we can just count
    let mut faces_exposed = 0;
    for (x, y, z) in input.iter().copied() {
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let new_pos = (x + dx, y + dy, z + dz);
            if is_exposed_to_air.contains(&new_pos) {
                faces_exposed += 1;
            }
        }
    }

    faces_exposed
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

    fn parse_line_helper(input: &str) -> IResult<&str, (i32, i32, i32)> {
        let (input, x) = parse_num(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = parse_num(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, z) = parse_num(input)?;
        let (_, _) = eof(input)?;

        Ok(("", (x, y, z)))
    }

    fn parse_line(input: &str) -> (i32, i32, i32) {
        parse_line_helper(input).unwrap().1
    }

    pub(super) fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
        input.lines().map(parse_line).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::parse_line;

        #[test]
        fn sample_lines() {
            assert_eq!(parse_line("1,2,2"), (1, 2, 2));
            assert_eq!(parse_line("3,2,115"), (3, 2, 115));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 64);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 58);
    }
}

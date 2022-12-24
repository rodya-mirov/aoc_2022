use std::collections::{HashMap, HashSet};

fn input() -> String {
    std::fs::read_to_string("input/input_23.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let mut elves = parse::parse_input(input);
    let num_elves = elves.len();

    let mut dirs = vec![Dir::N, Dir::S, Dir::W, Dir::E];

    // elf pos -> elf desired pos
    let mut desired_moves: HashMap<Pos, Pos> = HashMap::new();

    // desired pos -> number of elves wanting it
    let mut dest_reverse_lookup: HashMap<Pos, usize> = HashMap::new();

    for _ in 0..10 {
        desired_moves.clear();
        dest_reverse_lookup.clear();

        // pick directions
        for elf in elves.iter().copied() {
            let can_move_north = [elf.north().west(), elf.north(), elf.north().east()]
                .iter()
                .all(|p| !elves.contains(p));
            let can_move_south = [elf.south().west(), elf.south(), elf.south().east()]
                .iter()
                .all(|p| !elves.contains(p));
            let can_move_west = [elf.west().north(), elf.west(), elf.west().south()]
                .iter()
                .all(|p| !elves.contains(p));
            let can_move_east = [elf.east().north(), elf.east(), elf.east().south()]
                .iter()
                .all(|p| !elves.contains(p));

            if can_move_north && can_move_south && can_move_west && can_move_east {
                // elf is content, stop moving
                continue;
            }

            let mut desired_move: Option<Pos> = None;
            for dir in dirs.iter().copied() {
                desired_move = match dir {
                    Dir::N => Some(elf.north()).filter(|_| can_move_north),
                    Dir::E => Some(elf.east()).filter(|_| can_move_east),
                    Dir::W => Some(elf.west()).filter(|_| can_move_west),
                    Dir::S => Some(elf.south()).filter(|_| can_move_south),
                };
                if desired_move.is_some() {
                    break;
                }
            }

            if let Some(desired_move) = desired_move {
                desired_moves.insert(elf, desired_move);
                *dest_reverse_lookup.entry(desired_move).or_insert(0) += 1;
            }
        }

        // apply changes
        let mut new_elves = HashSet::new();
        for elf in elves {
            let mut moved = false;

            if let Some(desired_pos) = desired_moves.get(&elf).copied() {
                if dest_reverse_lookup.get(&desired_pos).copied().unwrap() == 1 {
                    new_elves.insert(desired_pos);
                    moved = true;
                }
            }

            if !moved {
                new_elves.insert(elf);
            }
        }
        elves = new_elves;

        dirs.rotate_left(1);
    }

    assert_eq!(elves.len(), num_elves);

    let mut xmin = i64::MAX;
    let mut xmax = i64::MIN;
    let mut ymin = i64::MAX;
    let mut ymax = i64::MIN;

    for Pos(x, y) in elves {
        xmin = xmin.min(x);
        xmax = xmax.max(x);
        ymin = ymin.min(y);
        ymax = ymax.max(y);
    }

    let w = (xmax - xmin + 1) as usize;
    let h = (ymax - ymin + 1) as usize;

    w * h - num_elves
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos(i64, i64);

impl Pos {
    fn west(self) -> Self {
        Self(self.0 - 1, self.1)
    }

    fn east(self) -> Self {
        Self(self.0 + 1, self.1)
    }

    fn north(self) -> Self {
        Self(self.0, self.1 - 1)
    }

    fn south(self) -> Self {
        Self(self.0, self.1 + 1)
    }
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let mut elves = parse::parse_input(input);

    let mut dirs = vec![Dir::N, Dir::S, Dir::W, Dir::E];

    // elf pos -> elf desired pos
    let mut desired_moves: HashMap<Pos, Pos> = HashMap::new();

    // desired pos -> number of elves wanting it
    let mut dest_reverse_lookup: HashMap<Pos, usize> = HashMap::new();

    let mut rounds = 0;

    loop {
        rounds += 1;
        desired_moves.clear();
        dest_reverse_lookup.clear();

        // pick directions
        for elf in elves.iter().copied() {
            let can_move_north = [elf.north().west(), elf.north(), elf.north().east()]
                .iter()
                .all(|p| !elves.contains(p));
            let can_move_south = [elf.south().west(), elf.south(), elf.south().east()]
                .iter()
                .all(|p| !elves.contains(p));
            let can_move_west = [elf.west().north(), elf.west(), elf.west().south()]
                .iter()
                .all(|p| !elves.contains(p));
            let can_move_east = [elf.east().north(), elf.east(), elf.east().south()]
                .iter()
                .all(|p| !elves.contains(p));

            if can_move_north && can_move_south && can_move_west && can_move_east {
                // elf is content, stop moving
                continue;
            }

            let mut desired_move: Option<Pos> = None;
            for dir in dirs.iter().copied() {
                desired_move = match dir {
                    Dir::N => Some(elf.north()).filter(|_| can_move_north),
                    Dir::E => Some(elf.east()).filter(|_| can_move_east),
                    Dir::W => Some(elf.west()).filter(|_| can_move_west),
                    Dir::S => Some(elf.south()).filter(|_| can_move_south),
                };
                if desired_move.is_some() {
                    break;
                }
            }

            if let Some(desired_move) = desired_move {
                desired_moves.insert(elf, desired_move);
                *dest_reverse_lookup.entry(desired_move).or_insert(0) += 1;
            }
        }

        let mut any_moved = false;

        // apply changes
        let mut new_elves = HashSet::new();
        for elf in elves {
            let mut moved = false;

            if let Some(desired_pos) = desired_moves.get(&elf).copied() {
                if dest_reverse_lookup.get(&desired_pos).copied().unwrap() == 1 {
                    new_elves.insert(desired_pos);
                    moved = true;
                    any_moved = true;
                }
            }

            if !moved {
                new_elves.insert(elf);
            }
        }
        elves = new_elves;

        dirs.rotate_left(1);

        if !any_moved {
            return rounds;
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

mod parse {
    use std::collections::HashSet;

    use super::Pos;

    pub(super) fn parse_input(input: &str) -> HashSet<Pos> {
        let mut out = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        out.insert(Pos(x as i64, y as i64));
                    }
                    '.' => {}
                    _ => unimplemented!("Unknown character {}", c),
                }
            }
        }
        out
    }

    #[cfg(test)]
    mod parse_test {
        use super::*;

        const SAMPLE: &'static str = ".....
..##.
..#..
.....
..##.
.....";

        #[test]
        fn parse_test() {
            let actual = parse_input(SAMPLE);
            let mut expected = HashSet::new();

            expected.insert(Pos(2, 1));
            expected.insert(Pos(3, 1));
            expected.insert(Pos(2, 2));
            expected.insert(Pos(2, 4));
            expected.insert(Pos(3, 4));

            assert_eq!(expected, actual);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_SAMPLE_INPUT_STR: &'static str = ".....
..##.
..#..
.....
..##.
.....";

    const BIGGER_SAMPLE_INPUT_STR: &'static str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn sample_a() {
        let input = SMALL_SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 25);
    }

    #[test]
    fn sample_a2() {
        let input = BIGGER_SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 110);
    }

    #[test]
    fn sample_b() {
        let input = BIGGER_SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 20);
    }
}

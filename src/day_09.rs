use std::collections::HashSet;

fn input() -> String {
    std::fs::read_to_string("input/input_09.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    move_with_input::<2>(input)
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    move_with_input::<10>(input)
}

fn move_with_input<const LENGTH: usize>(input: &str) -> usize {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut positions: [(i32, i32); LENGTH] = [(0, 0); LENGTH];

    seen.insert(positions[LENGTH - 1]);

    fn far(a: (i32, i32), b: (i32, i32)) -> bool {
        (a.0 - b.0).abs() > 1 || (a.1 - b.1).abs() > 1
    }

    for Move { dir, amt } in input.lines().map(|line| parse_move(line)) {
        for _ in 0..amt {
            match dir {
                Dir::D => positions[0].1 -= 1,
                Dir::U => positions[0].1 += 1,
                Dir::L => positions[0].0 -= 1,
                Dir::R => positions[0].0 += 1,
            };

            for i in 0..LENGTH - 1 {
                let head_pos = positions[i];
                let tail_pos = &mut positions[i + 1];
                while far(head_pos, *tail_pos) {
                    tail_pos.0 += (head_pos.0 - tail_pos.0).signum();
                    tail_pos.1 += (head_pos.1 - tail_pos.1).signum();

                    if i + 1 == LENGTH - 1 {
                        seen.insert(*tail_pos);
                    }
                }
            }
        }
    }

    seen.len()
}

fn parse_move(line: &str) -> Move {
    let dir = parse_dir(line.chars().next().unwrap());
    // look, no error handling, it's a puzzle not enterprise software
    let amt_str = line.chars().skip(2).collect::<String>();
    let amt = amt_str.parse::<i32>().unwrap();
    Move { dir, amt }
}

fn parse_dir(c: char) -> Dir {
    match c {
        'R' => Dir::R,
        'U' => Dir::U,
        'L' => Dir::L,
        'D' => Dir::D,
        other => unimplemented!("Bad direction input: {}", other),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    R,
    U,
    L,
    D,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Move {
    dir: Dir,
    amt: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const SECOND_SAMPLE: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE_INPUT_STR), 13);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input(SAMPLE_INPUT_STR), 1);
        assert_eq!(b_with_input(SECOND_SAMPLE), 36);
    }
}

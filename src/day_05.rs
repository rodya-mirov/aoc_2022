use itertools::Itertools;
use std::collections::VecDeque;

fn input() -> String {
    std::fs::read_to_string("input/input_05.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> String {
    let mut lines = input.lines();

    let mut stacks = parse_stacks(&mut lines);

    for next_move in lines.map(|line| parse_move(line)) {
        apply_move_a(&mut stacks, next_move);
    }

    make_output(&stacks)
}

fn make_output(stacks: &[VecDeque<char>]) -> String {
    let mut out_str = String::new();

    for stack in stacks {
        if let Some(c) = stack.get(0) {
            out_str.push(*c);
        }
    }

    out_str
}

// parses the stacks and leaves the iterator in a state where the next line is a MOVE line
fn parse_stacks<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    // first, parse the stacks
    loop {
        let Some(line) = lines.next() else {
            panic!("unexpected EOF");
        };

        if !line.contains('[') {
            // we don't actually need to do anything with the ' 1   2   3   4 ' line
            // so skip that one and the blank line following
            lines.next().expect("Should have a blank line");
            break;
        }

        let mut column = 0;
        for token in &line.chars().chunks(4) {
            while stacks.len() <= column {
                stacks.push(VecDeque::new());
            }

            // we know (don't validate) that each column is 0 or 1 character
            if let Some(c) = token
                .filter(|&c| c != '[' && c != ']' && !c.is_ascii_whitespace())
                .next()
            {
                stacks[column].push_back(c);
            }
            // moves are 1-indexed but our vector is 0-indexed so we increment at the end
            column += 1;
        }
    }

    stacks
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Move {
    num_to_move: usize,
    from_col: usize,
    to_col: usize,
}

fn parse_move(line: &str) -> Move {
    let tokens: Vec<usize> = line
        // this is the laziest parser but ugh i have shit to do
        .replace("move ", "")
        .replace(" from ", ",")
        .replace(" to ", ",")
        .split(",")
        .map(|tok| tok.parse::<usize>().expect("Should parse"))
        .collect();

    assert_eq!(tokens.len(), 3);

    Move {
        num_to_move: tokens[0],
        // input is 1-indexed but we want 0-indexed
        from_col: tokens[1] - 1,
        to_col: tokens[2] - 1,
    }
}

fn apply_move_a(stacks: &mut Vec<VecDeque<char>>, m: Move) {
    if m.from_col == m.to_col {
        return;
    }

    // e.g. if from_col is 0, to_col is 1, and we want to move 3 things
    // and col 0 is:    A B C D E F G
    // and col 1 is:    V W X Y Z
    // the goal should be:
    // col 0 is:        E F G
    // col 1 is:        C B A V W X Y Z

    let mut temp = VecDeque::with_capacity(m.num_to_move);
    for _ in 0..m.num_to_move {
        temp.push_back(stacks[m.from_col].pop_front().unwrap());
    }

    // temp now contains the top N elements from the "from_col"
    // in our example we're now at this state:
    // col 0 is:        E F G
    // temp  is:        A B C
    // col 1 is:        V W X Y Z

    for _ in 0..m.num_to_move {
        stacks[m.to_col].push_front(temp.pop_front().unwrap());
    }
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> String {
    let mut lines = input.lines();

    let mut stacks = parse_stacks(&mut lines);

    for next_move in lines.map(|line| parse_move(line)) {
        apply_move_b(&mut stacks, next_move);
    }

    make_output(&stacks)
}

fn apply_move_b(stacks: &mut Vec<VecDeque<char>>, m: Move) {
    if m.from_col == m.to_col {
        return;
    }

    // e.g. if from_col is 0, to_col is 1, and we want to move 3 things
    // and col 0 is:    A B C D E F G
    // and col 1 is:    V W X Y Z
    // the goal should be:
    // col 0 is:        E F G
    // col 1 is:        A B C V W X Y Z

    let mut temp = VecDeque::with_capacity(m.num_to_move);
    for _ in 0..m.num_to_move {
        temp.push_back(stacks[m.from_col].pop_front().unwrap());
    }

    // temp now contains the top N elements from the "from_col"
    // in our example we're now at this state:
    // col 0 is:        E F G
    // temp  is:        A B C
    // col 1 is:        V W X Y Z

    for _ in 0..m.num_to_move {
        stacks[m.to_col].push_front(temp.pop_back().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(a_with_input(input), "CMZ");
    }

    #[test]
    fn sample_b() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(b_with_input(input), "MCD");
    }
}

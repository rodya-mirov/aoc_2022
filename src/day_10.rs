use std::collections::VecDeque;

fn input() -> String {
    std::fs::read_to_string("input/input_10.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> i64 {
    let mut x_val: i64 = 1;
    let mut timer: usize = 0;

    let milestones: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut milestone_idx: usize = 0;
    let mut running_score: i64 = 0;

    for line in input.lines() {
        let old_val = x_val;
        if line == "noop" {
            timer += 1;
        } else if line.starts_with("addx") {
            timer += 2;
            let diff = line
                .chars()
                .skip(5)
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            x_val += diff;
        } else {
            unimplemented!("Unknown input: {}", line);
        }

        if milestone_idx >= milestones.len() {
            break;
        }

        let milestone = milestones[milestone_idx];
        if milestone <= timer {
            running_score += milestone as i64 * old_val;
            milestone_idx += 1;
        }
    }

    running_score
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> String {
    // a move is (END_TIME, DIFF)
    let mut moves: VecDeque<(i64, i64)> = VecDeque::new();

    let mut parsing_timer: i64 = 0;
    for line in input.lines() {
        if line == "noop" {
            parsing_timer += 1;
        } else if line.starts_with("addx") {
            parsing_timer += 2;
            let diff = line
                .chars()
                .skip(5)
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            moves.push_back((parsing_timer, diff));
        } else {
            unimplemented!("Unknown input: {}", line);
        }
    }

    let mut x_val: i64 = 1;

    let mut buffer = String::new();

    let mut next_move = moves.pop_front();

    for timer in 1..241 {
        // gets you in the 0-39 range
        // note the difference between timer (1-indexed) and x-pos (0-indexed), which i hate
        let timer_x = (timer - 1) % 40;
        if timer_x == 0 {
            buffer.push('\n');
        }
        if (timer_x - x_val).abs() <= 1 {
            buffer.push('#');
        } else {
            buffer.push('.');
        }

        if let Some((move_time, diff)) = next_move {
            if move_time == timer {
                x_val += diff;
                next_move = moves.pop_front();
            }
        }
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input(SAMPLE_INPUT_STR), 13140);
    }

    #[test]
    fn sample_b() {
        let expected = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let actual = b_with_input(SAMPLE_INPUT_STR);
        println!("Actual:\n{}", actual.trim());
        println!("Expected:\n{}", expected.trim());
        assert_eq!(actual.trim(), expected.trim());
    }
}

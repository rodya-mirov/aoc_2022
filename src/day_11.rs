use std::collections::VecDeque;

fn input() -> String {
    std::fs::read_to_string("input/input_11.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

mod parse {
    use std::collections::VecDeque;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, multispace0, multispace1},
        combinator::{eof, map},
        multi::{many0, separated_list1},
        sequence::tuple,
        IResult,
    };

    use super::{Arg, Monkey, Op, WorryAction};

    fn parse_monkey_line(input: &str) -> IResult<&str, usize> {
        let (out, val) = tuple((tag("Monkey "), digit1, tag(":"), multispace0))(input)?;
        let (_, digits, _, _) = val;
        let num: usize = digits.parse::<usize>().unwrap();
        Ok((out, num))
    }

    fn parse_starting_items_line(input: &str) -> IResult<&str, VecDeque<u64>> {
        let (out, val) = tuple((
            tag("Starting items:"),
            multispace0,
            separated_list1(tag(", "), digit1),
            multispace0,
        ))(input)?;

        let (_, _, items, _) = val;
        let items = items
            .iter()
            .map(|tok| tok.parse::<u64>().unwrap())
            .collect();
        Ok((out, items))
    }

    fn parse_op(input: &str) -> IResult<&str, Op> {
        let parse_add = map(tag("+"), |_| Op::Add);
        let parse_mul = map(tag("*"), |_| Op::Mul);

        alt((parse_add, parse_mul))(input)
    }

    fn parse_rhs(input: &str) -> IResult<&str, Arg> {
        let parse_old = map(tag("old"), |_| Arg::Old);
        let parse_num = map(digit1, |digits: &str| {
            Arg::Num(digits.parse::<u64>().unwrap())
        });

        alt((parse_num, parse_old))(input)
    }

    fn operation_line(input: &str) -> IResult<&str, WorryAction> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("Operation: new = old")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, op) = parse_op(input)?;
        let (input, _) = multispace1(input)?;
        let (input, rhs) = parse_rhs(input)?;
        let (input, _) = multispace0(input)?;

        Ok((input, WorryAction { op, rhs }))
    }

    fn test_line(input: &str) -> IResult<&str, u64> {
        let (out, val) =
            tuple((tag("Test: divisible by"), multispace1, digit1, multispace0))(input)?;
        let (_, _, digits, _) = val;
        Ok((out, digits.parse::<u64>().unwrap()))
    }

    fn if_true_line(input: &str) -> IResult<&str, usize> {
        let (out, val) = tuple((
            multispace0,
            tag("If true: throw to monkey"),
            multispace1,
            digit1,
            multispace0,
        ))(input)?;
        let (_, _, _, digits, _) = val;
        Ok((out, digits.parse::<usize>().unwrap()))
    }

    fn if_false_line(input: &str) -> IResult<&str, usize> {
        // note that the if_false line sometimes has an extra newline at the end, and sometimes not
        let (out, val) = tuple((
            multispace0,
            tag("If false: throw to monkey"),
            multispace1,
            digit1,
            multispace0,
        ))(input)?;
        let (_, _, _, digits, _) = val;
        Ok((out, digits.parse::<usize>().unwrap()))
    }

    fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
        let (out, val) = tuple((
            parse_monkey_line,
            parse_starting_items_line,
            operation_line,
            test_line,
            if_true_line,
            if_false_line,
        ))(input)?;

        let (idx, items, operation, div_test, if_true_goal, if_false_goal) = val;
        Ok((
            out,
            Monkey {
                idx,
                items,
                operation,
                div_test,
                if_true_goal,
                if_false_goal,
            },
        ))
    }

    fn parse_helper(input: &str) -> IResult<&str, Vec<Monkey>> {
        let (input, monkeys) = many0(parse_monkey)(input)?;
        let (_, _) = eof(input)?;
        Ok(("", monkeys))
    }

    pub(super) fn parse_input(input: &str) -> Vec<Monkey> {
        // just error "handling" around the nom stuff
        let (remaining, monkeys) = parse_helper(input).unwrap();
        assert_eq!(remaining, "");
        monkeys
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_test_0() {
            assert_eq!(parse_monkey_line("Monkey 0:").unwrap().1, 0);
            assert_eq!(
                parse_starting_items_line("Starting items: 79, 98")
                    .unwrap()
                    .1,
                VecDeque::from(vec![79, 98])
            );

            let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

            let (rem_input, actual) = parse_monkey(input).unwrap();
            assert_eq!(rem_input, "");

            let expected = Monkey {
                idx: 0,
                items: vec![79, 98].into(),
                operation: WorryAction {
                    op: Op::Mul,
                    rhs: Arg::Num(19),
                },
                div_test: 23,
                if_true_goal: 2,
                if_false_goal: 3,
            };

            assert_eq!(actual, expected);
        }
    }
}

fn a_with_input(input: &str) -> usize {
    let mut monkeys = parse::parse_input(input);
    a_parsed(&mut monkeys)
}

fn a_parsed(monkeys: &mut [Monkey]) -> usize {
    assert!(monkeys.len() >= 2);

    let mut inspection_counts: Vec<usize> = vec![0; monkeys.len()];
    const NUM_ROUNDS: usize = 20;

    for _ in 0..NUM_ROUNDS {
        simulate_round_a(monkeys, &mut inspection_counts);
    }

    // now we can compute the MONKEY BUSINESS
    inspection_counts.sort();

    inspection_counts.pop().unwrap() * inspection_counts.pop().unwrap()
}

fn simulate_round_a(monkeys: &mut [Monkey], inspection_counts: &mut [usize]) {
    for i in 0..monkeys.len() {
        inspection_counts[i] += monkeys[i].items.len();

        while let Some(mut next) = monkeys[i].items.pop_front() {
            next = apply_op_a(next, &monkeys[i].operation);
            let goal_monkey = if apply_div_test(&next, &monkeys[i].div_test) {
                monkeys[i].if_true_goal
            } else {
                monkeys[i].if_false_goal
            };
            monkeys[goal_monkey].items.push_back(next);
        }
    }
}

fn apply_div_test(num: &u64, test: &u64) -> bool {
    num % test == 0
}

fn apply_op_a(old_val: u64, op: &WorryAction) -> u64 {
    let base = match op.op {
        Op::Add => match &op.rhs {
            Arg::Num(num) => &old_val + num,
            Arg::Old => &old_val + &old_val,
        },
        Op::Mul => match &op.rhs {
            Arg::Num(num) => &old_val * num,
            Arg::Old => &old_val * &old_val,
        },
    };

    &base / 3_u64
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let mut monkeys = parse::parse_input(input);
    b_parsed(&mut monkeys)
}

fn b_parsed(monkeys: &mut [Monkey]) -> usize {
    assert!(monkeys.len() >= 2);

    let mut inspection_counts: Vec<usize> = vec![0; monkeys.len()];
    const NUM_ROUNDS: usize = 10000;

    let combined_modulus: u64 = monkeys.iter().map(|m| m.div_test).product();

    for _ in 0..NUM_ROUNDS {
        simulate_round_b(monkeys, &mut inspection_counts, combined_modulus);
    }

    // now we can compute the MONKEY BUSINESS
    inspection_counts.sort();

    inspection_counts.pop().unwrap() * inspection_counts.pop().unwrap()
}

fn simulate_round_b(
    monkeys: &mut [Monkey],
    inspection_counts: &mut [usize],
    combined_modulus: u64,
) {
    for i in 0..monkeys.len() {
        inspection_counts[i] += monkeys[i].items.len();

        while let Some(mut next) = monkeys[i].items.pop_front() {
            next = apply_op_b(next, monkeys[i].operation, combined_modulus);
            let goal_monkey = if next % monkeys[i].div_test == 0 {
                monkeys[i].if_true_goal
            } else {
                monkeys[i].if_false_goal
            };
            monkeys[goal_monkey].items.push_back(next);
        }
    }
}

fn apply_op_b(old_val: u64, op: WorryAction, modulus: u64) -> u64 {
    let base = match op.op {
        Op::Add => match op.rhs {
            Arg::Num(num) => old_val + num,
            Arg::Old => old_val + old_val,
        },
        Op::Mul => match op.rhs {
            Arg::Num(num) => old_val * num,
            Arg::Old => old_val * old_val,
        },
    };

    base % modulus
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    idx: usize,
    // represents the item's current worry level (not an index into a canonical map or anything)
    // in part A the //=3 keeps them manageable; in part B we use modular arithmetic; so we don't
    // need to worry about bigints
    items: VecDeque<u64>,
    operation: WorryAction,
    div_test: u64,
    if_true_goal: usize,
    if_false_goal: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct WorryAction {
    op: Op,
    // lhs is always Arg::Old
    rhs: Arg,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Arg {
    Old,
    Num(u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 10605);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 2713310158);
    }
}

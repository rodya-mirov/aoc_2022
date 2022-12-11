use std::collections::VecDeque;

fn input() -> String {
    std::fs::read_to_string("input/input_11.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    // let val = a_with_input(&contents);
    let val = a_parsed(&mut get_parsed_input());

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    unimplemented!()
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

    // let val = b_with_input(&contents);
    let val = b_parsed(&mut get_parsed_input());

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    unimplemented!()
}

fn b_parsed(monkeys: &mut [Monkey]) -> usize {
    assert!(monkeys.len() >= 2);

    let mut inspection_counts: Vec<usize> = vec![0; monkeys.len()];
    const NUM_ROUNDS: usize = 10000;

    let combined_modulus: u64 = monkeys.iter().map(|m| m.div_test).product();

    for i in 0..NUM_ROUNDS {
        simulate_round_b(monkeys, &mut inspection_counts, combined_modulus);

        let nice_i = i + 1;
        if nice_i == 1 || nice_i == 20 || nice_i % 1000 == 0 {
            println!("== After round {} ==", nice_i);
            for monkey_idx in 0..monkeys.len() {
                println!(
                    "Monkey {} inspected items {} times.",
                    monkey_idx, inspection_counts[monkey_idx]
                );
            }
            println!();
        }
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

// TODO: this is sort of cheating but ugh
fn get_parsed_input() -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    monkeys.push(Monkey {
        idx: 0,
        items: vec![80].into(),
        operation: WorryAction {
            op: Op::Mul,
            rhs: arg(5),
        },
        div_test: 2,
        if_true_goal: 4,
        if_false_goal: 3,
    });

    monkeys.push(Monkey {
        idx: 1,
        items: vec![75, 83, 74].into(),
        operation: WorryAction {
            op: Op::Add,
            rhs: arg(7),
        },
        div_test: 7,
        if_true_goal: 5,
        if_false_goal: 6,
    });

    monkeys.push(Monkey {
        idx: 2,
        items: vec![86, 67, 61, 96, 52, 63, 73].into(),
        operation: WorryAction {
            op: Op::Add,
            rhs: arg(5),
        },
        div_test: 3,
        if_true_goal: 7,
        if_false_goal: 0,
    });

    monkeys.push(Monkey {
        idx: 3,
        items: vec![85, 83, 55, 85, 57, 70, 85, 52].into(),
        operation: WorryAction {
            op: Op::Add,
            rhs: arg(8),
        },
        div_test: 17,
        if_true_goal: 1,
        if_false_goal: 5,
    });

    monkeys.push(Monkey {
        idx: 4,
        items: vec![67, 75, 91, 72, 89].into(),
        operation: WorryAction {
            op: Op::Add,
            rhs: arg(4),
        },
        div_test: 11,
        if_true_goal: 3,
        if_false_goal: 1,
    });

    monkeys.push(Monkey {
        idx: 5,
        items: vec![66, 64, 68, 92, 68, 77].into(),
        operation: WorryAction {
            op: Op::Mul,
            rhs: arg(2),
        },
        div_test: 19,
        if_true_goal: 6,
        if_false_goal: 2,
    });

    monkeys.push(Monkey {
        idx: 6,
        items: vec![97, 94, 79, 88].into(),
        operation: WorryAction {
            op: Op::Mul,
            rhs: Arg::Old,
        },
        div_test: 5,
        if_true_goal: 2,
        if_false_goal: 7,
    });

    monkeys.push(Monkey {
        idx: 7,
        items: vec![77, 85].into(),
        operation: WorryAction {
            op: Op::Add,
            rhs: arg(6),
        },
        div_test: 13,
        if_true_goal: 4,
        if_false_goal: 0,
    });

    monkeys
}

fn arg(num: u64) -> Arg {
    Arg::Num(num)
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
        let mut input = sample_monkeys();
        let actual = a_parsed(&mut input);
        assert_eq!(actual, 10605);
    }

    #[test]
    fn sample_b() {
        let mut input = sample_monkeys();
        let actual = b_parsed(&mut input);
        assert_eq!(actual, 2713310158);
    }

    fn sample_monkeys() -> Vec<Monkey> {
        let mut monkeys = Vec::new();

        monkeys.push(Monkey {
            idx: 0,
            items: vec![79, 98].into(),
            operation: WorryAction {
                op: Op::Mul,
                rhs: arg(19),
            },
            div_test: 23,
            if_true_goal: 2,
            if_false_goal: 3,
        });

        monkeys.push(Monkey {
            idx: 1,
            items: vec![54, 65, 75, 74].into(),
            operation: WorryAction {
                op: Op::Add,
                rhs: arg(6),
            },
            div_test: 19,
            if_true_goal: 2,
            if_false_goal: 0,
        });

        monkeys.push(Monkey {
            idx: 2,
            items: vec![79, 60, 97].into(),
            operation: WorryAction {
                op: Op::Mul,
                rhs: Arg::Old,
            },
            div_test: 13,
            if_true_goal: 1,
            if_false_goal: 3,
        });

        monkeys.push(Monkey {
            idx: 3,
            items: vec![74].into(),
            operation: WorryAction {
                op: Op::Add,
                rhs: arg(3),
            },
            div_test: 17,
            if_true_goal: 0,
            if_false_goal: 1,
        });

        monkeys
    }
}

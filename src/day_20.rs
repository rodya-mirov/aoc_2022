use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input/input_20.txt").expect("Should be able to read the file")
}

const KEY: i64 = 811589153;

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> i64 {
    let input = parse::parse_input(input);

    let input: Vec<(usize, i64)> = input.into_iter().enumerate().collect();

    assert!(input.len() < i64::MAX as usize);
    let mut mixed = input.clone();

    mix_list(&input, &mut mixed);

    let mut out = 0;
    for i in [1000, 2000, 3000] {
        out += problem_index(&mixed, i);
    }
    out
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> i64 {
    let input = parse::parse_input(input);

    let input: Vec<(usize, i64)> = input
        .into_iter()
        .enumerate()
        .map(|(i, val)| (i, val * KEY))
        .collect();

    assert!(input.len() < i64::MAX as usize);
    let mut mixed = input.clone();

    for _ in 0..10 {
        mix_list(&input, &mut mixed);
    }

    let mut out = 0;
    for i in [1000, 2000, 3000] {
        out += problem_index(&mixed, i);
    }
    out
}

fn mix_list(input: &[(usize, i64)], mixed: &mut Vec<(usize, i64)>) {
    for (original_index, to_mix) in input.iter().copied() {
        let start_index = mixed
            .iter()
            .find_position(|(ind, _)| *ind == original_index)
            .unwrap()
            .0;
        let popped = mixed.remove(start_index);
        debug_assert!(popped == (original_index, to_mix));

        // this shifting is strange because of the circularity
        // if you're moving _backward_ (to_mix is positive) then you can end up at the end of
        //      the list, but never the beginning
        // if you're moving _forward_ (to_mix is negative) then you can end up at the beginning of
        //      the list, but never the end
        let desired_index = {
            if to_mix == 0 {
                start_index
            } else if to_mix < 0 {
                rounded_index(start_index as i64 + to_mix, mixed.len())
            } else {
                let new_ind = rounded_index(start_index as i64 + to_mix, mixed.len());
                if new_ind == 0 {
                    mixed.len()
                } else {
                    new_ind
                }
            }
        };
        mixed.insert(desired_index, popped);
    }
}

fn problem_index(slice: &[(usize, i64)], ind: i64) -> i64 {
    let zero_ind = slice.iter().find_position(|(_, val)| *val == 0).unwrap().0 as i64;
    let ind = rounded_index(zero_ind + ind, slice.len());
    slice[ind].1
}

fn rounded_index(ind: i64, arr_len: usize) -> usize {
    (ind.rem_euclid(arr_len as i64)) as usize
}

mod parse {
    pub(super) fn parse_input(input: &str) -> Vec<i64> {
        input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "1
2
-3
3
-2
0
4";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 3);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 1623178306);
    }

    #[test]
    fn problem_index_tests() {
        let data = vec![1, 2, -3, 4, 0, 3, -2];
        let data: Vec<_> = data.into_iter().enumerate().collect();
        assert_eq!(problem_index(&data, 1000), 4);
        assert_eq!(problem_index(&data, 2000), -3);
        assert_eq!(problem_index(&data, 3000), 2);
    }
}

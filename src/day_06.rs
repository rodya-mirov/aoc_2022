use std::collections::VecDeque;

fn input() -> String {
    std::fs::read_to_string("input/input_06.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let mut chars = input.chars();

    let mut seen = VecDeque::new();

    for _ in 0..4 {
        seen.push_back(chars.next().expect("Must have length >= 4"));
    }

    fn is_done(seen: &VecDeque<char>) -> bool {
        assert_eq!(seen.len(), 4);
        let mut v: Vec<_> = seen.iter().copied().collect();
        v.sort();
        v[0] < v[1] && v[1] < v[2] && v[2] < v[3]
    }

    let mut c_ind = 4;
    while let Some(next) = chars.next() {
        c_ind += 1;
        seen.pop_front().unwrap();
        seen.push_back(next);
        if is_done(&seen) {
            return c_ind;
        }
    }

    unimplemented!("bad input")
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let mut chars = input.chars();

    let mut seen = VecDeque::new();

    for _ in 0..14 {
        seen.push_back(chars.next().expect("Must have length >= 4"));
    }

    fn is_done(seen: &VecDeque<char>) -> bool {
        assert_eq!(seen.len(), 14);
        let mut v: Vec<_> = seen.iter().copied().collect();
        v.sort();
        for i in 0..13 {
            if v[i] == v[i + 1] {
                return false;
            }
        }
        true
    }

    let mut c_ind = 14;
    while let Some(next) = chars.next() {
        c_ind += 1;
        seen.pop_front().unwrap();
        seen.push_back(next);
        if is_done(&seen) {
            return c_ind;
        }
    }

    unimplemented!("bad input")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(a_with_input("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(a_with_input("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(a_with_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(a_with_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_with_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(b_with_input("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(b_with_input("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(b_with_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(b_with_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

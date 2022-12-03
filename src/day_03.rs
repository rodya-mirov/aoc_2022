use std::collections::HashSet;

fn input() -> String {
    std::fs::read_to_string("input/input_03.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> i32 {
    input
        .lines()
        .map(|line| a_find_dupe(line))
        .map(|dupe| priority(dupe))
        .sum::<i32>()
}

fn a_find_dupe(line: &str) -> char {
    // NOTE: input is ASCII so .len() does what we want
    assert_eq!(line.len() % 2, 0);

    let mut seen = HashSet::new();

    let mut char_idx = 0;

    for c in line.trim().chars() {
        if char_idx * 2 < line.len() {
            seen.insert(c);
        } else if seen.contains(&c) {
            return c;
        }

        char_idx += 1;
    }

    unreachable!("Did not find a duplicate, possible bad input")
}

// gets the priority of a character.
// unspecified behavior when c is not in [A-Za-z]
fn priority(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        1 + (c as i32) - ('a' as i32)
    } else {
        27 + (c as i32) - ('A' as i32)
    }
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> i32 {
    let mut chunk = Vec::with_capacity(3);

    let mut total = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        chunk.push(line);
        if chunk.len() == 3 {
            let dupe = b_chunk_dupe(&chunk);
            let dupe_pri = priority(dupe);
            total += dupe_pri;
            chunk.clear();
        }
    }

    if chunk.len() != 0 {
        unimplemented!("Bad input -- should have #lines % 3 == 0")
    }

    total
}

fn b_chunk_dupe(lines: &Vec<&str>) -> char {
    assert_eq!(lines.len(), 3);

    let a: HashSet<char> = lines[0].chars().collect();
    let b: HashSet<char> = lines[1].chars().filter(|ch| a.contains(ch)).collect();
    let c: HashSet<char> = lines[2].chars().filter(|ch| b.contains(ch)).collect();

    assert_eq!(c.len(), 1);

    c.into_iter().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        assert_eq!(a_with_input("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(a_with_input("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
        assert_eq!(a_with_input("PmmdzqPrVvPwwTWBwg"), 42);
        assert_eq!(a_with_input("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
        assert_eq!(a_with_input("ttgJtRGJQctTZtZT"), 20);
        assert_eq!(a_with_input("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);

        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(a_with_input(input), 157);
    }

    #[test]
    fn sample_b() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";

        assert_eq!(b_with_input(input), 18);

        let input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(b_with_input(input), 52);

        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(b_with_input(input), 70);
    }
}

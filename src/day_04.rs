fn input() -> String {
    std::fs::read_to_string("input/input_04.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| a_line(line))
        .count()
}

fn a_line(line: &str) -> bool {
    let ((left0, left1), (right0, right1)) = parse_line(line);

    if left0 <= right0 && left1 >= right1 {
        true
    } else if right0 <= left0 && right1 >= left1 {
        true
    } else {
        false
    }
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let assignments: Vec<&str> = line.trim().split(",").collect();
    assert_eq!(assignments.len(), 2, "Bad input: {}", line);
    let left: Vec<i32> = assignments[0]
        .split("-")
        .map(|token| token.parse::<i32>().expect("Should parse"))
        .collect();
    assert_eq!(left.len(), 2);
    assert!(left[0] <= left[1]);
    let right: Vec<i32> = assignments[1]
        .split("-")
        .map(|token| token.parse::<i32>().expect("Should parse"))
        .collect();
    assert_eq!(right.len(), 2);
    assert!(right[0] <= right[1]);

    ((left[0], left[1]), (right[0], right[1]))
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| b_line(line))
        .count()
}

fn b_line(line: &str) -> bool {
    let ((left0, left1), (right0, right1)) = parse_line(line);

    // it is a fact that the intervals overlap iff one of the endpoints of one of the intervals
    // is contained in the other interval
    // i feel like these conditions are redundant but didn't quite crack it
    if left0 >= right0 && left0 <= right1 {
        true
    } else if left1 >= right0 && left1 <= right1 {
        true
    } else if right0 >= left0 && right0 <= left1 {
        true
    } else if right1 >= left0 && right1 <= left1 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        assert_eq!(a_line("2-4,6-8"), false);
        assert_eq!(a_line("2-3,4-5"), false);
        assert_eq!(a_line("5-7,7-9"), false);
        assert_eq!(a_line("2-8,3-7"), true);
        assert_eq!(a_line("6-6,4-6"), true);
        assert_eq!(a_line("2-6,4-8"), false);

        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(a_with_input(input), 2);
    }

    #[test]
    fn sample_b() {
        assert_eq!(b_line("2-4,6-8"), false);
        assert_eq!(b_line("2-3,4-5"), false);
        assert_eq!(b_line("5-7,7-9"), true);
        assert_eq!(b_line("2-8,3-7"), true);
        assert_eq!(b_line("6-6,4-6"), true);
        assert_eq!(b_line("2-6,4-8"), true);

        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(b_with_input(input), 4);
    }
}

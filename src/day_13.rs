use std::cmp::Ordering;

fn input() -> String {
    std::fs::read_to_string("input/input_13.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let vals = parse::parse_full_input(input);
    let len = vals.len() / 2;

    let mut out = 0;

    for i in 0..len {
        if vals[i * 2] < vals[i * 2 + 1] {
            out += i + 1;
        }
    }

    out
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let mut vals = parse::parse_full_input(input);
    let div_a = Val::List(vec![Val::List(vec![Val::Num(2)])]);
    let div_b = Val::List(vec![Val::List(vec![Val::Num(6)])]);

    vals.push(div_a.clone());
    vals.push(div_b.clone());

    vals.sort();

    let mut product = 1;

    for i in 0..vals.len() {
        let val = vals.get(i).unwrap();
        if val == &div_a || val == &div_b {
            product *= i + 1;
        }
    }

    product
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{eof, map},
        multi::separated_list0,
        sequence::tuple,
        IResult,
    };

    use super::Val;

    fn parse_list(input: &str) -> IResult<&str, Val> {
        let (input, (_, vals, _)) =
            tuple((tag("["), separated_list0(tag(","), parse_val), tag("]")))(input)?;

        Ok((input, Val::List(vals)))
    }

    fn parse_num(input: &str) -> IResult<&str, Val> {
        let (input, val) = map(digit1, |digits: &str| digits.parse::<u32>().unwrap())(input)?;

        Ok((input, Val::Num(val)))
    }

    fn parse_val(input: &str) -> IResult<&str, Val> {
        if input.starts_with("[") {
            parse_list(input)
        } else {
            parse_num(input)
        }
    }

    pub(super) fn parse_packet_line(line: &str) -> Val {
        let (_, (val, _)) = tuple((parse_val, eof))(line).expect("Should parse a val");

        val
    }

    pub(super) fn parse_full_input(all_input: &str) -> Vec<Val> {
        all_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| parse_packet_line(line))
            .collect::<Vec<Val>>()
    }

    #[cfg(test)]
    mod parse_tests {
        use super::*;

        #[test]
        fn parse_num_test() {
            assert_eq!(parse_num("12").unwrap().1, Val::Num(12));
        }

        #[test]
        fn parse_list_test() {
            assert_eq!(parse_list("[]").unwrap().1, Val::List(vec![]));
            assert_eq!(parse_list("[12]").unwrap().1, Val::List(vec![Val::Num(12)]));
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Val {
    Num(u32),
    List(Vec<Val>),
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Val::Num(n1) => match other {
                Val::Num(n2) => n1.cmp(n2),
                Val::List(list2) => {
                    let list1 = vec![Val::Num(*n1)];
                    list_cmp(&list1, list2)
                }
            },
            Val::List(list1) => match other {
                Val::Num(n2) => {
                    let list2 = vec![Val::Num(*n2)];
                    list_cmp(list1, &list2)
                }
                Val::List(list2) => list_cmp(list1, list2),
            },
        }
    }
}

fn list_cmp(list1: &[Val], list2: &[Val]) -> Ordering {
    let len = list1.len().min(list2.len());

    for i in 0..len {
        let v1 = list1.get(i).unwrap();
        let v2 = list2.get(i).unwrap();

        let ord = v1.cmp(v2);
        if ord != Ordering::Equal {
            return ord;
        }
    }

    list1.len().cmp(&list2.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 13);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 140);
    }
}

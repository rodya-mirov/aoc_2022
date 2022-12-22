use std::collections::HashMap;

fn input() -> String {
    std::fs::read_to_string("input/input_21.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> i64 {
    let (name_to_id, lines) = parse::parse_input(input);

    let mut id_to_idx_lookup = vec![0; lines.len()];

    for (row_idx, (id, _)) in lines.iter().enumerate() {
        id_to_idx_lookup[*id] = row_idx;
    }

    let root_id = name_to_id.get_or_panic("root");

    let tokens: Vec<Token> = lines.into_iter().map(|(_, token)| token).collect();

    let mut cache = HashMap::new();

    fn eval(
        id: usize,
        id_to_idx: &[usize],
        tokens: &[Token],
        cache: &mut HashMap<usize, i64>,
    ) -> i64 {
        if let Some(saved) = cache.get(&id).copied() {
            return saved;
        }

        let row_to_eval = id_to_idx[id];
        let token = tokens.get(row_to_eval).unwrap();

        let val = match token {
            Token::Num(val) => *val,
            Token::Humn => unreachable!("HUMN token should not appear in 21a"),
            Token::Action(Ref(id1), op, Ref(id2)) => {
                let val1 = eval(*id1, id_to_idx, tokens, cache);
                let val2 = eval(*id2, id_to_idx, tokens, cache);

                match op {
                    Op::Divide => val1 / val2,
                    Op::Times => val1 * val2,
                    Op::Plus => val1 + val2,
                    Op::Minus => val1 - val2,
                    Op::Equals => unreachable!("EQUALS op should not appear in 21a"),
                }
            }
        };

        cache.insert(id, val);
        val
    }

    eval(root_id, &id_to_idx_lookup, &tokens, &mut cache)
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> i64 {
    let (name_to_id, mut lines) = parse::parse_input(input);

    let mut id_to_idx_lookup = vec![0; lines.len()];

    for (row_idx, (id, _)) in lines.iter().enumerate() {
        id_to_idx_lookup[*id] = row_idx;
    }

    let humn_id = name_to_id.get_or_panic("humn");
    let humn_row = id_to_idx_lookup[humn_id];

    lines[humn_row] = (humn_id, Token::Humn);

    let root_id = name_to_id.get_or_panic("root");
    let root_row = id_to_idx_lookup[root_id];

    lines[root_row] = (
        root_id,
        match lines[root_row].1.clone() {
            Token::Num(_) => unreachable!(),
            Token::Humn => unreachable!(),
            Token::Action(r1, _, r2) => Token::Action(r1, Op::Equals, r2),
        },
    );

    let tokens: Vec<Token> = lines.into_iter().map(|(_, token)| token).collect();

    let mut cache = HashMap::new();

    // TODO: deal with root's weird OP thing

    fn fill_cache<'a, 'b: 'a>(
        id: usize,
        id_to_idx: &[usize],
        tokens: &[Token],
        cache: &'a mut HashMap<usize, Val>,
    ) {
        if cache.contains_key(&id) {
            return;
        }
        let row_to_eval = id_to_idx[id];
        let token = tokens.get(row_to_eval).unwrap();

        let val = match token {
            Token::Num(val) => Val::from_num(*val),
            Token::Humn => Val::from_var(),
            Token::Action(Ref(id1), op, Ref(id2)) => {
                fill_cache(*id1, id_to_idx, tokens, cache);
                fill_cache(*id2, id_to_idx, tokens, cache);

                let val1 = cache.get(&id1).unwrap();
                let val2 = cache.get(&id2).unwrap();

                match op {
                    Op::Divide => val1.divide(val2),
                    Op::Times => val1.times(val2),
                    Op::Plus => val1.plus(val2),
                    Op::Minus => val1.minus(val2),
                    Op::Equals => {
                        let diff = val1.minus(val2);
                        diff
                    }
                }
            }
        };

        cache.insert(id, val);
    }

    fill_cache(root_id, &id_to_idx_lookup, &tokens, &mut cache);

    let root_val = cache.get(&root_id).unwrap();
    let root_numer = root_val.numer();

    let solved_val = root_numer.equals_zero();

    solved_val
}

use val::Val;

mod val {
    use super::Poly;

    #[derive(Clone, Debug)]
    pub struct Val {
        numer: Poly,
        denom: Poly,
    }

    impl Val {
        pub fn from_num(num: i64) -> Val {
            Val {
                numer: Poly::from_vec(vec![num]),
                denom: Poly::from_vec(vec![1]),
            }
        }

        pub fn from_var() -> Val {
            Val {
                numer: Poly::from_vec(vec![0, 1]),
                denom: Poly::from_vec(vec![1]),
            }
        }

        pub fn numer(&self) -> &Poly {
            &self.numer
        }

        fn reduce(self) -> Self {
            let div = self.numer.scalar_gcd(&self.denom);
            if div > 1 {
                Self {
                    numer: self.numer.reduce_scalar(div).unwrap(),
                    denom: self.denom.reduce_scalar(div).unwrap(),
                }
            } else {
                self
            }
        }

        pub fn plus(&self, other: &Self) -> Val {
            Val {
                numer: (self.numer.times(&other.denom)).plus(&other.numer.times(&self.denom)),
                denom: (self.denom.times(&other.denom)),
            }
            .reduce()
        }

        pub fn minus(&self, other: &Self) -> Val {
            Val {
                numer: (self.numer.times(&other.denom)).minus(&other.numer.times(&self.denom)),
                denom: (self.denom.times(&other.denom)),
            }
            .reduce()
        }

        pub fn times(&self, other: &Self) -> Val {
            Val {
                numer: (self.numer.times(&other.numer)),
                denom: (self.denom.times(&other.denom)),
            }
            .reduce()
        }

        pub fn divide(&self, other: &Self) -> Val {
            Val {
                numer: (self.numer.times(&other.denom)),
                denom: (self.denom.times(&other.numer)),
            }
            .reduce()
        }
    }
}

use poly::Poly;

mod poly {
    use gcd::{binary_u64, Gcd};

    #[derive(Clone, Debug)]
    // 3x^5 + 2x^3 + 12 would be [12, 0, 0, 2, 0, 3]
    // -24 would be [-24]
    // coefficients, gcd of coefficients
    pub struct Poly(Vec<i64>, u64);

    impl Poly {
        pub fn from_vec(mut coefficients: Vec<i64>) -> Self {
            while coefficients.len() > 1 && coefficients[coefficients.len() - 1] == 0 {
                coefficients.pop();
            }
            let mut running_gcd = coefficients[0].abs() as u64;
            for i in 1..coefficients.len() {
                running_gcd = binary_u64(running_gcd, coefficients[i].abs() as u64);
            }
            Self(coefficients, running_gcd)
        }

        pub fn equals_zero(&self) -> i64 {
            if self.0.len() != 2 {
                unimplemented!("Can only solve linear equations");
            }

            let intercept = self.0[0];
            let slope = self.0[1];

            let val = -intercept / slope;

            assert_eq!(val * slope + intercept, 0);

            val
        }

        pub fn plus(&self, other: &Self) -> Self {
            let len = self.0.len().max(other.0.len());
            let mut out: Vec<i64> = vec![0; len];
            for (i, val) in self.0.iter().copied().enumerate() {
                out[i] = out[i].checked_add(val).unwrap();
            }
            for (i, val) in other.0.iter().copied().enumerate() {
                out[i] = out[i].checked_add(val).unwrap();
            }
            Self::from_vec(out)
        }

        pub fn minus(&self, other: &Self) -> Self {
            let len = self.0.len().max(other.0.len());
            let mut out: Vec<i64> = vec![0; len];
            for (i, val) in self.0.iter().copied().enumerate() {
                out[i] = out[i].checked_add(val).unwrap();
            }
            for (i, val) in other.0.iter().copied().enumerate() {
                out[i] = out[i].checked_sub(val).unwrap();
            }
            Self::from_vec(out)
        }

        pub fn reduce_scalar(&self, scalar: u64) -> Option<Self> {
            if self.1 % scalar != 0 {
                return None;
            }

            let mut out = self.0.clone();

            for i in 0..out.len() {
                out[i] /= scalar as i64;
                assert_eq!(out[i] * (scalar as i64), self.0[i]);
            }

            Some(Self::from_vec(out))
        }

        pub fn scalar_gcd(&self, other: &Self) -> u64 {
            self.1.gcd(other.1)
        }

        pub fn times(&self, other: &Self) -> Self {
            // (x^2 + 2) * (x^3 + 3x + 1)  == x^5 + 5x^3 + x^2 + 6x + 2
            // so [2, 0, 1] * [1, 3, 0, 1] == [2, 6, 1, 5, 0, 1]
            let len = self.0.len() + other.0.len() - 1;
            let mut out: Vec<i64> = vec![0; len];
            for (a_ind, a_val) in self.0.iter().copied().enumerate() {
                for (b_ind, b_val) in other.0.iter().copied().enumerate() {
                    let i = a_ind + b_ind;
                    let val = a_val.checked_mul(b_val).unwrap();
                    out[i] = out[i].checked_add(val).unwrap();
                }
            }
            Self::from_vec(out)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Op {
    Plus,
    Times,
    Minus,
    Divide,
    Equals,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Ref(usize);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Token {
    Num(i64),
    Humn,
    Action(Ref, Op, Ref),
}

#[derive(Default, Debug)]
struct Renamer {
    lookup: HashMap<String, usize>,
}

impl Renamer {
    fn get_or_insert(&mut self, name: String) -> usize {
        if let Some(id) = self.lookup.get(&name).copied() {
            return id;
        }
        let id = self.lookup.len();
        self.lookup.insert(name, id);
        id
    }

    fn get_or_panic(&self, name: &str) -> usize {
        self.lookup
            .get(name)
            .copied()
            .expect("Name should be registered")
    }
}

mod parse {
    use super::{Op, Ref, Renamer, Token};

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::{eof, map},
        IResult,
    };

    fn parse_num(input: &str) -> IResult<&str, i64> {
        if input.starts_with("-") {
            let (input, _) = tag("-")(input)?;
            let (input, num) = map(digit1, |digits: &str| digits.parse::<i64>().unwrap())(input)?;
            Ok((input, -num))
        } else {
            let (input, val) = map(digit1, |digits: &str| digits.parse::<i64>().unwrap())(input)?;

            Ok((input, val))
        }
    }

    fn parse_name<'a>(input: &'a str, ren: &mut Renamer) -> IResult<&'a str, usize> {
        let (input, name) = alpha1(input)?;
        let id = ren.get_or_insert(name.to_string());
        Ok((input, id))
    }

    fn parse_op(input: &str) -> IResult<&str, Op> {
        alt((
            map(tag(" * "), |_| Op::Times),
            map(tag(" + "), |_| Op::Plus),
            map(tag(" - "), |_| Op::Minus),
            map(tag(" / "), |_| Op::Divide),
        ))(input)
    }

    fn parse_op_expr<'a>(input: &'a str, ren: &mut Renamer) -> IResult<&'a str, Token> {
        let (input, id1) = parse_name(input, ren)?;
        let (input, op) = parse_op(input)?;
        let (input, id2) = parse_name(input, ren)?;

        let out = Token::Action(Ref(id1), op, Ref(id2));
        Ok((input, out))
    }

    fn parse_num_expr(input: &str) -> IResult<&str, Token> {
        map(parse_num, |num| Token::Num(num as i64))(input)
    }

    fn parse_token<'a>(input: &'a str, ren: &mut Renamer) -> IResult<&'a str, Token> {
        alt((parse_num_expr, |s| parse_op_expr(s, ren)))(input)
    }

    fn parse_line<'a>(input: &'a str, ren: &mut Renamer) -> IResult<&'a str, (usize, Token)> {
        let (input, id) = parse_name(input, ren)?;
        let (input, _) = tag(": ")(input)?;
        let (input, tok) = parse_token(input, ren)?;
        let (_, _) = eof(input)?;

        Ok(("", (id, tok)))
    }

    pub(super) fn parse_input(input: &str) -> (Renamer, Vec<(usize, Token)>) {
        let mut ren = Renamer::default();

        let lines = input
            .lines()
            .map(|line| parse_line(line, &mut ren).unwrap().1)
            .collect();

        (ren, lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 152);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 301);
    }
}

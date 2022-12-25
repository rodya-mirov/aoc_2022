fn input() -> String {
    std::fs::read_to_string("input/input_25.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> String {
    let mut total_dec = 0;
    for snafu in input.trim().lines() {
        let dec = snafu_to_dec(snafu);
        total_dec += dec;
    }
    dec_to_snafu(total_dec)
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    unimplemented!()
}

// strictly speaking this uses O(n^2) memory usage and time, where n is the number of digits in input,
// but i really don't think it matters. If it turns out to be a problem we can optimize it or something.
fn dec_to_snafu(input: i64) -> String {
    if input == 0 {
        return "0".to_string();
    } else if input < 0 {
        return negate_snafu(dec_to_snafu(-input).as_str());
    } else if input == 1 {
        return "1".to_string();
    } else if input == 2 {
        return "2".to_string();
    }

    let mut expected_num_digits = 2;

    // the magnitude of the left-most digit we will assign
    let mut running_power = 5;
    // the maximum amount of 'swing' (additional magnitude) the rest of the digits can provide
    // this can go either way; so if we pick '2' for our first digit, the expressible range is
    // 2 * running_power, plus or minus max_swing
    let mut max_swing = 2;

    while input > running_power * 2 + max_swing {
        max_swing += 2 * running_power;
        running_power *= 5;
        expected_num_digits += 1;
    }

    let out = if input < running_power - max_swing {
        unreachable!("Incremented power too far (input {})", input);
    } else if input <= running_power + max_swing {
        let mut out = String::new();
        out.push('1');
        let remainder = input - running_power;
        let rem_string = dec_to_snafu(remainder);
        // needed because the sub-outputs don't include leading zeroes, which we need in our
        // actual assembled output for context
        for _ in 0 .. expected_num_digits - rem_string.len() - 1 {
            out.push('0');
        }
        out += rem_string.as_str();
        out
    } else if input <= running_power * 2 + max_swing {
        let mut out = String::new();
        out.push('2');
        let remainder = input - 2 * running_power;
        let rem_string = dec_to_snafu(remainder);
        // needed because the sub-outputs don't include leading zeroes, which we need in our
        // actual assembled output for context
        for _ in 0 .. expected_num_digits - rem_string.len() - 1 {
            out.push('0');
        }
        out += rem_string.as_str();
        out
    } else {
        unimplemented!("Didn't increment power enough (input {})", input);
    };

    out
}

fn negate_snafu(snafu: &str) -> String {
    let mut out = String::new();

    for c in snafu.chars() {
        out.push(match c {
            '2' => '=',
            '1' => '-',
            '0' => '0',
            '-' => '1',
            '=' => '2',
            _ => unimplemented!("Bad input: bad SNAFU character {}", c)
        });
    }

    out
}

fn snafu_to_dec(input: &str) -> i64 {
    let mut out = 0;
    let mut out_pow = 1;

    for c in input.trim().chars().rev() {
        out += out_pow
            * match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => unimplemented!("Bad input: unknown SNAFU character {}", c),
            };
        out_pow *= 5;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    const PAIRS: [(i64, &'static str); 15] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    #[test]
    fn idempotence_tests() {
        for (dec, _) in &PAIRS {
            let snafu = dec_to_snafu(*dec);
            let dec_2 = snafu_to_dec(snafu.as_str());
            assert_eq!(*dec, dec_2);
        }
    }

    #[test]
    fn idempotence_tests_2() {
        for (_, snafu) in &PAIRS {
            let dec = snafu_to_dec(snafu);
            let snafu_2 = dec_to_snafu(dec);
            assert_eq!(*snafu, snafu_2.as_str());
        }
    }

    #[test]
    fn dec_to_snafu_tests() {
        for (dec, snafu) in &PAIRS {
            let actual = dec_to_snafu(*dec);
            assert_eq!(actual.as_str(), *snafu);
        }
    }

    #[test]
    fn snafu_to_dec_tests() {
        for (dec, snafu) in &PAIRS {
            let actual = snafu_to_dec(*snafu);
            assert_eq!(actual, *dec);
        }
    }

    #[test]
    fn sample_a() {
        let actual = a_with_input(SAMPLE_INPUT);
        assert_eq!(actual, "2=-1=0".to_string());
    }

    #[test]
    fn sample_b() {
        let actual = b_with_input(SAMPLE_INPUT);
        assert_eq!(actual, unimplemented!());
    }
}

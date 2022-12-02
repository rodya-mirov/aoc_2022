fn input() -> String {
    std::fs::read_to_string("input/input_02.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> i32 {
    enum Outcome {
        WIN, LOSS, TIE
    }

    impl Outcome {
        fn reverse(&self) -> Outcome {
            match self {
                Outcome::WIN => Outcome::LOSS,
                Outcome::LOSS => Outcome::WIN,
                Outcome::TIE => Outcome::TIE
            }
        }
    }

    // returns the result of a play from left to right
    // note this is the outcome for the LEFT player
    fn outcome(left: &str, right: &str) -> Outcome {
        match left {
            "A" => match right {
                "X" => Outcome::TIE,
                "Y" => Outcome::LOSS,
                "Z" => Outcome::WIN,
                _ => unimplemented!()
            },
            "B" => match right {
                "X" => Outcome::WIN,
                "Y" => Outcome::TIE,
                "Z" => Outcome::LOSS,
                _ => unimplemented!()
            },
            "C" => match right {
                "X" => Outcome::LOSS,
                "Y" => Outcome::WIN,
                "Z" => Outcome::TIE,
                _ => unimplemented!()
            },
            _ => unimplemented!()
        }
    }

    let mut score = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let left = tokens.next().unwrap();
        let right = tokens.next().unwrap();

        if tokens.next().is_some() {
            panic!()
        }

        score += match outcome(left, right).reverse() {
            Outcome::WIN => 6,
            Outcome::LOSS => 0,
            Outcome::TIE => 3,
        };

        score += match right {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!()
        };
    }

    score
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> i32 {
    enum Outcome {
        WIN, LOSS, TIE
    }

    enum Play {
        ROCK, SCISSORS, PAPER
    }

    fn outcome_from_token(token: &str) -> Outcome {
        match token {
            "X" => Outcome::LOSS,
            "Y" => Outcome::TIE,
            "Z" => Outcome::WIN,
            _ => unimplemented!()
        }
    }

    impl Outcome {
        fn score(&self) -> i32 {
            match self {
                Outcome::LOSS => 0,
                Outcome::TIE => 3,
                Outcome::WIN => 6
            }
        }
    }

    impl Play {
        fn score(&self) -> i32 {
            match self {
                Play::ROCK => 1,
                Play::PAPER => 2,
                Play::SCISSORS => 3,
            }
        }
    }

    fn play_from_token(token: &str) -> Play {
        match token {
            "A" => Play::ROCK,
            "B" => Play::PAPER,
            "C" => Play::SCISSORS,
            _ => panic!()
        }
    }

    fn desired_play(left: Play, desired_outcome: Outcome) -> Play {
        match left {
            Play::ROCK => match desired_outcome {
                Outcome::WIN => Play::PAPER,
                Outcome::LOSS => Play::SCISSORS,
                Outcome::TIE => Play::ROCK,
            },
            Play::SCISSORS => match desired_outcome {
                Outcome::WIN => Play::ROCK,
                Outcome::LOSS => Play::PAPER,
                Outcome::TIE => Play::SCISSORS,
            }
            Play::PAPER => match desired_outcome {
                Outcome::WIN => Play::SCISSORS,
                Outcome::LOSS => Play::ROCK,
                Outcome::TIE => Play::PAPER
            }
        }
    }

    let mut score = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let left = tokens.next().unwrap();
        let right = tokens.next().unwrap();

        if tokens.next().is_some() {
            panic!()
        }

        let left_play = play_from_token(left);
        let desired_outcome = outcome_from_token(right);

        score += desired_outcome.score();

        let desired_play = desired_play(left_play, desired_outcome);

        score += desired_play.score();
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_2a() {
        let input = "A Y
B X
C Z";
        let expected = 15;
        let actual = a_with_input(input);

        assert_eq!(expected, actual);

        assert_eq!(a_with_input("A Y"), 8);
        assert_eq!(a_with_input("B X"), 1);
        assert_eq!(a_with_input("C Z"), 6);
    }

    #[test]
    fn sample_2b() {
        let input = "A Y
B X
C Z";
        let expected = 12;
        let actual = b_with_input(input);

        assert_eq!(expected, actual);

        assert_eq!(b_with_input("A Y"), 4);
        assert_eq!(b_with_input("B X"), 1);
        assert_eq!(b_with_input("C Z"), 7);
    }

    #[test]
    fn sample_2b_more() {
        assert_eq!(b_with_input("A X"), 0+3); // lose -- 0 points; they play rock, so we play scissors, get 3 points
        assert_eq!(b_with_input("A Y"), 3+1); // tie -- 3 points; they play rock, so we play rock, get 1 point
        assert_eq!(b_with_input("A Z"), 6+2); // win -- 6 points; they play rock, so we play paper, get 2 points
        assert_eq!(b_with_input("B X"), 0+1); // lose -- 0 points; they play paper, so we play rock, get 1 point
        assert_eq!(b_with_input("B Y"), 3+2); // tie -- 3 points; they play paper, so we play paper, get 2 points
        assert_eq!(b_with_input("B Z"), 6+3); // win -- 6 points; they play paper, so we play scissors, get 3 points
        assert_eq!(b_with_input("C X"), 0+2); // lose -- 0 points; they play scissors, so we play paper, get 2 points
        assert_eq!(b_with_input("C Y"), 3+3); // tie -- 3 points; they play scissors, so we play scissors, get 3 points
        assert_eq!(b_with_input("C Z"), 6+1); // win -- 6 points; they play scissors, so we play rock, get 1 point
    }
}

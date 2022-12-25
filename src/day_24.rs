use std::collections::{HashMap, HashSet, VecDeque};

fn input() -> String {
    std::fs::read_to_string("input/input_24.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents);

    val.to_string()
}

fn a_with_input(input: &str) -> usize {
    let parse::ParseResult {
        xmin,
        xmax,
        ymin,
        ymax,
        blizzards,
    } = parse::parse_input(input);

    // realistically i don't really need variables for xmin / ymin
    // but it helps the code self document ... ? I guess?
    assert_eq!(xmin, 1);
    assert!(xmax > xmin);
    assert_eq!(ymin, 1);
    assert!(ymax > ymin);

    let blocked = compute_blockage(xmin, xmax, ymin, ymax, blizzards);

    let start = (Pos(xmin, ymin - 1), 0);
    let goal = Pos(xmax, ymax + 1);

    compute_path_cost(&blocked, start, goal)
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> usize {
    let parse::ParseResult {
        xmin,
        xmax,
        ymin,
        ymax,
        blizzards,
    } = parse::parse_input(input);

    // realistically i don't really need variables for xmin / ymin
    // but it helps the code self document ... ? I guess?
    assert_eq!(xmin, 1);
    assert!(xmax > xmin);
    assert_eq!(ymin, 1);
    assert!(ymax > ymin);

    let blocked = compute_blockage(xmin, xmax, ymin, ymax, blizzards);

    let start_pos = Pos(xmin, ymin - 1);
    let end_pos = Pos(xmax, ymax + 1);

    let end_a_time = compute_path_cost(&blocked, (start_pos, 0), end_pos);
    let start_b_time = compute_path_cost(&blocked, (end_pos, end_a_time), start_pos);
    let end_b_time = compute_path_cost(&blocked, (start_pos, start_b_time), end_pos);

    end_b_time
}

// blocked[timestep % period][(y - ymin) * height + (x - xmin)] indicates whether you're blocked
// at that time
struct Blockage {
    blockages: Vec<Vec<bool>>,
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    period: usize,
}

impl Blockage {
    fn is_free(&self, pos: Pos, timestep: usize) -> bool {
        let Pos(x, y) = pos;

        // start/end positions are always free
        if x == self.xmin && y + 1 == self.ymin {
            return true;
        } else if x == self.xmax && y == self.ymax + 1 {
            return true;
        }
        // otherwise, can't move to off-grid, but
        else if !(x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax) {
            return false;
        }
        // if we're on-grid, return true if we aren't blocked by a blizzard
        else {
            let timestep = timestep % self.period;

            let ind = (y - self.ymin) * self.width() + (x - self.xmin);
            !self.blockages[timestep][ind]
        }
    }

    fn width(&self) -> usize {
        self.xmax - self.xmin + 1
    }
}

fn compute_blockage(
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    blizzards: HashSet<(Pos, Dir)>,
) -> Blockage {
    // realistically i don't really need variables for xmin / ymin
    // but it helps the code self document ... ? I guess?
    assert_eq!(xmin, 1);
    assert!(xmax > xmin);
    assert_eq!(ymin, 1);
    assert!(ymax > ymin);

    let width = xmax - xmin + 1;
    let height = ymax - ymin + 1;
    let period = width * height / (gcd::binary_u64(width as u64, height as u64) as usize);

    // blocked[time % period][(y-1) * height + (x-1)] determines if the space is blocked (has
    // a blizzard) at that time and place
    let mut blocked = vec![vec![false; width * height]; period];

    // simulate the blizzards for PERIOD timesteps to determine passability
    let mut blizzards: Vec<(Pos, Dir)> = blizzards.into_iter().collect();

    let wrap = |val, min, max| {
        if val < min {
            max
        } else if val > max {
            min
        } else {
            val
        }
    };

    for timestep in 0..period {
        let blocked_now = &mut blocked[timestep];

        for (ref mut pos, dir) in blizzards.iter_mut() {
            blocked_now[(pos.1 - ymin) * width + (pos.0 - xmin)] = true;

            match dir {
                Dir::E => {
                    pos.0 = wrap(pos.0 + 1, xmin, xmax);
                }
                Dir::W => {
                    pos.0 = wrap(pos.0 - 1, xmin, xmax);
                }
                Dir::N => {
                    pos.1 = wrap(pos.1 - 1, ymin, ymax);
                }
                Dir::S => {
                    pos.1 = wrap(pos.1 + 1, ymin, ymax);
                }
            }
        }
    }

    Blockage {
        xmax,
        xmin,
        ymax,
        ymin,
        blockages: blocked,
        period,
    }
}

// from start (pos, start_time) to goal (pos), return earliest time of arrival
fn compute_path_cost(blocked: &Blockage, start: (Pos, usize), goal: Pos) -> usize {
    // map from (pos, timestep % period) to (actual timesteps, no modulus)
    // only care about the best
    let mut best_so_far: HashMap<(Pos, usize), usize> = HashMap::new();

    // (pos, timestep)
    // note this is an unweighted graph, so we naturally traverse paths in ascending order of cost
    // we don't need to do priority queues or any kind of exhaustive search to find the best path --
    // the first path is the best one
    let mut to_process: VecDeque<(Pos, usize)> = VecDeque::new();
    to_process.push_back(start);

    let push_if_free = |pos: Pos, timestep: usize, queue: &mut VecDeque<(Pos, usize)>| {
        if blocked.is_free(pos, timestep) {
            queue.push_back((pos, timestep));
        }
    };

    while let Some((pos, timestep)) = to_process.pop_front() {
        let saved = best_so_far
            .entry((pos, timestep % blocked.period))
            .or_insert(usize::MAX);
        let old = *saved;
        if timestep < old {
            *saved = timestep;
        } else {
            continue;
        }

        if pos == goal {
            return timestep;
        }

        // can wait, if there isn't a blizzard coming
        push_if_free(pos, timestep + 1, &mut to_process);

        // can also move N/E/S/W
        push_if_free(pos.n(), timestep + 1, &mut to_process);
        push_if_free(pos.e(), timestep + 1, &mut to_process);
        push_if_free(pos.s(), timestep + 1, &mut to_process);
        push_if_free(pos.w(), timestep + 1, &mut to_process);
    }

    unreachable!("Could not path to the goal position")
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos(usize, usize);

impl Pos {
    fn n(self) -> Self {
        Pos(self.0, self.1.saturating_sub(1))
    }

    fn s(self) -> Self {
        Pos(self.0, self.1.saturating_add(1))
    }

    fn w(self) -> Self {
        Pos(self.0.saturating_sub(1), self.1)
    }

    fn e(self) -> Self {
        Pos(self.0.saturating_add(1), self.1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

mod parse {
    use std::collections::HashSet;

    use super::{Dir, Pos};

    #[derive(Clone, Eq, PartialEq, Debug)]
    pub(super) struct ParseResult {
        // bounds for blizzard positions
        pub(super) xmin: usize,
        pub(super) xmax: usize,
        pub(super) ymin: usize,
        pub(super) ymax: usize,
        // blizzard positions at time zero
        pub(super) blizzards: HashSet<(Pos, Dir)>,
    }

    pub(super) fn parse_input(input: &str) -> ParseResult {
        let mut blizzards = HashSet::new();
        let mut xmax = 0;
        let mut ymax = 0;
        for (y, line) in input.lines().enumerate() {
            ymax = ymax.max(y);
            for (x, c) in line.chars().enumerate() {
                xmax = xmax.max(x);
                match c {
                    '#' => {}
                    '.' => {}
                    '>' => {
                        blizzards.insert((Pos(x, y), Dir::E));
                    }
                    'v' => {
                        blizzards.insert((Pos(x, y), Dir::S));
                    }
                    '<' => {
                        blizzards.insert((Pos(x, y), Dir::W));
                    }
                    '^' => {
                        blizzards.insert((Pos(x, y), Dir::N));
                    }
                    _ => unimplemented!("Unknown character {}", c),
                }
            }
        }
        ParseResult {
            xmin: 1,
            ymin: 1,
            xmax: xmax - 1,
            ymax: ymax - 1,
            blizzards,
        }
    }

    #[cfg(test)]
    mod parse_test {
        use super::*;

        const SAMPLE: &'static str = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";

        #[test]
        fn parse_test() {
            let actual = parse_input(SAMPLE);
            let mut expected_blizzards = HashSet::new();

            expected_blizzards.insert((Pos(1, 2), Dir::E));
            expected_blizzards.insert((Pos(4, 4), Dir::S));

            let expected = ParseResult {
                xmin: 1,
                ymin: 1,
                xmax: 5,
                ymax: 5,
                blizzards: expected_blizzards,
            };

            assert_eq!(expected, actual);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BIGGER_SAMPLE_INPUT_STR: &'static str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn sample_a() {
        let input = BIGGER_SAMPLE_INPUT_STR;
        let actual = a_with_input(input);
        assert_eq!(actual, 18);
    }

    #[test]
    fn sample_b() {
        let input = BIGGER_SAMPLE_INPUT_STR;
        let actual = b_with_input(input);
        assert_eq!(actual, 54);
    }
}

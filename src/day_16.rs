use std::collections::HashMap;

use opened::ValvesOpen;
use valves::Valves;

fn input() -> String {
    std::fs::read_to_string("input/input_16.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents, 30);

    val.to_string()
}

fn a_with_input(input: &str, total_time: u32) -> u32 {
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    struct ImmediateState {
        current_position: usize,
        valves_open: ValvesOpen,
        time_remaining: u32,
    }

    // A map from immediate state to the best _additional_ flow that can be managed from that point
    // on. The idea is that no state needs to be traversed twice.
    type Cache = HashMap<ImmediateState, u32>;

    let mut valves = Valves::new();
    let input = parse::parse_input(input, &mut valves);

    let num_nodes = valves.num_nodes();

    let mut valves_open = ValvesOpen::new(num_nodes);
    let mut flow_rate_by_node = vec![0; num_nodes];
    let mut tunnels_by_node = vec![Vec::new(); num_nodes];

    for (node_index, flow_rate, tunnels) in input {
        flow_rate_by_node[node_index] = flow_rate;
        tunnels_by_node[node_index] = tunnels;
        if flow_rate == 0 {
            valves_open.open(node_index);
        }
    }
    let mut best_at_state: Cache = HashMap::new();

    // simple recursion with no pruning, just a DFS
    // stop conditions: time runs out, or all valves are open
    // Returns: the ADDITIONAL_FLOW that can be managed from this point.
    fn recurse(
        valves_open: ValvesOpen,
        flow_rate_by_node: &[u32],
        tunnels_by_node: &[Vec<usize>],
        current_position: usize,
        time_remaining: u32,
        cache: &mut Cache,
    ) -> u32 {
        // if we're out of time, or every valve is open, there's no more room for benefit, just
        // stop the simulation and be done
        if time_remaining == 0 || valves_open.all_open() {
            return 0;
        }

        let state = ImmediateState {
            current_position,
            valves_open,
            time_remaining,
        };

        if let Some(saved) = cache.get(&state).copied() {
            return saved;
        }

        let mut best = 0;

        if !valves_open.is_open(current_position) {
            let mut valves_open = valves_open.clone();
            valves_open.open(current_position);
            let direct_flow = flow_rate_by_node[current_position] * (time_remaining - 1);
            let flow_after_here = recurse(
                valves_open,
                flow_rate_by_node,
                tunnels_by_node,
                current_position,
                time_remaining - 1,
                cache,
            );
            let flow_here = direct_flow + flow_after_here;
            best = best.max(flow_here);
        }

        for connecting_position in tunnels_by_node[current_position].iter().copied() {
            let flow_after_move = recurse(
                valves_open,
                flow_rate_by_node,
                tunnels_by_node,
                connecting_position,
                time_remaining - 1,
                cache,
            );
            best = best.max(flow_after_move);
        }

        cache.insert(state, best);

        best
    }

    let start_position = valves.start_position();
    let best = recurse(
        valves_open,
        flow_rate_by_node.as_slice(),
        tunnels_by_node.as_slice(),
        start_position,
        total_time,
        &mut best_at_state,
    );

    best
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents, 15);

    val.to_string()
}

fn b_with_input(input: &str, total_time: u32) -> u32 {
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    struct ImmediateState {
        a_pos: usize,
        b_pos: usize,
        valves_open: ValvesOpen,
        a_time_remaining: u32,
        b_time_remaining: u32,
    }

    // A map from immediate state to the best _additional_ flow that can be managed from that point
    // on. The idea is that no state needs to be traversed twice.
    type Cache = HashMap<ImmediateState, u32>;

    let mut valves = Valves::new();
    let input = parse::parse_input(input, &mut valves);

    let num_nodes = valves.num_nodes();

    let mut valves_open = ValvesOpen::new(num_nodes);
    let mut flow_rate_by_node = vec![0; num_nodes];
    let mut tunnels_by_node = vec![Vec::new(); num_nodes];

    for (node_index, flow_rate, tunnels) in input {
        flow_rate_by_node[node_index] = flow_rate;
        tunnels_by_node[node_index] = tunnels;
        if flow_rate == 0 {
            valves_open.open(node_index);
        }
    }
    let mut best_at_state: Cache = HashMap::new();

    // simple recursion with no pruning, just a DFS
    // stop conditions: time runs out, or all valves are open
    // Returns: the ADDITIONAL_FLOW that can be managed from this point.
    fn recurse(
        valves_open: ValvesOpen,
        flow_rate_by_node: &[u32],
        tunnels_by_node: &[Vec<usize>],
        // NOTE: a_pos / a_time_remaining is the "next player"
        //       after player a goes, it's in the next call as player b
        a_pos: usize,
        a_time_remaining: u32,
        b_pos: usize,
        b_time_remaining: u32,
        cache: &mut Cache,
        // abort the flow if it is determined that we cannot exceed this goal
        min_flow_goal: u32,
        // not important for correctness, just indicates the number of cache misses
        iterations: &mut usize,
    ) -> Option<u32> {
        debug_assert!(a_time_remaining >= b_time_remaining);
        debug_assert!(b_time_remaining + 1 >= a_time_remaining);

        // if we're out of time, or every valve is open, there's no more room for benefit, just
        // stop the simulation and be done
        if a_time_remaining == 0 || valves_open.all_open() {
            return Some(0);
        }

        if valves_open.max_remaining_flow(flow_rate_by_node, a_time_remaining, a_pos, b_pos)
            <= min_flow_goal
        {
            return None;
        }

        let state = ImmediateState {
            valves_open,
            a_pos,
            b_pos,
            a_time_remaining,
            b_time_remaining,
        };

        if let Some(saved) = cache.get(&state).copied() {
            return Some(saved);
        }

        *iterations += 1;

        let mut best = 0;

        if !valves_open.is_open(a_pos) {
            let mut valves_open = valves_open.clone();
            valves_open.open(a_pos);
            let direct_flow = flow_rate_by_node[a_pos] * (a_time_remaining - 1);
            let flow_after_here = recurse(
                valves_open,
                flow_rate_by_node,
                tunnels_by_node,
                // B becomes A
                b_pos,
                b_time_remaining,
                // A moves to new position, reduces time, and becomes B
                a_pos,
                a_time_remaining - 1,
                cache,
                best,
                iterations,
            );
            if let Some(flow_after_here) = flow_after_here {
                let flow_here = direct_flow + flow_after_here;
                best = best.max(flow_here);
            }
        }

        let mut ideal_positions: Vec<usize> = tunnels_by_node[a_pos].iter().copied().collect();
        ideal_positions.sort_by_key(|&node| {
            if valves_open.is_open(node) {
                0
            } else {
                flow_rate_by_node[node]
            }
        });
        for connecting_position in ideal_positions.into_iter().rev() {
            let flow_after_move = recurse(
                valves_open,
                flow_rate_by_node,
                tunnels_by_node,
                // B becomes A
                b_pos,
                b_time_remaining,
                // A moves to new position, reduces time, and becomes B
                connecting_position,
                a_time_remaining - 1,
                cache,
                best,
                iterations,
            );
            if let Some(flow_after_move) = flow_after_move {
                best = best.max(flow_after_move);
            }
        }

        cache.insert(state, best);

        Some(best)
    }

    let mut iterations = 0;
    let start_position = valves.start_position();
    let best = recurse(
        valves_open,
        flow_rate_by_node.as_slice(),
        tunnels_by_node.as_slice(),
        start_position,
        total_time,
        start_position,
        total_time,
        &mut best_at_state,
        0,
        &mut iterations,
    );

    println!("  Hit {} iterations", iterations);
    println!("  Had {} states in cache at the end", best_at_state.len());

    best.unwrap()
}

mod valves {
    use std::collections::HashMap;

    pub(super) struct Valves {
        lookup: HashMap<String, usize>,
    }

    impl Valves {
        pub(super) fn new() -> Self {
            Valves {
                lookup: HashMap::new(),
            }
        }

        pub(super) fn get_or_insert_index(&mut self, s: &str) -> usize {
            if self.lookup.contains_key(s) {
                return self.lookup.get(s).copied().unwrap();
            } else {
                let ind = self.lookup.len();
                self.lookup.insert(s.to_string(), ind);
                ind
            }
        }

        pub(super) fn num_nodes(&self) -> usize {
            self.lookup.len()
        }

        pub(super) fn start_position(&self) -> usize {
            self.lookup
                .get("AA")
                .copied()
                .expect("AA should be a valid node")
        }
    }
}

mod opened {
    #[derive(Hash, Eq, PartialEq, Copy, Clone)]
    // bit at position i is 1 if the valve is CLOSED and 0 if the valve is OPEN
    // so we're all open if the inner value is zero
    pub(super) struct ValvesOpen(u64);

    impl ValvesOpen {
        // Construct a new valve-open set, with everything defaulted to closed
        pub fn new(num_nodes: usize) -> Self {
            assert!(num_nodes < 64);

            let mut out = Self(0);

            for i in 0..num_nodes {
                out.close(i);
            }

            out
        }

        pub fn is_open(&self, ind: usize) -> bool {
            let mask = 1 << ind;
            (self.0 & mask) == 0
        }

        pub fn open(&mut self, ind: usize) {
            self.0 = self.0 & (!(1 << ind));
        }

        pub fn close(&mut self, ind: usize) {
            self.0 = self.0 | (1 << ind);
        }

        pub fn all_open(&self) -> bool {
            self.0 == 0
        }

        /// Provides an upper bound on the amount of possible additional flow which can be
        /// obtained from this point. Not intended to be a sharp upper bound, just sufficient
        /// for culling.
        /// TODO: basically I just don't think this heuristic is good enough
        pub fn max_remaining_flow(
            &self,
            flows: &[u32],
            time_remaining: u32,
            a_pos: usize,
            b_pos: usize,
        ) -> u32 {
            let mut out = 0;
            for i in 0..flows.len() {
                if !self.is_open(i) {
                    if time_remaining >= 1 && (i == a_pos || i == b_pos) {
                        out += flows[i] * (time_remaining - 1);
                    } else if time_remaining >= 2 {
                        out += flows[i] * (time_remaining - 2);
                    }
                }
            }
            out
        }
    }

    #[cfg(test)]
    mod bitmask_tests {
        use super::ValvesOpen;

        #[test]
        fn basic_test() {
            let mut mask = ValvesOpen::new(12);
            assert_eq!(mask.0, (1 << 12) - 1);

            assert!(!mask.all_open());

            for i in 0..12 {
                assert!(!mask.is_open(i));
                mask.open(i);
                assert!(mask.is_open(i));
                mask.close(i);
                assert!(!mask.is_open(i));
                mask.open(i);
                assert!(mask.is_open(i));
                assert_eq!(mask.all_open(), i == 11);
            }
        }
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::{eof, map},
        multi::separated_list1,
        IResult,
    };

    use super::Valves;

    fn parse_num(input: &str) -> IResult<&str, u32> {
        let (input, val) = map(digit1, |digits: &str| digits.parse::<u32>().unwrap())(input)?;

        Ok((input, val))
    }

    fn parse_valve_name<'a>(input: &'a str, valves: &mut Valves) -> IResult<&'a str, usize> {
        let (input, name) = alpha1(input)?;
        let ind = valves.get_or_insert_index(name);
        Ok((input, ind))
    }

    fn parse_line_helper<'a>(
        input: &'a str,
        valves: &mut Valves,
    ) -> IResult<&'a str, (usize, u32, Vec<usize>)> {
        let (input, _) = tag("Valve ")(input)?;
        let (input, valve_ind) = parse_valve_name(input, valves)?;
        let (input, _) = tag(" has flow rate=")(input)?;
        let (input, flow_rate) = parse_num(input)?;
        let (input, _) = alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        ))(input)?;
        let (input, destinations) =
            separated_list1(tag(", "), |s| parse_valve_name(s, valves))(input)?;
        let (_, _) = eof(input)?;

        Ok(("", (valve_ind, flow_rate, destinations)))
    }

    fn parse_line(input: &str, valves: &mut Valves) -> (usize, u32, Vec<usize>) {
        parse_line_helper(input, valves).unwrap().1
    }

    pub(super) fn parse_input(input: &str, valves: &mut Valves) -> Vec<(usize, u32, Vec<usize>)> {
        input.lines().map(|line| parse_line(line, valves)).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::parse_line;
        use super::Valves;

        #[test]
        fn sample_lines() {
            let mut valves = Valves::new();

            assert_eq!(
                parse_line(
                    "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
                    &mut valves
                ),
                (0, 0, vec![1, 2, 3])
            );
            // note BB has already been given index 3, and AA has been given index 0
            // but CC is new, so it gets 4
            assert_eq!(
                parse_line(
                    "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
                    &mut valves
                ),
                (3, 13, vec![4, 0])
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str =
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input, 30);
        assert_eq!(actual, 1651);
    }

    #[test]
    fn sample_b() {
        let input = SAMPLE_INPUT_STR;
        let actual = b_with_input(input, 26);
        assert_eq!(actual, 1707);
    }
}

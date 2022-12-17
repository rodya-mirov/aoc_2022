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
    let tunnel_state = parse_and_trcl_input(input);

    let num_nodes = tunnel_state.flows.len();

    let mut valves_open = ValvesOpen::new(num_nodes);

    for i in 0..num_nodes {
        if tunnel_state.flows[i] == 0 {
            valves_open.open(i);
        }
    }

    fn recurse(
        valves_open: ValvesOpen,
        tunnel_state: &TunnelState,
        // CONTRACT: if a player is in a location, that location is ALREADY open and the flow
        //      is accounted for in the flow_so_far variable
        pos: usize,
        time_remaining: u32,
        flow_so_far: u32,
        // maintained as we go
        best_total_ever: &mut u32,
    ) {
        // if we got here, the configuration is possible; we could just stop here and it would
        // be one possible solution
        *best_total_ever = (*best_total_ever).max(flow_so_far);

        // if we're out of time, or every valve is open, there's no more room for benefit, just
        // stop the simulation and be done
        if time_remaining == 0 || valves_open.all_open() {
            return;
        }

        // this isn't a perfect metric but it's probably good enough -- if it's simply impossible
        // to exceed the best total ever from here, then stop
        if valves_open.max_remaining_flow_one_player(tunnel_state, pos, time_remaining)
            + flow_so_far
            <= *best_total_ever
        {
            return;
        }

        // Player has a couple of options -- they can either take an optimal path to an unopened
        // valve and immediately open it; or they can stop forever (there is no advantage
        // in doing anything else)
        for target_node in 0..tunnel_state.flows.len() {
            if valves_open.is_open(target_node) {
                continue;
            }

            let travel_and_open_time = tunnel_state.path_weights[pos][target_node] + 1;
            if travel_and_open_time >= time_remaining {
                continue;
            }

            let open_time = time_remaining - travel_and_open_time;

            let local_flow = flow_so_far + open_time * tunnel_state.flows[target_node];
            let mut valves_open = valves_open.clone();
            valves_open.open(target_node);

            recurse(
                valves_open,
                tunnel_state,
                target_node,
                open_time,
                local_flow,
                best_total_ever,
            );
        }

        // I don't think there's any advantage to just stopping, but why not cover it
        // since I don't feel 100% certain
        recurse(
            valves_open,
            tunnel_state,
            pos,
            0,
            flow_so_far,
            best_total_ever,
        );
    }

    let mut best_total_ever = 0;
    recurse(
        valves_open,
        &tunnel_state,
        tunnel_state.start_node,
        total_time,
        0,
        &mut best_total_ever,
    );

    best_total_ever
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents, 26);

    val.to_string()
}

fn b_with_input(input: &str, total_time: u32) -> u32 {
    let tunnel_state = parse_and_trcl_input(input);

    let num_nodes = tunnel_state.flows.len();

    let mut valves_open = ValvesOpen::new(num_nodes);

    for i in 0..num_nodes {
        if tunnel_state.flows[i] == 0 {
            valves_open.open(i);
        }
    }

    fn recurse(
        valves_open: ValvesOpen,
        tunnel_state: &TunnelState,
        // CONTRACT: if a player is in a location, that location is ALREADY open and the flow
        //      is accounted for in the flow_so_far variable
        // NOTE: a_pos / a_time_remaining is the "next player"
        //       after player a goes, it's in the next call as player b
        a_pos: usize,
        a_time_remaining: u32,
        b_pos: usize,
        b_time_remaining: u32,
        flow_so_far: u32,
        // maintained as we go
        best_total_ever: &mut u32,
    ) {
        // if we got here, the configuration is possible; we could just stop here and it would
        // be one possible solution
        *best_total_ever = (*best_total_ever).max(flow_so_far);

        // no real downside to making sure we're doing things in time-order
        // and lots of upsides (always better to hit a valve sooner than later)
        // we also just assume that, if the times are equal, a_pos < b_pos; this just drops some
        // symmetric cases that we don't need to cover twice
        if a_time_remaining < b_time_remaining
            || (a_time_remaining == b_time_remaining && a_pos > b_pos)
        {
            recurse(
                valves_open,
                tunnel_state,
                b_pos,
                b_time_remaining,
                a_pos,
                a_time_remaining,
                flow_so_far,
                best_total_ever,
            );
            return;
        }

        // if we're out of time, or every valve is open, there's no more room for benefit, just
        // stop the simulation and be done
        if a_time_remaining == 0 || valves_open.all_open() {
            return;
        }

        // this isn't a perfect metric but it's probably good enough -- if it's simply impossible
        // to exceed the best total ever from here, then stop
        if valves_open.max_remaining_flow_two_players(
            tunnel_state,
            a_pos,
            a_time_remaining,
            b_pos,
            b_time_remaining,
        ) + flow_so_far
            <= *best_total_ever
        {
            return;
        }

        // Player A has a couple of options -- they can either take an optimal path to an unopened
        // valve and immediately open it; or they can stop forever (there is no advantage
        // in doing anything else)
        for target_node in 0..tunnel_state.flows.len() {
            if valves_open.is_open(target_node) {
                continue;
            }

            let travel_and_open_time = tunnel_state.path_weights[a_pos][target_node] + 1;
            if travel_and_open_time >= a_time_remaining {
                continue;
            }

            let open_time = a_time_remaining - travel_and_open_time;

            let local_flow = flow_so_far + open_time * tunnel_state.flows[target_node];
            let mut valves_open = valves_open.clone();
            valves_open.open(target_node);

            recurse(
                valves_open,
                tunnel_state,
                b_pos,
                b_time_remaining,
                target_node,
                open_time,
                local_flow,
                best_total_ever,
            );
        }

        // there might be an advantage in A just stopping here (in case B can get to the valve faster or whatever)
        // so just cover that case too
        recurse(
            valves_open,
            tunnel_state,
            b_pos,
            b_time_remaining,
            a_pos,
            0,
            flow_so_far,
            best_total_ever,
        );
    }

    let mut best_total_ever = 0;
    recurse(
        valves_open,
        &tunnel_state,
        tunnel_state.start_node,
        total_time,
        tunnel_state.start_node,
        total_time,
        0,
        &mut best_total_ever,
    );

    best_total_ever
}

mod valves {
    use std::collections::HashMap;

    pub(super) struct Valves<T = String> {
        lookup: HashMap<T, usize>,
    }

    impl<T: Eq + PartialEq + std::hash::Hash> Valves<T> {
        pub(super) fn new() -> Self {
            Valves {
                lookup: HashMap::new(),
            }
        }

        pub(super) fn get_or_insert_index(&mut self, s: T) -> usize {
            if self.lookup.contains_key(&s) {
                return self.lookup.get(&s).copied().unwrap();
            } else {
                let ind = self.lookup.len();
                self.lookup.insert(s, ind);
                ind
            }
        }
    }
}

mod opened {
    use crate::day_16::TunnelState;

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
        pub fn max_remaining_flow_one_player(
            &self,
            tunnel_state: &TunnelState,
            pos: usize,
            time_remaining: u32,
        ) -> u32 {
            let mut out = 0;
            for i in 0..tunnel_state.flows.len() {
                if !self.is_open(i) {
                    let mut best_time = 0;
                    let local_flow = tunnel_state.flows[i];

                    let travel_time = tunnel_state.path_weights[pos][i];
                    if travel_time + 1 < time_remaining {
                        best_time = time_remaining - travel_time - 1;
                    }
                    out += best_time * local_flow;
                }
            }
            out
        }

        /// Provides an upper bound on the amount of possible additional flow which can be
        /// obtained from this point. Not intended to be a sharp upper bound, just sufficient
        /// for culling.
        pub fn max_remaining_flow_two_players(
            &self,
            tunnel_state: &TunnelState,
            a_pos: usize,
            a_time_remaining: u32,
            b_pos: usize,
            b_time_remaining: u32,
        ) -> u32 {
            let mut out = 0;
            for i in 0..tunnel_state.flows.len() {
                if !self.is_open(i) {
                    let mut best_time = 0;
                    let local_flow = tunnel_state.flows[i];

                    let travel_time_a = tunnel_state.path_weights[a_pos][i];
                    if travel_time_a + 1 < a_time_remaining {
                        best_time = a_time_remaining - travel_time_a - 1;
                    }
                    let travel_time_b = tunnel_state.path_weights[b_pos][i];
                    if travel_time_b + 1 < b_time_remaining {
                        best_time = best_time.max(b_time_remaining - travel_time_b - 1);
                    }
                    out += best_time * local_flow;
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

    fn parse_num(input: &str) -> IResult<&str, u32> {
        let (input, val) = map(digit1, |digits: &str| digits.parse::<u32>().unwrap())(input)?;

        Ok((input, val))
    }

    fn parse_valve_name(input: &str) -> IResult<&str, String> {
        let (input, name) = alpha1(input)?;
        Ok((input, name.to_string()))
    }

    fn parse_line_helper(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
        let (input, _) = tag("Valve ")(input)?;
        let (input, valve_name) = parse_valve_name(input)?;
        let (input, _) = tag(" has flow rate=")(input)?;
        let (input, flow_rate) = parse_num(input)?;
        let (input, _) = alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        ))(input)?;
        let (input, destinations) = separated_list1(tag(", "), |s| parse_valve_name(s))(input)?;
        let (_, _) = eof(input)?;

        Ok(("", (valve_name, flow_rate, destinations)))
    }

    fn parse_line(input: &str) -> (String, u32, Vec<String>) {
        parse_line_helper(input).unwrap().1
    }

    pub(super) fn parse_input(input: &str) -> Vec<(String, u32, Vec<String>)> {
        input.lines().map(parse_line).collect()
    }
}

struct TunnelState {
    flows: Vec<u32>,
    // 2D array -- path_weights[i, j] is cost of shortest path from i to j
    //      unreachable path is u32::MAX
    path_weights: Vec<Vec<u32>>,
    start_node: usize,
}

fn parse_and_trcl_input(input: &str) -> TunnelState {
    let parsed_lines: Vec<(String, u32, Vec<String>)> = parse::parse_input(input);
    let mut valves_a = Valves::new();
    let num_nodes = parsed_lines.len();

    let mut tunnel_state = TunnelState {
        flows: vec![0; num_nodes],
        path_weights: vec![vec![u32::MAX; num_nodes]; num_nodes],
        start_node: valves_a.get_or_insert_index("AA".to_string()),
    };

    // turn the parsed lines into a nice matrix
    for (node_name, flow, connections) in parsed_lines {
        let node_ind = valves_a.get_or_insert_index(node_name);
        tunnel_state.flows[node_ind] = flow;
        tunnel_state.path_weights[node_ind][node_ind] = 0;
        for target_node in connections {
            let target_ind = valves_a.get_or_insert_index(target_node);
            tunnel_state.path_weights[node_ind][target_ind] = 1;
        }
    }

    // then compute the transitive closure
    // this is guaranteed to end after N steps since there are no negative-length cycles
    //      and N is small enough that N^4 is trivial; we still detect early stopping (why not)
    //      but we have an upper bound on the number of required iterations
    // also we don't have to worry about overflow since the original weights are all 1, so maximum
    //      ending weight is N, which is WELL below the bound of the datatype
    for _ in 0..num_nodes {
        let mut improvement_found = false;
        for start_node in 0..num_nodes {
            for mid_node in 0..num_nodes {
                let start_to_mid = tunnel_state.path_weights[start_node][mid_node];
                if mid_node == start_node || start_to_mid == u32::MAX {
                    continue;
                }

                for target_node in 0..num_nodes {
                    let mid_to_target = tunnel_state.path_weights[mid_node][target_node];

                    if target_node == start_node
                        || target_node == mid_node
                        || mid_to_target == u32::MAX
                    {
                        continue;
                    }

                    let new_weight = start_to_mid + mid_to_target;
                    let old_weight = tunnel_state.path_weights[start_node][target_node];
                    if new_weight < old_weight {
                        tunnel_state.path_weights[start_node][target_node] = new_weight;
                        improvement_found = true;
                    }
                }
            }
        }
        if !improvement_found {
            break;
        }
    }

    // now, we're gonna strip out all the 0-flow nodes that are not the start node
    // just keeps the graph compact and clean
    let old_tunnel_state = tunnel_state;

    let keep_index = |old_index: usize| {
        old_tunnel_state.flows[old_index] > 0 || old_tunnel_state.start_node == old_index
    };

    let num_kept_nodes = (0..num_nodes).filter(|f| keep_index(*f)).count();

    let mut reindexer: Valves<usize> = Valves::new();
    let mut tunnel_state = TunnelState {
        flows: vec![0; num_kept_nodes],
        path_weights: vec![vec![u32::MAX; num_kept_nodes]; num_kept_nodes],
        start_node: reindexer.get_or_insert_index(old_tunnel_state.start_node),
    };

    for old_index in 0..num_nodes {
        if !keep_index(old_index) {
            continue;
        }

        let new_index = reindexer.get_or_insert_index(old_index);
        tunnel_state.flows[new_index] = old_tunnel_state.flows[old_index];

        for old_index_target in 0..num_nodes {
            if !keep_index(old_index_target) {
                continue;
            }

            let new_target_index = reindexer.get_or_insert_index(old_index_target);

            tunnel_state.path_weights[new_index][new_target_index] =
                old_tunnel_state.path_weights[old_index][old_index_target];
        }
    }

    tunnel_state
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

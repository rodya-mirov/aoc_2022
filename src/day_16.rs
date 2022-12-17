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

    let (input, start_locations) = parse_and_prep_input(input);

    let num_nodes = input.len();

    let mut valves_open = ValvesOpen::new(num_nodes);
    let mut flow_rate_by_node: Vec<u32> = vec![0; num_nodes];
    let mut tunnels_by_node: Vec<Vec<(usize, u32)>> = vec![Vec::new(); num_nodes];

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
        tunnels_by_node: &[Vec<(usize, u32)>],
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

        for (connecting_position, connecting_cost) in
            tunnels_by_node[current_position].iter().copied()
        {
            if time_remaining < connecting_cost {
                continue;
            }

            let flow_after_move = recurse(
                valves_open,
                flow_rate_by_node,
                tunnels_by_node,
                connecting_position,
                time_remaining - connecting_cost,
                cache,
            );
            best = best.max(flow_after_move);
        }

        cache.insert(state, best);

        best
    }

    let mut best = u32::MIN;
    for (start_pos, cost_to_start) in start_locations {
        let next_best = recurse(
            valves_open,
            flow_rate_by_node.as_slice(),
            tunnels_by_node.as_slice(),
            start_pos,
            total_time - cost_to_start,
            &mut best_at_state,
        );
        best = best.max(next_best);
    }

    best
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

    // TODO: after all this prep and culling, needs either better heuristics or a cache
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

        // TODO if needed: cache? if we end up in a state twice, skip it

        // if we're out of time, or every valve is open, there's no more room for benefit, just
        // stop the simulation and be done
        if a_time_remaining == 0 || valves_open.all_open() {
            return;
        }

        // this isn't a perfect metric but it's probably good enough -- if it's simply impossible
        // to exceed the best total ever from here, then stop
        if valves_open.max_remaining_flow(
            tunnel_state,
            a_pos,
            a_time_remaining,
            b_pos,
            b_time_remaining,
        ) + flow_so_far
            <= *best_total_ever
        {
            if a_time_remaining >= 40 {
                println!(
                    "Killing a branch with times remaining {}/{}",
                    a_time_remaining, b_time_remaining
                );
            }
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
        /// TODO: basically I just don't think this heuristic is good enough
        pub fn max_remaining_flow(
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
    //      and N is small enough that N^4 is trivial, so we're not gonna bother detecting early stopping
    // also we don't have to worry about overflow since the original weights are all 1, so maximum
    //      ending weight is N, which is WELL below the bound of the datatype
    for big_round in 0..num_nodes {
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
        if improvement_found {
            if big_round + 1 < num_nodes {
                println!(
                    "After {} rounds ({} possible), improvement was found; will try again",
                    big_round, num_nodes
                );
            } else {
                println!("After {} rounds ({} possible), improvement was found, but we'll stop anyway, which is a little weird", big_round, num_nodes);
            }
        } else {
            println!(
                "After {} rounds ({} possible), no improvement was found; stopping now.",
                big_round, num_nodes
            );
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

// Return: list of rows (node_ind, flow_rate, list<(connecting_node_ind, time_to_get_to_node)>
// Also return list<(starting_node_ind, time_to_start_at_node)>
fn parse_and_prep_input(input: &str) -> (Vec<(usize, u32, Vec<(usize, u32)>)>, Vec<(usize, u32)>) {
    let parsed_lines: Vec<(String, u32, Vec<String>)> = parse::parse_input(input);
    // First, acknowledge that each path has a variable cost (not just 1)
    let mut parsed_lines: Vec<(String, u32, Vec<(String, u32)>)> = parsed_lines
        .into_iter()
        .map(|(name, flow, paths)| {
            let paths = paths
                .into_iter()
                .map(|connection| (connection, 1))
                .collect();
            (name, flow, paths)
        })
        .collect();

    let mut paths_to_start: Vec<(String, u32)> = vec![("AA".to_string(), 0)];

    // Then, sequentially remove 0-flow nodes, until they're all gone

    // sweep through all nodes that connect to this node, and replace them with connections
    // through this node with higher cost, then remove redundant connections
    // (e.g. GJ for 2, GJ for 2, and GJ for 13) and self-connections
    fn clean_connections(
        deleted_node_name: &str,
        connections_from_deleted: &[(String, u32)],
        source: Option<&str>,
        connections_from_source: &mut Vec<(String, u32)>,
    ) {
        // early stop if this source doesn't go to the deleted node
        if !connections_from_source
            .iter()
            .any(|(target, _)| target == &deleted_node_name)
        {
            return;
        }

        // replace all the old connections with altered connections (as needed)
        // we do this by replacing the vector of connections
        let old_connections = std::mem::take(connections_from_source);
        for (target, cost) in old_connections {
            if target != deleted_node_name {
                connections_from_source.push((target, cost));
            } else {
                for (new_target, addl_cost) in connections_from_deleted.iter() {
                    // skip self-connections right away; these can arise from cycles in the graph
                    // they don't really hurt anything but they're a waste of time
                    if Some(new_target.as_str()) == source {
                        continue;
                    }
                    let new_cost = cost + addl_cost;
                    connections_from_source.push((new_target.to_string(), new_cost));
                }
            }
        }

        // now again we replace the vector with a new one; here we dedupe
        let mut old_connections: Vec<(String, u32)> = std::mem::take(connections_from_source);

        // sorting order is important -- we sort by target, then subsort by cost (ascending)
        // so the first time we see a target, it's the one we want to keep
        old_connections.sort();

        let mut old_connections_iter = old_connections.into_iter();
        let mut to_insert = old_connections_iter.next().unwrap();

        while let Some(next) = old_connections_iter.next() {
            if to_insert.0 != next.0 {
                connections_from_source.push(to_insert);
                to_insert = next;
            }
            // else skip the edge entirely, it's redundant
        }

        connections_from_source.push(to_insert);
    }

    fn pop_next_removable_node(
        parsed_lines: &mut Vec<(String, u32, Vec<(String, u32)>)>,
    ) -> Option<(String, u32, Vec<(String, u32)>)> {
        parsed_lines
            .iter()
            .enumerate()
            // find a line with no flow
            .filter(|(_, (_, flow, _))| *flow == 0)
            // grab the index
            .map(|(i, _)| i)
            .next()
            // pop it out of the vector and return it
            .map(|i| parsed_lines.remove(i))
    }

    while let Some((deleted_node_name, _, connections_from_deleted)) =
        pop_next_removable_node(&mut parsed_lines)
    {
        for (ref source, _, ref mut connections_from_source) in parsed_lines.iter_mut() {
            clean_connections(
                &deleted_node_name,
                &connections_from_deleted,
                Some(source),
                connections_from_source,
            );
        }

        clean_connections(
            &deleted_node_name,
            &connections_from_deleted,
            None,
            &mut paths_to_start,
        );
    }

    // Finally, give everything sequential names and be done
    let mut valves = Valves::new();

    let paths_to_start = paths_to_start
        .into_iter()
        .map(|(name, cost)| (valves.get_or_insert_index(name), cost))
        .collect();
    let parsed_lines = parsed_lines
        .into_iter()
        .map(|(name, flow, connections)| {
            let connections: Vec<(usize, u32)> = connections
                .into_iter()
                .map(|(target, cost)| (valves.get_or_insert_index(target), cost))
                .collect();
            (valves.get_or_insert_index(name), flow, connections)
        })
        .collect();

    (parsed_lines, paths_to_start)
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

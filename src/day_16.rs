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

    // currently:
    //      n=18 ->  6.7s
    //      n=19 -> 11.9s
    //      n=20 -> 21.8s
    //      n=26 -> :sob:
    let val = b_with_input(&contents, 20);

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

    let (input, start_locations) = parse_and_prep_input(input);

    let num_nodes = input.len();

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

    // simple recursion with no pruning, just a DFS
    // stop conditions: time runs out, or all valves are open
    fn recurse(
        valves_open: ValvesOpen,
        flow_rate_by_node: &[u32],
        tunnels_by_node: &[Vec<(usize, u32)>],
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
        if a_time_remaining < b_time_remaining {
            recurse(
                valves_open,
                flow_rate_by_node,
                tunnels_by_node,
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
        if valves_open.max_remaining_flow(flow_rate_by_node, a_time_remaining, a_pos, b_pos)
            + flow_so_far
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

        if !valves_open.is_open(a_pos) {
            let mut valves_open = valves_open.clone();
            valves_open.open(a_pos);
            let direct_flow = flow_rate_by_node[a_pos] * (a_time_remaining - 1);
            recurse(
                valves_open,
                flow_rate_by_node,
                tunnels_by_node,
                // B becomes A
                b_pos,
                b_time_remaining,
                // A moves to new position, reduces time, and becomes B
                a_pos,
                a_time_remaining - 1,
                flow_so_far + direct_flow,
                best_total_ever,
            );
        }

        // We assume the "greedy" solution is a good first guess -- go to the most high-value
        // locations first, establish a high baseline goal, so we can cull lots of branches that
        // were never competitive without walking the whole way down
        let mut ideal_positions: Vec<(usize, u32)> =
            tunnels_by_node[a_pos].iter().copied().collect();
        ideal_positions.sort_by_key(|(node, cost)| {
            // factor in the time it takes to get there
            std::cmp::Reverse(
                if valves_open.is_open(*node) || *cost >= a_time_remaining + 1 {
                    0
                } else {
                    flow_rate_by_node[*node] * (a_time_remaining - *cost - 1)
                },
            )
        });
        for (connecting_position, cost) in ideal_positions.into_iter() {
            if cost > a_time_remaining {
                continue;
            }
            recurse(
                valves_open,
                flow_rate_by_node,
                tunnels_by_node,
                // B becomes A
                b_pos,
                b_time_remaining,
                // A moves to new position, reduces time, and becomes B
                connecting_position,
                a_time_remaining - cost,
                // the totals didn't change since we just did a move
                flow_so_far,
                best_total_ever,
            );
        }
    }

    let mut best_total_ever = 0;
    for (pos_a, starting_cost_a) in start_locations.iter().copied() {
        for (pos_b, starting_cost_b) in start_locations.iter().copied() {
            recurse(
                valves_open,
                flow_rate_by_node.as_slice(),
                tunnels_by_node.as_slice(),
                pos_a,
                total_time - starting_cost_a,
                pos_b,
                total_time - starting_cost_b,
                0,
                &mut best_total_ever,
            );
        }
    }

    best_total_ever
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
        .map(|(name, cost)| (valves.get_or_insert_index(&name), cost))
        .collect();
    let parsed_lines = parsed_lines
        .into_iter()
        .map(|(name, flow, connections)| {
            let connections: Vec<(usize, u32)> = connections
                .into_iter()
                .map(|(target, cost)| (valves.get_or_insert_index(&target), cost))
                .collect();
            (valves.get_or_insert_index(&name), flow, connections)
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

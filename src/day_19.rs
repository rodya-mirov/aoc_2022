fn input() -> String {
    std::fs::read_to_string("input/input_19.txt").expect("Should be able to read the file")
}

pub fn a() -> String {
    let contents = input();

    let val = a_with_input(&contents, 24);

    val.to_string()
}

fn a_with_input(input: &str, total_time: i32) -> i32 {
    let input = parse::parse_input(input);

    let mut total_score = 0;

    for blueprint in input {
        let optimal_output = optimal_geode_output(blueprint, total_time);
        // note some of them can't produce geodes

        total_score += blueprint.id * optimal_output;
    }

    total_score
}

pub fn b() -> String {
    let contents = input();

    let val = b_with_input(&contents);

    val.to_string()
}

fn b_with_input(input: &str) -> i32 {
    let input = parse::parse_input(input);

    let best_scores: Vec<i32> = input
        .into_iter()
        .take(3)
        .map(|bp| optimal_geode_output(bp, 32))
        .collect();

    let mut out = 1;

    for score in best_scores {
        out *= score;
    }

    out
}

fn optimal_geode_output(blueprint: Blueprint, total_time: i32) -> i32 {
    #[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
    struct ResourceState {
        // spendable resources
        ore: i32,
        clay: i32,
        obsidian: i32,
        // useful robots
        ore_robots: i32,
        clay_robots: i32,
        obsidian_robots: i32,
        // note we just compute the total output of a geode robot instantly
        // rather than building a geode robot and simulating it harvesting a geode every timestep
    }

    impl ResourceState {
        fn new() -> Self {
            ResourceState {
                ore: 0,
                clay: 0,
                obsidian: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
            }
        }

        fn wait_time(mut self, time: i32) -> Self {
            debug_assert!(time >= 0);

            if time > 0 {
                self.ore += self.ore_robots * time;
                self.clay += self.clay_robots * time;
                self.obsidian += self.obsidian_robots * time;
            }

            self
        }
    }

    fn dfs(
        blueprint: Blueprint,
        time_remaining: i32,
        geodes_so_far: i32,
        resources: ResourceState,
    ) -> i32 {
        if time_remaining <= 0 {
            return geodes_so_far;
        }

        // we can only buy one robot per turn; so our speeds for each resource never needs to exceed
        // the maximum amount we can spend per turn
        let max_ore_speed = blueprint.clay_robot_ore_cost.max(blueprint.obsidian_robot_ore_cost).max(blueprint.geode_robot_ore_cost);
        let max_clay_speed = blueprint.obsidian_robot_clay_cost;
        let max_obsidian_speed = blueprint.geode_robot_obsidian_cost;

        // this gives you the time to when you'll have enough resources to purchase a robot, plus
        // the time for the robot to actually be produces
        let time_to_ore_robot = time_to_amount(
            blueprint.ore_robot_ore_cost,
            resources.ore,
            resources.ore_robots,
        )
        .saturating_add(1);

        let time_to_clay_robot = time_to_amount(
            blueprint.clay_robot_ore_cost,
            resources.ore,
            resources.ore_robots,
        )
        .saturating_add(1);

        let time_to_obsidian_robot = time_to_amount(
            blueprint.obsidian_robot_ore_cost,
            resources.ore,
            resources.ore_robots,
        )
        .max(time_to_amount(
            blueprint.obsidian_robot_clay_cost,
            resources.clay,
            resources.clay_robots,
        ))
        .saturating_add(1);

        let time_to_geode_robot = time_to_amount(
            blueprint.geode_robot_ore_cost,
            resources.ore,
            resources.ore_robots,
        )
        .max(time_to_amount(
            blueprint.geode_robot_obsidian_cost,
            resources.obsidian,
            resources.obsidian_robots,
        ))
        .saturating_add(1);

        // basically all our options are "wait until we have resources to build a robot, then build it"

        let mut best = geodes_so_far;

        if resources.ore_robots < max_ore_speed && time_to_ore_robot < time_remaining {
            let mut next_resources = resources.wait_time(time_to_ore_robot);
            next_resources.ore -= blueprint.ore_robot_ore_cost;
            next_resources.ore_robots += 1;

            best = best.max(dfs(
                blueprint,
                time_remaining - time_to_ore_robot,
                geodes_so_far,
                next_resources,
            ));
        }

        if resources.clay_robots < max_clay_speed && time_to_clay_robot < time_remaining {
            let mut next_resources = resources.wait_time(time_to_clay_robot);
            next_resources.ore -= blueprint.clay_robot_ore_cost;
            next_resources.clay_robots += 1;

            best = best.max(dfs(
                blueprint,
                time_remaining - time_to_clay_robot,
                geodes_so_far,
                next_resources,
            ));
        }

        if resources.obsidian_robots < max_obsidian_speed && time_to_obsidian_robot < time_remaining {
            let mut next_resources = resources.wait_time(time_to_obsidian_robot);
            next_resources.ore -= blueprint.obsidian_robot_ore_cost;
            next_resources.clay -= blueprint.obsidian_robot_clay_cost;
            next_resources.obsidian_robots += 1;

            best = best.max(dfs(
                blueprint,
                time_remaining - time_to_obsidian_robot,
                geodes_so_far,
                next_resources,
            ));
        }

        if time_to_geode_robot < time_remaining {
            let mut next_resources = resources.wait_time(time_to_geode_robot);
            next_resources.ore -= blueprint.geode_robot_ore_cost;
            next_resources.obsidian -= blueprint.geode_robot_obsidian_cost;

            // note that the geode robot needs one more unit of time to actually _get_ a geode
            // after it's produced (but it produces one per minute thereafter)
            let geode_income = time_remaining - time_to_geode_robot;

            best = best.max(dfs(
                blueprint,
                time_remaining - time_to_geode_robot,
                geode_income + geodes_so_far,
                next_resources,
            ));
        }

        best
    }

    dfs(blueprint, total_time, 0, ResourceState::new())
}

fn time_to_amount(goal: i32, current: i32, speed: i32) -> i32 {
    let needed = goal - current;
    if needed <= 0 {
        0
    } else if speed == 0 {
        i32::MAX
    } else {
        let out = needed / speed;
        if out * speed >= needed {
            out
        } else {
            out + 1
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Blueprint {
    id: i32,
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
}

mod parse {
    use crate::day_19::Blueprint;
    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{eof, map},
        IResult,
    };

    fn parse_num(input: &str) -> IResult<&str, i32> {
        if input.starts_with("-") {
            let (input, _) = tag("-")(input)?;
            let (input, num) = map(digit1, |digits: &str| digits.parse::<i32>().unwrap())(input)?;
            Ok((input, -num))
        } else {
            let (input, val) = map(digit1, |digits: &str| digits.parse::<i32>().unwrap())(input)?;

            Ok((input, val))
        }
    }

    fn parse_line_helper(input: &str) -> IResult<&str, Blueprint> {
        let (input, _) = tag("Blueprint ")(input)?;
        let (input, id) = parse_num(input)?;
        let (input, _) = tag(": Each ore robot costs ")(input)?;
        let (input, ore_robot_ore_cost) = parse_num(input)?;
        let (input, _) = tag(" ore. Each clay robot costs ")(input)?;
        let (input, clay_robot_ore_cost) = parse_num(input)?;
        let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;
        let (input, obsidian_robot_ore_cost) = parse_num(input)?;
        let (input, _) = tag(" ore and ")(input)?;
        let (input, obsidian_robot_clay_cost) = parse_num(input)?;
        let (input, _) = tag(" clay. Each geode robot costs ")(input)?;
        let (input, geode_robot_ore_cost) = parse_num(input)?;
        let (input, _) = tag(" ore and ")(input)?;
        let (input, geode_robot_obsidian_cost) = parse_num(input)?;
        let (input, _) = tag(" obsidian.")(input)?;
        let (_, _) = eof(input)?;

        Ok((
            "",
            Blueprint {
                id,
                ore_robot_ore_cost,
                clay_robot_ore_cost,
                obsidian_robot_ore_cost,
                obsidian_robot_clay_cost,
                geode_robot_ore_cost,
                geode_robot_obsidian_cost,
            },
        ))
    }

    fn parse_line(input: &str) -> Blueprint {
        parse_line_helper(input).unwrap().1
    }

    pub(super) fn parse_input(input: &str) -> Vec<Blueprint> {
        input.lines().map(parse_line).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::parse_line;
        use super::Blueprint;

        #[test]
        fn sample_lines() {
            assert_eq!(parse_line("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 2 ore and 16 obsidian."), Blueprint {
                id: 1,
                ore_robot_ore_cost: 4,
                clay_robot_ore_cost: 4,
                obsidian_robot_ore_cost: 4,
                obsidian_robot_clay_cost: 14,
                geode_robot_ore_cost: 2,
                geode_robot_obsidian_cost: 16
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_STR: &'static str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn sample_a_smaller_1() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let input = parse::parse_input(input);

        assert_eq!(optimal_geode_output(input[0], 19), 1);
        assert_eq!(optimal_geode_output(input[0], 20), 2);
        assert_eq!(optimal_geode_output(input[0], 21), 3);
        assert_eq!(optimal_geode_output(input[0], 22), 5);
        assert_eq!(optimal_geode_output(input[0], 23), 7);
        assert_eq!(optimal_geode_output(input[0], 24), 9);
    }

    #[test]
    fn sample_a_smaller_2() {
        let input = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        let input = parse::parse_input(input);
        assert_eq!(optimal_geode_output(input[0], 24), 12);
    }

    #[test]
    fn sample_a() {
        let input = SAMPLE_INPUT_STR;
        let actual = a_with_input(input, 24);
        assert_eq!(actual, 33);
    }
}

use adventofcode25::{input_path, read_lines};
use pathfinding::prelude::astar;
use z3::{Optimize, SatResult};

const DAY: u8 = 10;

fn main() {
    let machines = parse_input(&input_path(DAY));
    solve_part1(&machines);
    solve_part2(&machines);
}

struct Machine {
    required_lights: LightState,
    buttons: Vec<Vec<u16>>,
    joltage_requirements: JoltageRequirements,
}

type LightState = Vec<bool>;
type JoltageRequirements = Vec<u16>;

fn parse_input(input: &str) -> Vec<Machine> {
    let machines = read_lines(input)
        .expect("Failed to read input file")
        .map_while(Result::ok)
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let light_config = parts[0];
            let required_lights = light_config[1..light_config.len() - 1]
                .chars()
                .map(|c| c == '#')
                .collect::<LightState>();

            let joltage_requirements_part = parts[parts.len() - 1];
            let joltage_requirements = joltage_requirements_part
                [1..joltage_requirements_part.len() - 1]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<JoltageRequirements>();

            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|button_part| {
                    button_part[1..button_part.len() - 1]
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<u16>>()
                })
                .collect::<Vec<Vec<u16>>>();
            Machine {
                required_lights,
                buttons,
                joltage_requirements,
            }
        })
        .collect();
    machines
}

fn create_successors_part1(
    buttons: &Vec<Vec<u16>>,
) -> impl Fn(&LightState) -> Vec<(LightState, u64)> + '_ {
    move |state: &LightState| {
        let mut successors = Vec::new();
        for button in buttons {
            let mut new_state = state.clone();
            for &light_index in button {
                new_state[light_index as usize] = !new_state[light_index as usize];
            }
            successors.push((new_state, 1));
        }
        successors
    }
}

fn create_heuristic_part1(_required_lights: &LightState) -> impl Fn(&LightState) -> u64 + '_ {
    move |_state: &LightState| 1
}

fn solve_part1(machines: &Vec<Machine>) -> u64 {
    let mut result = 0u64;
    for machine in machines {
        let start = vec![false; machine.required_lights.len()];
        let shortest_sequence = astar(
            &start,
            create_successors_part1(&machine.buttons),
            create_heuristic_part1(&machine.required_lights),
            |state| *state == machine.required_lights,
        );
        result += shortest_sequence.unwrap().1;
    }
    println!("Part 1: {}", result);
    result
}

fn solve_part2(machines: &Vec<Machine>) -> u64 {
    let mut result = 0u64;

    for machine in machines {
        let optimize = Optimize::new();
        let num_buttons = machine.buttons.len();
        let num_counters = machine.joltage_requirements.len();

        let button_vars: Vec<_> = (0..num_buttons)
            .map(|i| z3::ast::Int::new_const(format!("b{}", i)))
            .collect();

        let zero = z3::ast::Int::from_i64(0);
        for var in &button_vars {
            optimize.assert(&var.ge(&zero));
        }

        for counter_idx in 0..num_counters {
            let target = machine.joltage_requirements[counter_idx] as i64;

            let mut sum_terms: Vec<z3::ast::Int> = Vec::new();
            for (btn_idx, button) in machine.buttons.iter().enumerate() {
                if button.contains(&(counter_idx as u16)) {
                    sum_terms.push(button_vars[btn_idx].clone());
                }
            }

            let sum = if sum_terms.is_empty() {
                z3::ast::Int::from_i64(0)
            } else {
                let refs: Vec<_> = sum_terms.iter().collect();
                z3::ast::Int::add(&refs)
            };

            let target_val = z3::ast::Int::from_i64(target);
            optimize.assert(&sum.eq(&target_val));
        }

        let var_refs: Vec<_> = button_vars.iter().collect();
        let total_presses = z3::ast::Int::add(&var_refs);
        optimize.minimize(&total_presses);

        if optimize.check(&[]) == SatResult::Sat {
            let model = optimize.get_model().unwrap();
            let mut machine_presses = 0u64;
            for var in &button_vars {
                let val = model.eval(var, true).unwrap().as_i64().unwrap();
                machine_presses += val as u64;
            }
            result += machine_presses;
        }
    }
    println!("Part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse_input(&example_path(DAY))), 7);
    }

    #[test]
    fn part1_real() {
        solve_part1(&parse_input(&input_path(DAY)));
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse_input(&example_path(DAY))), 33);
    }

    #[test]
    fn part2_real() {
        solve_part2(&parse_input(&input_path(DAY)));
    }
}

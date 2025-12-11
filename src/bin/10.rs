advent_of_code::solution!(10);

use std::collections::{HashSet, VecDeque};

use good_lp::{Solution, SolverModel, constraint, default_solver, variable};

mod machine_parser {
    use super::Machine;
    use nom::{
        IResult, Parser,
        branch::alt,
        character::complete::{char, multispace0, multispace1, u64},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{delimited, preceded},
    };

    fn parse_target_state(input: &str) -> IResult<&str, Vec<bool>> {
        delimited(
            char('['),
            many1(map(alt((char('.'), char('#'))), |c| c == '#')),
            char(']'),
        )
        .parse(input)
    }

    fn parse_button(input: &str) -> IResult<&str, Vec<usize>> {
        let number = map(u64, |n| n as usize);
        let sep = delimited(multispace0, char(','), multispace0);
        delimited(
            char('('),
            delimited(multispace0, separated_list1(sep, number), multispace0),
            char(')'),
        )
        .parse(input)
    }

    fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
        separated_list1(multispace1, parse_button).parse(input)
    }

    fn parse_joltages(input: &str) -> IResult<&str, Vec<usize>> {
        let number = map(u64, |n| n as usize);
        let sep = delimited(multispace0, char(','), multispace0);
        delimited(
            char('{'),
            delimited(multispace0, separated_list1(sep, number), multispace0),
            char('}'),
        )
        .parse(input)
    }

    pub fn parse_machine(input: &str) -> IResult<&str, Machine> {
        map(
            (
                parse_target_state,
                preceded(multispace1, parse_buttons),
                preceded(multispace1, parse_joltages),
            ),
            |(target_state, buttons, joltages)| Machine {
                target_state,
                buttons,
                joltages,
            },
        )
        .parse(input)
    }
}

#[derive(Debug)]
struct Machine {
    target_state: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn from_str(line: &str) -> Machine {
        match machine_parser::parse_machine(line) {
            Ok((_, machine)) => machine,
            Err(e) => panic!("Failed to parse machine '{line}': {e}"),
        }
    }

    fn num_lights(&self) -> usize {
        self.target_state.len()
    }

    fn solve_state(&self) -> u64 {
        let num_lights = self.num_lights();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let initial_state = vec![false; num_lights];
        queue.push_back((initial_state.clone(), 0));
        visited.insert(initial_state);

        while let Some((current_state, presses)) = queue.pop_front() {
            // Check if we reached the target state
            if current_state == self.target_state {
                return presses;
            }

            // Try pressing each button
            for button_wiring in &self.buttons {
                let mut next_state = current_state.clone();

                // Handle effect of button press
                for &light_index in button_wiring {
                    next_state[light_index] = !next_state[light_index];
                }

                // If this state hasn't been visited, add it to the queue
                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, presses + 1));
                }
            }
        }
        panic!("Target state unreachable!");
    }

    /// Gemini Autocompleted this lp solver when asked, works as well just a bit slower
    /// Feeling Cooked LVL + 1
    #[allow(dead_code, clippy::all)]
    fn solve_joltage(&self) -> u64 {
        let num_counters = self.joltages.len();
        let num_buttons = self.buttons.len();

        if num_counters == 0 {
            return 0;
        }

        // 1. Set up the augmented matrix [A|t] for the system Ax = t.
        // The matrix will have `num_counters` rows and `num_buttons + 1` columns.
        let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; num_buttons + 1]; num_counters];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, button) in self.buttons.iter().enumerate() {
                if button.contains(&i) {
                    row[j] = 1.0;
                }
            }
            row[num_buttons] = self.joltages[i] as f64;
        }

        // 2. Perform Gaussian elimination to get the matrix into Reduced Row Echelon Form (RREF).
        let mut pivot_row = 0;
        for j in 0..num_buttons {
            if pivot_row >= num_counters {
                break;
            }

            let mut i = pivot_row;
            while matrix[i][j].abs() < 1e-9 {
                i += 1;
                if i >= num_counters {
                    i = pivot_row;
                    break;
                }
            }

            if i < num_counters && matrix[i][j].abs() > 1e-9 {
                matrix.swap(pivot_row, i);
                let pivot_val = matrix[pivot_row][j];
                for col in j..=num_buttons {
                    matrix[pivot_row][col] /= pivot_val;
                }

                for i2 in 0..num_counters {
                    if i2 != pivot_row {
                        let factor = matrix[i2][j];
                        for col in j..=num_buttons {
                            matrix[i2][col] -= factor * matrix[pivot_row][col];
                        }
                    }
                }
                pivot_row += 1;
            }
        }

        // Check for inconsistencies (e.g., 0 = 1), which means no solution.
        for i in 0..num_counters {
            let is_zero_row = matrix[i].iter().take(num_buttons).all(|&x| x.abs() < 1e-9);
            if is_zero_row && matrix[i][num_buttons].abs() > 1e-9 {
                panic!("No solution exists for this machine configuration.");
            }
        }

        // 3. Identify pivot and free variables.
        let mut pivot_vars = vec![-1; num_counters];
        let mut free_vars = Vec::new();
        let mut p_idx = 0;
        for j in 0..num_buttons {
            if p_idx < num_counters && matrix[p_idx][j].abs() > 1e-9 {
                pivot_vars[p_idx] = j as i32;
                p_idx += 1;
            } else {
                free_vars.push(j);
            }
        }

        // 4. Build expressions for pivot variables and sum(x_i) in terms of free variables.
        // exprs[i] will hold the expression for x_i.
        // Each expression is a constant term + coefficients for each free variable.
        let mut exprs: Vec<Vec<f64>> = vec![vec![0.0; free_vars.len() + 1]; num_buttons];
        for &free_idx in &free_vars {
            exprs[free_idx][free_vars.iter().position(|&v| v == free_idx).unwrap() + 1] = 1.0;
        }

        for i in 0..num_counters {
            if pivot_vars[i] != -1 {
                let pivot_var = pivot_vars[i] as usize;
                exprs[pivot_var][0] = matrix[i][num_buttons];
                for (k, &free_idx) in free_vars.iter().enumerate() {
                    exprs[pivot_var][k + 1] = -matrix[i][free_idx];
                }
            }
        }

        // Objective function: minimize sum(x_i). Build its expression in terms of free variables.
        let mut objective: Vec<f64> = vec![0.0; free_vars.len() + 1];
        for i in 0..num_buttons {
            for k in 0..=free_vars.len() {
                objective[k] += exprs[i][k];
            }
        }

        // 5. Recursively search the free variable space.
        let mut min_sum = f64::MAX;

        fn search(
            var_idx: usize,
            free_vars: &Vec<usize>,
            exprs: &Vec<Vec<f64>>,
            objective: &Vec<f64>,
            current_values: &mut Vec<i64>,
            min_sum: &mut f64,
        ) {
            if var_idx == free_vars.len() {
                let mut current_obj_val = objective[0];
                for (i, &val) in current_values.iter().enumerate() {
                    current_obj_val += objective[i + 1] * val as f64;
                }

                let rounded_sum = current_obj_val.round();
                if rounded_sum < *min_sum {
                    // Final check: ensure all xi are non-negative integers
                    for i in 0..exprs.len() {
                        let mut val = exprs[i][0];
                        for k in 0..current_values.len() {
                            val += exprs[i][k + 1] * current_values[k] as f64;
                        }
                        if val.round() < -1e-9 || (val.round() - val).abs() > 1e-9 {
                            return; // Not a valid non-negative integer solution
                        }
                    }
                    *min_sum = rounded_sum;
                }
                return;
            }

            // Heuristic search range. This is the trickiest part. A large range will be too slow.
            // These problems usually have small integer solutions.
            for val in 0..=150 {
                current_values.push(val);
                search(
                    var_idx + 1,
                    free_vars,
                    exprs,
                    objective,
                    current_values,
                    min_sum,
                );
                current_values.pop();
            }
        }

        search(0, &free_vars, &exprs, &objective, &mut vec![], &mut min_sum);

        if min_sum == f64::MAX {
            panic!("No solution found within the search range.");
        }
        min_sum as u64
    }

    fn solve_joltage_good_lp(&self) -> u64 {
        let num_buttons = self.buttons.len();

        let mut vars = good_lp::ProblemVariables::new();
        let presses = vars.add_vector(variable().min(0), num_buttons);

        let objective_expr: good_lp::Expression = presses.iter().sum();
        let mut model = vars.minimise(&objective_expr).using(default_solver);

        for (i, &joltage) in self.joltages.iter().enumerate() {
            let mut expr = good_lp::Expression::from(0);
            for (j, button) in self.buttons.iter().enumerate() {
                if button.contains(&i) {
                    expr += presses[j];
                }
            }
            model = model.with(constraint!(expr == joltage as f64));
        }

        let solution = model.solve().unwrap();
        solution.eval(objective_expr).round() as u64
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(Machine::from_str).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);
    println!("Machines: {:?}", machines);

    let minimal_moves = machines.iter().map(|m| m.solve_state()).sum();
    Some(minimal_moves)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse(input);

    let minimal_moves = machines.iter().map(|m| m.solve_joltage_good_lp()).sum();
    Some(minimal_moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two_line_2() {
        let line = "[#.#...#] (0,1,4,5,6) (1,4,6) (1,3,6) (1,2,5) (1,2,3) (4) (1,3,5,6) (0,1,2,4) (2,3,4,5,6) {23,74,43,39,55,46,57}";
        let machine = Machine::from_str(line);
        assert_eq!(machine.solve_joltage(), 88);
        assert_eq!(machine.solve_joltage_good_lp(), 88);
    }

    #[test]
    fn test_part_two_line_4() {
        let line = "[#.#..###.] (2,3,4,5) (3,4,5) (0,1,2,3,5,6) (1,3,7) (0,1,3,7) (0,1,4,5,7,8) (0,1,3,6,7,8) (1,2,3,5,6,8) (0,2,5,6) {59,48,29,42,22,50,38,35,27}";
        let machine = Machine::from_str(line);
        assert_eq!(machine.solve_joltage(), 71);
        assert_eq!(machine.solve_joltage_good_lp(), 71);
    }
}

use std::rc::Rc;
use itertools::Itertools;

use crate::solver::logical_solver::candidates::Candidates;
use crate::solver::Solver;
use crate::types::{CellPosition, Rule, SolutionStep, SolutionType};
use super::technique::Technique;

// Check if a cell candidate leads to an invalid grid state

pub struct NishioForcingChains;

impl Technique for NishioForcingChains {
  fn get_rule(&self) -> Rule { Rule::NishioForcingChains }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let temp_techniques: Vec<Rc<dyn Technique>> = Solver::default_techniques()
      .iter()
      .filter(|t| t.get_rule() == Rule::Candidates || t.is_grid_step() || t.is_candidate_validity_update_step())
      .cloned()
      .collect();
    
    let cells: Vec<CellPosition> = solver
      .get_all_empty_cells()
      .into_iter()
      .sorted_by_key(|cell| solver.candidates[cell.row][cell.col].len())
      .collect();

    for cell in cells {
      let invalid_values: Vec<u32> = solver.candidates[cell.row][cell.col].iter().sorted().filter(|&value| {
        let mut temp_solver: Solver = solver
          .clone()
          .with_techniques(temp_techniques.clone())
          .with_step_count_limit(solver.constraints.grid_size * 2);

        temp_solver.grid[cell.row][cell.col] = *value;
        temp_solver.candidates_active = false; // force to recalculate all candidates
        temp_solver.apply_rule(&mut Candidates.run(&temp_solver).first().unwrap());

        let result = temp_solver.logical_solve();
        result.solution_type == SolutionType::None
      }).copied().collect();

      if !invalid_values.is_empty() {
        return vec![self.build_solution_step(
          vec![], // find out where it got stuck?
          invalid_values,
          vec![],
          vec![ cell ],
        )]
      }
    }

    vec![]
  }
}

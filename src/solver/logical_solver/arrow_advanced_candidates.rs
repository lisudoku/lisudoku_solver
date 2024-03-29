use crate::solver::Solver;
use crate::solver::logical_solver::combinations::cell_combinations_runner::CellCombinationsRunner;
use crate::types::{SolutionStep, Rule, Area};
use super::technique::Technique;

// Eliminate arrow combinations that remove all candidates from 1 cell
pub struct ArrowAdvancedCandidates;

impl Technique for ArrowAdvancedCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::ArrowAdvancedCandidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    solver.constraints.arrows.iter().enumerate().flat_map(|(arrow_index, arrow)| {
      let cells = arrow.all_cells();

      // Running the algorithm for really long arrows will take too much time, so
      // wait for better opportunities
      if solver.count_empty_cells_in_list(&cells) > 8 {
        return vec![]
      }

      let mut arrow_combinatons_logic_factory = solver.arrow_combinatons_logic_factory.borrow_mut();
      let combination_logic = arrow_combinatons_logic_factory.create(arrow, solver);
      let mut runner = CellCombinationsRunner::new(solver, Box::new(combination_logic));
      let (_, combinations) = runner.run();

      let invalid_candidates = solver.eliminate_combinations(&combinations, &cells);

      // TODO: may eliminate same candidate twice?
      invalid_candidates.into_iter().map(|(cell, invalid_values)| {
        self.build_simple_solution_step(
          invalid_values,
          vec![ Area::Arrow(arrow_index) ],
          vec![ cell ]
        )
      }).collect::<Vec<_>>()
    }).collect()
  }
}

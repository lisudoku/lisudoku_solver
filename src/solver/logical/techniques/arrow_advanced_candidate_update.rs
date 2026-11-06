use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::logical::techniques::combinations::cell_combinations_runner::CellCombinationsRunner;
use crate::solver::types::CandidateSet;
use crate::types::{SolutionStep, Area};
use itertools::Itertools;

// Eliminate arrow combinations that remove all candidates from 1 cell

#[derive(Clone, Debug)]
pub(crate) struct ArrowAdvancedCandidateUpdate;

impl TechniqueDefinition for ArrowAdvancedCandidateUpdate {
  fn kind(&self) -> TechniqueKind { TechniqueKind::ArrowAdvancedCandidateUpdate }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateValidity }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let Some(constraint) = solver.state.puzzle.arrow() else { return vec![] };

    let mut steps: Vec<SolutionStep> = vec![];
    let grid_size = solver.state.puzzle.grid_size;
    let mut cell_previous_invalid_values: Vec<CandidateSet> = vec![CandidateSet::new(); grid_size * grid_size];

    for (arrow_index, arrow) in constraint.arrows.iter().enumerate() {
      let cells = arrow.all_cells();

      // Running the algorithm for really long arrows will take too much time, so
      // wait for better opportunities
      if solver.state.count_empty_cells_in_list(&cells) > 8 {
        continue
      }

      let mut arrow_combinatons_logic_factory = solver.caches.arrow_combinatons_logic_factory.borrow_mut();
      let combination_logic = arrow_combinatons_logic_factory.create(arrow, solver);
      let mut runner = CellCombinationsRunner::new(solver, Box::new(combination_logic));
      let (_, combinations) = runner.run();

      let invalid_candidates = solver.eliminate_combinations(&combinations, &cells);

      if invalid_candidates.is_empty() {
        continue
      }

      let current_steps = solver.deduplicate_eliminations(invalid_candidates, &mut cell_previous_invalid_values)
        .map(move |(cell, invalid_values)| {
          self.build_simple_solution_step(
            invalid_values.into_iter().collect_vec(),
            vec![Area::Arrow(arrow_index)],
            vec![cell],
          )
        });

      steps.extend(current_steps);
    }

    steps
  }
}

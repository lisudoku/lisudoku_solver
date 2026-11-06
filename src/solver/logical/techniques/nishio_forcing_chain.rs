use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::logical::techniques::candidate_initialization::CandidateInitialization;
use crate::types::{CellPosition, InvalidStateReason, SolutionStep, SolutionType};
use itertools::Itertools;

// Check if a cell candidate leads to an invalid grid state

#[derive(Clone, Debug)]
pub(crate) struct NishioForcingChain;

impl TechniqueDefinition for NishioForcingChain {
  fn kind(&self) -> TechniqueKind { TechniqueKind::NishioForcingChain }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let temp_technique_kinds: Vec<TechniqueKind> = LogicalSolver::all_techniques()
      .filter_map(|t| {
        if t.kind() == TechniqueKind::CandidateInitialization || t.phase() != TechniquePhase::CandidateElimination {
          return Some(t.kind())
        }
        None
      })
      .collect();

    let cells: Vec<CellPosition> = solver
      .state
      .all_empty_cells()
      .into_iter()
      /*
        Only look at cells with relatively few candidates.
        We could eliminate candidates from the other cells, but it is probably useless and also less human.
        That together with the massive performance gains makes it worth it.
      */
      .filter(|cell| solver.state.candidates[cell.row][cell.col].len() <= 4)
      // Sort because there is higher impact to remove candidates when there are fewer
      .sorted_by_key(|cell| solver.state.candidates[cell.row][cell.col].len())
      .collect();

    for cell in cells {
      let invalid_values: Vec<(u32, InvalidStateReason)> = (&solver.state.candidates[cell.row][cell.col]).into_iter().filter_map(|value| {
        let mut temp_solver: LogicalSolver = solver
          .clone()
          .with_techniques(temp_technique_kinds.clone())
          // the limit might need some tweaking to account for some techniques
          // that return individual steps per step group
          .with_step_count_limit(solver.state.puzzle.grid_size)
          // parent solve might have hint mode, but we want it off here
          .with_hint_mode(false);

        temp_solver.state.set_value(&cell, value);
        temp_solver.state.candidates_active = false; // force to recalculate all candidates
        temp_solver.apply_step(&mut CandidateInitialization.run(&temp_solver).first().unwrap());

        let result = temp_solver.solve();
        if result.solution_type != SolutionType::None {
          return None
        }

        Some((value, result.invalid_state_reason.unwrap()))
      }).collect();

      if !invalid_values.is_empty() {
        return invalid_values.into_iter().map(|(invalid_value, invalid_state_reason)| {
          SolutionStep {
            technique_kind: self.kind(),
            cells: vec![],
            values: vec![ invalid_value ],
            areas: vec![],
            affected_cells: vec![ cell ],
            grid: None,
            candidates: None,
            invalid_state_reason: Some(invalid_state_reason),
          }
        }).collect()
      }
    }

    vec![]
  }
}

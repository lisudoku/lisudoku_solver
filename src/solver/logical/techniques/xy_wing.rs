use itertools::Itertools;
use crate::solver::{GridCheckResult, SolveState};
use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{StepApplication, TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::types::CandidateSet;
use crate::types::{CellPosition, SolutionStep};

#[derive(Clone, Debug)]
pub struct XYWing;

impl TechniqueDefinition for XYWing {
  fn kind(&self) -> TechniqueKind { TechniqueKind::XYWing }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn step_application(&self) -> StepApplication { StepApplication::Custom }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let cells_with_2: Vec<CellPosition> = solver
      .state
      .all_empty_cells()
      .into_iter()
      .filter(|cell| solver.state.candidates[cell.row][cell.col].len() == 2)
      .collect();

    for xy_cell in cells_with_2 {
      let candidates = &solver.state.candidates[xy_cell.row][xy_cell.col];
      let values: Vec<u32> = candidates.into_iter().sorted().collect();
      let x = values[0];
      let y = values[1];
      let seen_cells = solver.state.cell_peers_with_candidates(&xy_cell, candidates);

      for xz_cell in &seen_cells {
        let candidates = &solver.state.candidates[xz_cell.row][xz_cell.col];
        if candidates.len() != 2 || !candidates.contains(&x) {
          continue
        }
        let z = candidates.into_iter().filter(|value| *value != x).next().unwrap();
        let need_candidates = &CandidateSet::from([ y, z ]);

        for yz_cell in &seen_cells {
          let candidates = &solver.state.candidates[yz_cell.row][yz_cell.col];
          if yz_cell == xz_cell || candidates != need_candidates {
            continue
          }

          // Found the XYZ triplet, now search for cells with candidate Z that both XZ and YZ see
          let affected_cells = solver.cells_peers_with_any_candidates(&vec![ *xz_cell, *yz_cell ], &CandidateSet::from([z]));

          if !affected_cells.is_empty() {
            return vec![
              self.build_solution_step(
                vec![ xy_cell, *xz_cell, *yz_cell ],
                vec![ x, y, z ],
                vec![],
                affected_cells,
              )
            ]
          }
        }
      }
    }

    vec![]
  }

  fn apply(&self, step: &SolutionStep, state: &mut SolveState, _step_count_limit: Option<usize>) -> GridCheckResult {
    for &CellPosition { row, col } in &step.affected_cells {
      // Remove Z as candidate
      state.candidates[row][col].remove(&step.values[2]);
    }
    GridCheckResult::solved()
  }
}

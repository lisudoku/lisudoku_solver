use crate::solver::{GridCheckResult, SolveState};
use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{StepApplication, TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::types::{CandidateGrid, CandidateSet};
use crate::types::SolutionStep;

#[derive(Clone, Debug)]
pub(crate) struct CandidateInitialization;

impl TechniqueDefinition for CandidateInitialization {
  fn kind(&self) -> TechniqueKind { TechniqueKind::CandidateInitialization }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn step_application(&self) -> StepApplication { StepApplication::Custom }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if solver.state.candidates_active {
      return vec![]
    }

    let mut candidates: Vec<Vec<Vec<u32>>> = vec![
      vec![ vec![]; solver.state.puzzle.grid_size ];
      solver.state.puzzle.grid_size
    ];
    for cell in solver.state.all_empty_cells() {
      candidates[cell.row][cell.col] = solver.state.compute_cell_candidates(&cell).into_iter().collect();
    }

    return vec![
      SolutionStep {
        technique_kind: self.kind(),
        cells: vec![],
        values: vec![],
        areas: vec![],
        affected_cells: vec![],
        grid: None,
        candidates: Some(candidates),
        invalid_state_reason: None,
      }
    ]
  }

  fn apply(&self, step: &SolutionStep, state: &mut SolveState, _step_count_limit: Option<usize>) -> GridCheckResult {
    state.candidates_active = true;
    state.candidates = CandidateGrid::from(
      step.candidates.as_ref().unwrap().into_iter()
        .map(|row| row.into_iter().map(|col|
          CandidateSet::from_iter(col.iter().copied())
        ).collect::<Vec<_>>())
        .collect::<Vec<_>>()
    );
    GridCheckResult::solved()
  }
}

use crate::solver::GridCheckResult;
use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::Technique;
use crate::solver::types::CandidateSet;
use crate::types::{Rule, SolutionStep};

pub struct Candidates;

impl Technique for Candidates {
  fn get_rule(&self) -> Rule { Rule::Candidates }

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
        rule: self.get_rule(),
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

  fn apply(&self, step: &SolutionStep, solver: &mut LogicalSolver) -> GridCheckResult {
    solver.state.candidates_active = true;
    solver.state.candidates = step.candidates.as_ref().unwrap().into_iter()
      .map(|row| row.into_iter().map(|col|
        CandidateSet::from_iter(col.iter().copied())
      ).collect())
      .collect();
    GridCheckResult::solved()
  }
}

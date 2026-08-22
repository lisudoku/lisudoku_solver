use crate::solver::Solver;
use crate::solver::checker::SolvedState;
use crate::solver::types::CandidateSet;
use crate::types::{Rule, SolutionStep};
use super::technique::Technique;
use itertools::Itertools;

pub struct Candidates;

impl Technique for Candidates {
  fn get_rule(&self) -> Rule { Rule::Candidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if solver.candidates_active {
      return vec![]
    }

    let mut candidates: Vec<Vec<Vec<u32>>> = vec![
      vec![ vec![]; solver.puzzle.grid_size ];
      solver.puzzle.grid_size
    ];
    for cell in &solver.all_empty_cells() {
      candidates[cell.row][cell.col] = solver.compute_cell_candidates(cell).into_iter().sorted().collect();
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

  fn apply(&self, step: &SolutionStep, solver: &mut Solver) -> SolvedState {
    solver.candidates_active = true;
    solver.candidates = step.candidates.as_ref().unwrap().into_iter()
      .map(|row| row.into_iter().map(|col|
        CandidateSet::from_iter(col.iter().copied())
      ).collect())
      .collect();
    SolvedState::solved()
  }
}

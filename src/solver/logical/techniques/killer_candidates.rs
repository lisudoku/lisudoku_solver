use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::Technique}, types::{Area, CellPosition, Rule, SolutionStep}};

// X can't be a candidate in this cell because the other empty cells
// can't be assigned to make the sum Y
pub struct KillerCandidates;

impl Technique for KillerCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::KillerCandidates }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let Some(constraint) = solver.state.puzzle.killer() else { return vec![] };

    for (killer_cage_index, killer_cage) in constraint.killer_cages.iter().enumerate() {
      if killer_cage.sum.is_none() {
        continue
      }
      let empty_cells: Vec<CellPosition> = killer_cage.region
        .iter()
        .copied()
        .filter(|cell| solver.state.value_at(cell) == 0)
        .collect();
      if empty_cells.is_empty() {
        continue
      }

      let sum: u32 = killer_cage.region.iter().map(|cell| solver.state.value_at(cell)).sum();
      let total_sum = killer_cage.sum.unwrap();
      let rest_sum: u32 = total_sum - sum;

      let invalid_sum_candidates = solver.detect_invalid_sum_candidates(&empty_cells, rest_sum);

      if invalid_sum_candidates.is_empty() {
        continue
      }

      return invalid_sum_candidates.into_iter().map(|(cell, invalid_values)| {
        self.build_simple_solution_step(
          invalid_values,
          vec![ Area::KillerCage(killer_cage_index) ],
          vec![ cell ],
        )
      }).collect()
    }

    vec![]
  }
}

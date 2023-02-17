use crate::{types::{SolutionStep, Rule, CellPosition}, solver::Solver};

pub trait Technique {
  fn is_grid_step(&self) -> bool { false }

  fn is_candidate_validity_update_step(&self) -> bool { false }

  fn get_rule(&self) -> Rule;

  fn run(&self, solver: &Solver) -> Vec<SolutionStep>;

  fn apply(&self, step: &SolutionStep, solver: &mut Solver) {
    if self.is_grid_step() {
      let CellPosition { row, col } = step.cells[0];
        let value = step.values[0];

        solver.grid[row][col] = value;

        if solver.candidates_active {
          solver.candidates[row][col].clear();
          solver.update_candidates(&step.affected_cells, value);
        }
    } else {
      for &CellPosition { row, col } in &step.affected_cells {
        for value in &step.values {
          solver.candidates[row][col].remove(value);
        }
      }
    }
  }
}

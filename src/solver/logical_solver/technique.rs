use crate::{solver::Solver, types::{Area, CellPosition, Rule, SolutionStep}};

pub trait Technique {
  fn is_grid_step(&self) -> bool { false }

  // All XCandidates type techniques shuold return true because they need to run first
  fn is_candidate_validity_update_step(&self) -> bool { false }

  fn get_rule(&self) -> Rule;

  fn build_solution_step(
    &self,
    cells: Vec<CellPosition>, values: Vec<u32>, areas: Vec<Area>, affected_cells: Vec<CellPosition>
  ) -> SolutionStep {
    SolutionStep::new(self.get_rule(), cells, values, areas, affected_cells)
  }

  fn build_simple_solution_step(
    &self,
    values: Vec<u32>, areas: Vec<Area>, affected_cells: Vec<CellPosition>
  ) -> SolutionStep {
    SolutionStep::new(self.get_rule(), vec![], values, areas, affected_cells)
  }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep>;

  // This is the default base implementation, but can be overridden
  fn apply(&self, step: &SolutionStep, solver: &mut Solver) {
    if self.is_grid_step() {
      let CellPosition { row, col } = step.cells[0];
        let value = step.values[0];

        if solver.step_count_limit.is_some() {
          // In this mode we can apply multiple steps at once and
          // we could find naked and hidden single for the same cell
          assert!(
            solver.grid[row][col] == 0 || solver.grid[row][col] == value,
            "Attempted to overwrite cell with different value"
          );
        } else {
          assert_eq!(solver.grid[row][col], 0, "Attempted to overwrite cell");
        }

        if solver.grid[row][col] == 0 {
          solver.grid[row][col] = value;

          if solver.candidates_active {
            solver.candidates[row][col].clear();
            solver.update_candidates(&step.affected_cells, value);
          }
        }
    } else if self.apply_corresponding_indices() {
      for (index, cell) in step.affected_cells.iter().enumerate() {
        let value = step.values[index];
        solver.candidates[cell.row][cell.col].remove(&value);
      }
    } else {
      for &CellPosition { row, col } in &step.affected_cells {
        for value in &step.values {
          solver.candidates[row][col].remove(value);
        }
      }
    }
  }

  fn apply_corresponding_indices(&self) -> bool { false }
}

use std::collections::HashSet;
use crate::{solver::{Solver, checker::SolvedState}, types::{Area, CellPosition, InvalidStateReason, InvalidStateType, Rule, SolutionStep}};

pub trait Technique {
  // It is a grid step if it fills in a value in the grid
  fn is_grid_step(&self) -> bool { false }

  // All XCandidates type techniques should return true because they need to run first
  // These are the techniques that enforce constraints
  fn is_candidate_validity_update_step(&self) -> bool { false }

  fn get_rule(&self) -> Rule;

  fn build_solution_step(
    &self,
    cells: Vec<CellPosition>, values: Vec<u32>, areas: Vec<Area>, affected_cells: Vec<CellPosition>
  ) -> SolutionStep {
    SolutionStep::new(self.get_rule(), cells, values, areas, affected_cells)
  }

  fn build_grid_solution_step(
    &self, cells: Vec<CellPosition>, values: Vec<u32>, areas: Vec<Area>, solver: &Solver,
  ) -> SolutionStep {
    assert!(self.is_grid_step());
    let affected_cells = if solver.candidates_active {
      assert_eq!(cells.len(), 1);
      let cell = cells[0];
      let value = values[0];
      let values_set = &HashSet::from([value]);
      solver.get_affected_by_cell(&cell, values_set)
    } else {
      vec![]
    };
    self.build_solution_step(cells, values, areas, affected_cells)
  }

  fn build_simple_solution_step(
    &self,
    values: Vec<u32>, areas: Vec<Area>, affected_cells: Vec<CellPosition>
  ) -> SolutionStep {
    self.build_solution_step(vec![], values, areas, affected_cells)
  }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep>;

  // This is the default base implementation, but can be overridden
  fn apply(&self, step: &SolutionStep, solver: &mut Solver) -> SolvedState {
    if self.is_grid_step() {
      if solver.candidates_active {
        assert_eq!(step.cells.len(), 1);
      }
      let cell = step.cells[0];
      let CellPosition { row, col } = cell;
      let value = step.values[0];

      if solver.step_count_limit.is_some() {
        // In this mode we can apply multiple steps at once and
        // we could find naked and hidden single for the same cell
        // So if we find a contradiction we should stop

        if solver.grid[row][col] != 0 && solver.grid[row][col] != value {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }
          )
        }
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

    SolvedState::solved()
  }

  fn apply_corresponding_indices(&self) -> bool { false }
}

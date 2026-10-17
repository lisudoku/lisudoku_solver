pub mod naked_singles;
pub mod hidden_singles;
pub mod thermo_steps;
pub mod candidates;
pub mod locked_candidates;
pub mod naked_set;
pub mod thermo_candidates;
pub mod hidden_set;
pub mod x_wing;
pub mod xy_wing;
pub mod common_peer_elimination;
pub mod sum_candidates;
pub mod killer_candidates;
pub mod killer45;
pub mod kropki_chain_candidates;
pub mod kropki_advanced_candidates;
pub mod common_peer_elimination_kropki;
pub mod turbot_fish;
pub mod top_bottom_candidates;
pub mod empty_reclanges;
pub mod combinations;
pub mod arrow_candidates;
pub mod advanced_candidates;
pub mod arrow_advanced_candidates;
pub mod common_peer_elimination_arrow;
// pub mod phistomefel_ring;
pub mod nishio_forcing_chains;
pub mod renban_candidates;
pub mod palindrome_values;
pub mod palindrome_candidates;
pub mod adhoc_naked_set;

use crate::{solver::{SolvedState, logical::logical_solver::LogicalSolver, types::CandidateSet}, types::{Area, CellPosition, InvalidStateReason, InvalidStateType, Rule, SolutionStep}};

pub trait Technique {
  // It is a grid step if it fills in a value in the grid
  fn is_grid_step(&self) -> bool { false }

  // All XCandidates type techniques should return true because they need to run first
  // These are the techniques that enforce constraints
  fn is_candidate_validity_update_step(&self) -> bool { false }

  fn get_rule(&self) -> Rule;

  fn complexity(&self, _step: &SolutionStep, _solver: &LogicalSolver) -> u32 { u32::MAX }

  fn build_solution_step(
    &self,
    cells: Vec<CellPosition>, values: Vec<u32>, areas: Vec<Area>, affected_cells: Vec<CellPosition>
  ) -> SolutionStep {
    SolutionStep::new(self.get_rule(), cells, values, areas, affected_cells)
  }

  fn build_grid_solution_step(
    &self, cells: Vec<CellPosition>, values: Vec<u32>, areas: Vec<Area>, solver: &LogicalSolver,
  ) -> SolutionStep {
    assert!(self.is_grid_step());
    let affected_cells = if solver.state.candidates_active {
      assert_eq!(cells.len(), 1);
      let cell = cells[0];
      let value = values[0];
      let values_set = &CandidateSet::from([value]);
      solver.state.cell_peers_with_candidates(&cell, values_set)
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

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep>;

  // This is the default base implementation, but can be overridden
  // Return value indicates whether applying this step leads to a contradiction
  // (overriding cell value, things that check_partially_solved wouldn't catch)
  fn apply(&self, step: &SolutionStep, solver: &mut LogicalSolver) -> SolvedState {
    if self.is_grid_step() {
      if solver.state.candidates_active {
        assert_eq!(step.cells.len(), 1);
      }
      let cell = step.cells[0];
      let CellPosition { row, col } = cell;
      let value = step.values[0];

      if solver.step_count_limit.is_some() {
        // In this mode we can apply multiple steps at once and
        // we could find naked and hidden single for the same cell
        // So if we find a contradiction we should stop

        if solver.state.grid[row][col] != 0 && solver.state.grid[row][col] != value {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }
          )
        }
      } else {
        assert_eq!(solver.state.grid[row][col], 0, "Attempted to overwrite cell");
      }

      if solver.state.grid[row][col] == 0 {
        solver.state.grid[row][col] = value;

        if solver.state.candidates_active {
          solver.state.candidates[row][col].clear();
          solver.update_candidates(&step.affected_cells, value);
        }
      }
    } else if self.apply_corresponding_indices() {
      for (index, cell) in step.affected_cells.iter().enumerate() {
        let value = step.values[index];
        solver.state.candidates[cell.row][cell.col].remove(&value);
      }
    } else {
      for &CellPosition { row, col } in &step.affected_cells {
        for value in &step.values {
          solver.state.candidates[row][col].remove(value);
        }
      }
    }

    SolvedState::solved()
  }

  fn apply_corresponding_indices(&self) -> bool { false }
}

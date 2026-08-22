use crate::solver::Solver;
use crate::solver::types::CandidateSet;
use crate::types::{Area, CellPosition, Grid, Rule, SolutionStep};
use super::technique::Technique;

pub struct TopBottomCandidates {
  // True if we only want to check the validity of the constraint
  checker_mode: bool,
}

impl Technique for TopBottomCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::TopBottomCandidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    // Do not run in "technique" mode when not candidates active because
    // we can't apply it to remove candidates
    if (!solver.candidates_active && !self.checker_mode) || solver.puzzle.top_bottom().is_none() {
      return vec![]
    }

    let candidates = solver.compute_all_cell_candidates();

    self.run_with_data(&solver.grid, &candidates)
  }
}

impl TopBottomCandidates {
  pub fn new(checker_mode: bool) -> TopBottomCandidates {
    TopBottomCandidates { checker_mode }
  }

  pub fn run_with_data(&self, grid: &Grid, candidates: &Vec<Vec<CandidateSet>>) -> Vec<SolutionStep> {
    let steps = self.check_paths(true, grid, candidates);
    if !steps.is_empty() {
      return steps
    }

    let steps = self.check_paths(false, grid, candidates);
    if !steps.is_empty() {
      return steps
    }

    vec![]
  }

  fn check_paths(&self, ascending: bool, grid: &Grid, candidates: &Vec<Vec<CandidateSet>>) -> Vec<SolutionStep> {
    let rows: Vec<usize> = if ascending {
      (0..grid.len()).into_iter().collect()
    } else {
      (0..grid.len()).rev().into_iter().collect()
    };

    let mut next_cells = self.get_row_cells_with_value(rows[0], 1, grid, candidates);
    for (index, pair) in rows.windows(2).enumerate() {
      let row = pair[0];
      let next_row = pair[1];
      let value = index as u32 + 1;
      let next_value = value + 1;
      let crt_cells = next_cells;
      next_cells = self.get_row_cells_with_value(next_row, next_value, grid, candidates);

      let extra_cells = self.get_extra_cells(&crt_cells, &next_cells);
      // In "checker" mode only fail if all cells are extra (all row candidates would get removed)
      if !extra_cells.is_empty() && (!self.checker_mode || extra_cells.len() == crt_cells.len()) {
        return self.map_extra_cells_to_step(extra_cells, row, value, next_row)
      }

      let extra_cells = self.get_extra_cells(&next_cells, &crt_cells);
      if !extra_cells.is_empty() && (!self.checker_mode || extra_cells.len() == next_cells.len()) {
        return self.map_extra_cells_to_step(extra_cells, next_row, next_value, row)
      }
    }

    vec![]
  }

  fn get_row_cells_with_value(&self, row: usize, value: u32, grid: &Grid, candidates: &Vec<Vec<CandidateSet>>) -> Vec<CellPosition> {
    (0..grid.len()).filter_map(|col| {
      if grid[row][col] == value || candidates[row][col].contains(&value) {
        return Some(CellPosition::new(row, col))
      }
      None
    }).collect()
  }

  /// Returns cells from cells_1 that do not have a neighbour in cells_2
  fn get_extra_cells(&self, cells_1: &Vec<CellPosition>, cells_2: &Vec<CellPosition>) -> Vec<CellPosition> {
    cells_1.into_iter().filter(|cell| {
      !cells_2.iter().any(|other_cell| other_cell.col.abs_diff(cell.col) <= 1)
    }).copied().collect()
  }

  fn map_extra_cells_to_step(&self, extra_cells: Vec<CellPosition>, row: usize, value: u32, other_row: usize) -> Vec<SolutionStep> {
    vec![
      self.build_simple_solution_step(
        vec![ value ],
        vec![ Area::Row(row), Area::Row(other_row) ],
        extra_cells,
      )
    ]
  }
}

use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, CellPosition, Area};
use super::technique::Technique;

pub struct TopBottomCandidates {
  checker_validity: bool,
}

impl Technique for TopBottomCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::TopBottomCandidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if (!solver.candidates_active && !self.checker_validity) || !solver.constraints.top_bottom {
      return vec![]
    }

    let steps = self.check_paths(true, solver);
    if !steps.is_empty() {
      return steps
    }

    let steps = self.check_paths(false, solver);
    if !steps.is_empty() {
      return steps
    }

    vec![]
  }
}

impl TopBottomCandidates {
  pub fn new(checker_validity: bool) -> TopBottomCandidates {
    TopBottomCandidates { checker_validity }
  }

  fn check_paths(&self, ascending: bool, solver: &Solver) -> Vec<SolutionStep> {
    let rows: Vec<usize> = if ascending {
      (0..solver.constraints.grid_size).into_iter().collect()
    } else {
      (0..solver.constraints.grid_size).rev().into_iter().collect()
    };

    let mut next_cells = self.get_row_cells_with_value(rows[0], 1, solver);
    for (index, pair) in rows.windows(2).enumerate() {
      let row = pair[0];
      let next_row = pair[1];
      let value = index as u32 + 1;
      let next_value = value + 1;
      let crt_cells = next_cells;
      next_cells = self.get_row_cells_with_value(next_row, next_value, solver);

      let extra_cells = self.get_extra_cells(&crt_cells, &next_cells);
      if !extra_cells.is_empty() && !(self.checker_validity && extra_cells.len() < crt_cells.len()) {
        return self.map_extra_cells_to_step(extra_cells, row, value, next_row)
      }

      let extra_cells = self.get_extra_cells(&next_cells, &crt_cells);
      if !extra_cells.is_empty() && !(self.checker_validity && extra_cells.len() < next_cells.len()) {
        return self.map_extra_cells_to_step(extra_cells, next_row, next_value, row)
      }
    }

    vec![]
  }

  fn get_row_cells_with_value(&self, row: usize, value: u32, solver: &Solver) -> Vec<CellPosition> {
    (0..solver.constraints.grid_size).filter_map(|col| {
      if solver.grid[row][col] == value ||
         solver.compute_cell_candidates(&CellPosition::new(row, col)).contains(&value) {
        return Some(CellPosition::new(row, col))
      }
      None
    }).collect()
  }

  // Returns cells from cells1 that do not have a neighbour in cells2
  fn get_extra_cells(&self, cells1: &Vec<CellPosition>, cells2: &Vec<CellPosition>) -> Vec<CellPosition> {
    cells1.into_iter().filter(|cell| {
      !cells2.iter().any(|other_cell| other_cell.col.abs_diff(cell.col) <= 1)
    }).copied().collect()
  }

  fn map_extra_cells_to_step(&self, extra_cells: Vec<CellPosition>, row: usize, value: u32, other_row: usize) -> Vec<SolutionStep> {
    vec![
      SolutionStep {
        rule: self.get_rule(),
        cells: vec![],
        values: vec![ value ],
        areas: vec![ Area::Row(row), Area::Row(other_row) ],
        affected_cells: extra_cells,
        candidates: None,
      }
    ]
  }
}

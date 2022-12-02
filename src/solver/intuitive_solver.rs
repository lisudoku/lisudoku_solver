use crate::types::{SudokuIntuitiveSolveResult, CellPosition, SolutionStep, Rule};
use crate::solver::Solver;
use itertools::Itertools;

mod naked_singles;
mod hidden_singles;
mod thermo_steps;
mod candidates;
mod locked_candidates;

impl Solver {
  pub fn intuitive_solve(&mut self) -> SudokuIntuitiveSolveResult {
    let mut empty_cell_count = self.compute_empty_cell_count();

    println!("Empty cell count: {}", empty_cell_count);

    let mut full_solution = true;
    let mut no_solution = false;
    let mut steps: Vec<SolutionStep> = vec![];
    while empty_cell_count > 0 {
      if self.is_cell_with_no_candidates() {
        full_solution = false;
        no_solution = true;
        break;
      }

      let step = self.find_next_step();
      if step.is_none() {
        full_solution = false;
        break
      }

      let mut step = step.unwrap();

      self.apply_rule(&mut step);

      if ![ Rule::Candidates, Rule::LockedCandidates, Rule::NakedPairs ].contains(&step.rule) {
        empty_cell_count -= 1;
      }

      steps.push(step);
    }

    let res = SudokuIntuitiveSolveResult {
      full_solution,
      no_solution,
      solution: self.grid.to_vec(),
      steps,
    };
    res
  }

  fn find_next_step(&self) -> Option<SolutionStep> {
    let step = self.find_grid_step();
    if step.is_some() {
      return step
    }

    let step = self.find_nongrid_step();
    if step.is_some() {
      return step
    }

    None
  }

  pub fn find_grid_step(&self) -> Option<SolutionStep> {
    let mut step = self.find_naked_singles();

    if step.is_none() {
      step = self.find_hidden_singles();
    }

    if step.is_none() {
      step = self.find_thermo_steps();
    }

    if let Some(mut_step) = &mut step {
      if self.candidates_active {
        let CellPosition { row, col } = mut_step.cells[0];
        let value = mut_step.values[0];
        mut_step.affected_cells = self.get_affected_cells(row, col, value);
      }
      return step
    }

    None
  }

  fn find_nongrid_step(&self) -> Option<SolutionStep> {
    let step = self.find_candidates_step();
    if step.is_some() {
      return step
    }

    let step = self.find_locked_candidates();
    if step.is_some() {
      return step
    }

    // let step = self.find_naked_pairs();
    // if step.is_some() {
    //   return step
    // }

    // TODO: implement other rules

    None
  }

  pub fn apply_rule(&mut self, step: &mut SolutionStep) {
    match &step.rule {
      Rule::Candidates => {
        self.candidates_active = true;
        self.candidates = step.candidates.as_ref().unwrap().to_vec();
        println!("{:?}", step.rule);
      }
      Rule::LockedCandidates => {
        let value = step.values[0];
        let cell1 = step.cells[0];
        let cell2 = step.cells[1];
        print!("{:?} ({},{}) ({},{}) {}: ", step.rule, cell1.row, cell1.col, cell2.row, cell2.col, value);
        for CellPosition { row, col } in step.affected_cells.iter().cloned() {
          self.candidates[row][col].remove(&value);
          print!("({},{}) ", row, col);
        }
        println!();
      }
      _ => {
        let CellPosition { row, col } = step.cells[0];
        let value = step.values[0];

        self.grid[row][col] = value;

        if self.candidates_active {
          self.update_candidates(&step.affected_cells, value);
        }

        println!("{} {} {} {:?}", row, col, value, step.rule);
      }
    }
  }

  fn is_cell_with_no_candidates(&self) -> bool {
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if self.grid[row][col] == 0 {
          let cell_candidates = self.compute_cell_candidates(row, col);
          if cell_candidates.is_empty() {
            return true
          }
        }
      }
    }

    false
  }

  fn get_affected_cells(&self, row: usize, col: usize, value: u32) -> Vec<CellPosition> {
    self.get_cell_areas(row, col, true)
        .iter()
        .flat_map(|area| self.get_area_cells_with_candidate(area, value))
        .filter(|cell| (cell.row != row || cell.col != col) &&
                       self.grid[cell.row][cell.col] == 0)
        .unique()
        .collect()
  }

  fn update_candidates(&mut self, cells: &Vec<CellPosition>, _value: u32) {
    for cell in cells {
      // TODO: can be improved by not recomputing and just removing <value>
      self.candidates[cell.row][cell.col] = self.recompute_cell_candidates(cell.row, cell.col);
    }
  }
  
  fn compute_empty_cell_count(&self) -> usize {
    self.grid
        .iter()
        .map(|row| row.iter()
                      .map(|cell| if *cell == 0 { 1 } else { 0 })
                      .sum::<usize>())
        .sum()
  }
}

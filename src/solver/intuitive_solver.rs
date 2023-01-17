use std::collections::HashSet;
use crate::types::{SudokuIntuitiveSolveResult, CellPosition, SolutionStep, Rule, Area, SolutionType};
use crate::solver::Solver;
use itertools::Itertools;

mod naked_singles;
mod hidden_singles;
mod thermo_steps;
mod candidates;
mod locked_candidates;
mod naked_set;
mod thermo_candidates;
mod hidden_set;
mod x_wing;
mod xy_wing;
mod common_peer_elimination;
mod sum_candidates;
mod killer_candidates;
mod killer45;
mod kropki_chain_candidates;
mod common_peer_elimination_kropki;

const DEBUG: bool = false;

impl Solver {
  pub fn intuitive_solve(&mut self) -> SudokuIntuitiveSolveResult {
    let mut solution_type = SolutionType::Full;
    let mut solution_steps: Vec<SolutionStep> = vec![];

    if !self.check_partially_solved() {
      println!("Invalid initial grid");
      return SudokuIntuitiveSolveResult::no_solution()
    }

    let mut empty_cell_count = self.compute_empty_cell_count();
    println!("Empty cell count: {}", empty_cell_count);

    while empty_cell_count > 0 {
      if let Some(cell) = self.get_cell_with_no_candidates() {
        println!("Cell with no candidates {:?}", cell);
        return SudokuIntuitiveSolveResult::no_solution()
      }

      // TODO: only check cells impacted by latest change
      if !self.check_partially_solved() {
        println!("Reached invalid state");
        return SudokuIntuitiveSolveResult::no_solution()
      }

      // Some rules can find multiple steps at once
      let steps = self.find_next_steps();
      if steps.is_empty() {
        solution_type = SolutionType::Partial;
        break
      }

      for mut step in steps.into_iter() {
        self.apply_rule(&mut step);

        if [ Rule::NakedSingle, Rule::HiddenSingle, Rule::Thermo ].contains(&step.rule) {
          empty_cell_count -= 1;
        }

        solution_steps.push(step);
      }
    }

    let res = SudokuIntuitiveSolveResult {
      solution_type,
      solution: Some(self.grid.to_vec()),
      steps: solution_steps,
    };

    res
  }

  fn find_next_steps(&self) -> Vec<SolutionStep> {
    // This type of rule must be 1st to make sure all candidates are valid
    let steps = self.find_thermo_candidate_updates();
    if !steps.is_empty() {
      return steps
    }
    let steps = self.find_killer_candidate_updates();
    if !steps.is_empty() {
      return steps
    }
    let steps = self.find_kropki_chain_candidate_updates();
    if !steps.is_empty() {
      return steps
    }
    let steps = self.find_kropki_pair_candidate_updates();
    if !steps.is_empty() {
      return steps
    }

    let steps = self.find_grid_steps();
    if !steps.is_empty() {
      return steps
    }

    let steps = self.find_nongrid_steps();
    if !steps.is_empty() {
      return steps
    }

    vec![]
  }

  pub fn find_grid_steps(&self) -> Vec<SolutionStep> {
    let mut step = self.find_naked_singles();

    if step.is_none() {
      step = self.find_hidden_singles();
    }

    if step.is_none() {
      step = self.find_thermo_steps();
    }

    if let Some(mut_step) = &mut step {
      if self.candidates_active {
        let cell = mut_step.cells[0];
        let value = mut_step.values[0];
        let values_set = &HashSet::from([value]);
        mut_step.affected_cells = self.get_affected_by_cell(&cell, values_set);
      }
      return vec![ step.unwrap() ]
    }

    vec![]
  }

  fn find_nongrid_steps(&self) -> Vec<SolutionStep> {
    let step = self.find_candidates_step();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    // Killer
    let steps = self.find_killer45();
    if !steps.is_empty() {
      return steps
    }

    // Pairs

    let step = self.find_locked_candidates_pairs();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let step = self.find_naked_pairs();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let step = self.find_hidden_pairs();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    // Triples

    let step = self.find_locked_candidates_triples();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let step = self.find_naked_triples();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let step = self.find_hidden_triples();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    // Other

    let step = self.find_x_wing();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let step = self.find_xy_wing();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let step = self.find_common_peer_elimination();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let step = self.find_common_peer_elimination_kropki();
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    // TODO: implement other rules

    vec![]
  }

  pub fn apply_rule(&mut self, step: &SolutionStep) {
    println!(
      "{:?} ({}) ({}) ({}): {}",
      step.rule,
      step.areas.iter().map(|x| format!("{:?}", x)).join(", "),
      step.cells.iter().map(|x| format!("({},{})", x.row, x.col)).join(" "),
      step.values.iter().map(|x| format!("{}", x)).join(", "),
      step.affected_cells.iter().map(|x| format!("({},{})", x.row, x.col)).join(" ")
    );
    match &step.rule {
      Rule::NakedSingle | Rule::HiddenSingle | Rule::Thermo => {
        let CellPosition { row, col } = step.cells[0];
        let value = step.values[0];

        self.grid[row][col] = value;

        if self.candidates_active {
          self.candidates[row][col].clear();
          self.update_candidates(&step.affected_cells, value);
        }
      }
      Rule::Candidates => {
        self.candidates_active = true;
        self.candidates = step.candidates.as_ref().unwrap().to_vec();
      }
      Rule::HiddenPairs | Rule::HiddenTriples => {
        for &CellPosition { row, col } in &step.cells {
          let value_set: HashSet<u32> = step.values.iter().copied().collect();
          self.candidates[row][col] = self.candidates[row][col].intersection(&value_set).copied().collect();
        }
      }
      Rule::XYWing => {
        for &CellPosition { row, col } in &step.affected_cells {
          // Remove Z as candidate
          self.candidates[row][col].remove(&step.values[2]);
        }
      }
      Rule::CommonPeerEliminationKropki => {
        for (index, cell) in step.affected_cells.iter().enumerate() {
          let value = step.values[index];
          self.candidates[cell.row][cell.col].remove(&value);
        }
      }
      _ => {
        for &CellPosition { row, col } in &step.affected_cells {
          for value in &step.values {
            self.candidates[row][col].remove(value);
          }
        }
      }
    }

    if DEBUG {
      // only after cell changes
      self.validate_candidates();
    }
  }

  fn get_cell_with_no_candidates(&self) -> Option<CellPosition> {
    for cell in self.get_all_empty_cells() {
      let cell_candidates = self.compute_cell_candidates(&cell);
      if cell_candidates.is_empty() {
        return Some(cell)
      }
    }

    None
  }

  // This method is used after placing a digit into the grid
  // Returns cells that <cell> sees which have any of <values> candidates
  fn get_affected_by_cell(&self, cell: &CellPosition, values: &HashSet<u32>) -> Vec<CellPosition> {
    self.get_cell_peers_with_candidates(cell, values)
  }

  // Returns cells that are seen by all <cells> with any of <values> candidates
  fn get_affected_by_cells(&self, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> Vec<CellPosition> {
    self.get_affected_by_cell(&cells[0], values)
        .into_iter()
        .filter(|cell| {
          cells[1..].iter().all(|other_cell| {
            self.cells_affect_eachother(cell, other_cell)
          })
        })
        .collect()
  }

  fn cells_affect_eachother(&self, cell1: &CellPosition, cell2: &CellPosition) -> bool {
    cell1 != cell2 &&
    !self.get_cell_areas(cell1, false)
         .into_iter()
         .collect::<HashSet<Area>>()
         .is_disjoint(
           &self.get_cell_areas(cell2, false)
                .into_iter()
                .collect()
         )
  }

  // Returns cells in <area> except <cells> that have any of <values> candidates
  fn get_affected_by_area_cells(&self, area: &Area, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> Vec<CellPosition> {
    self.get_area_cells_with_candidates(area, values)
        .into_iter()
        .filter(|cell| !cells.contains(cell))
        .collect()
  }

  fn update_candidates(&mut self, cells: &Vec<CellPosition>, value: u32) {
    for cell in cells {
      self.candidates[cell.row][cell.col].remove(&value);
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

  fn find_common_areas_except(&self, cells: &Vec<CellPosition>, area_exception: Area) -> Vec<Area> {
    let areas = self.find_common_areas(cells);
    let other_areas: Vec<Area> = areas.into_iter().filter(|&area| area != area_exception).collect();
    other_areas
  }

  // Note: update when adding new areas
  fn find_common_areas(&self, cells: &Vec<CellPosition>) -> Vec<Area> {
    assert!(cells.len() >= 2);

    let cell1 = cells[0];

    let mut areas = vec![];
    if cells.iter().map(|cell| cell.row).all_equal() {
      areas.push(Area::Row(cell1.row));
    }
    if cells.iter().map(|cell| cell.col).all_equal() {
      areas.push(Area::Column(cell1.col));
    }
    if cells.iter().map(|cell| self.grid_to_region[cell.row][cell.col]).all_equal() {
      let region_index = self.grid_to_region[cell1.row][cell1.col];
      areas.push(Area::Region(region_index));
    }
    if cells.iter().map(|cell| self.grid_to_killer_cage[cell.row][cell.col]).all_equal() {
      let killer_cage_index = self.grid_to_killer_cage[cell1.row][cell1.col];
      if killer_cage_index != usize::MAX {
        areas.push(Area::KillerCage(killer_cage_index));
      }
    }
    if self.constraints.primary_diagonal && cells.iter().all(|cell| cell.row == cell.col) {
      areas.push(Area::PrimaryDiagonal);
    }
    if self.constraints.secondary_diagonal && cells.iter().all(|cell| cell.row + cell.col == self.constraints.grid_size - 1) {
      areas.push(Area::SecondaryDiagonal);
    }

    areas
  }

  fn any_cells_with_candidates(&self, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> bool {
    cells.iter().any(|cell| !self.candidates[cell.row][cell.col].is_disjoint(&values))
  }

  fn any_cells_with_other_candidates(&self, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> bool {
    cells.iter().any(|cell| self.candidates[cell.row][cell.col].difference(&values).count() > 0)
  }

  // Move <cell> orthogonally to <area>
  fn cell_to_area(&self, cell: &CellPosition, area: &Area) -> CellPosition {
    match area {
      &Area::Row(row) => CellPosition { row, col: cell.col },
      &Area::Column(col) => CellPosition { row: cell.row, col },
      _ => unimplemented!(),
    }
  }

  fn validate_candidates(&self) {
    if !self.candidates_active {
      return
    }
    for cell in &self.get_all_empty_cells() {
      let &CellPosition { row, col } = cell;
      let cell_candidates = self.recompute_cell_candidates(cell);
      if self.candidates[row][col] != cell_candidates {
        println!("==> Invalid candidates for ({},{})!", row, col);
        println!("Saved candidates: {:?}", self.candidates[row][col]);
        println!("Real candidates: {:?}", cell_candidates);
        return
      }
    }
  }

  pub fn cell_candidates_diff(&self, cells: &Vec<CellPosition>, valid_candidates: Vec<HashSet<u32>>) -> Vec<(CellPosition, Vec<u32>)> {
    cells.into_iter().enumerate().filter_map(|(cell_index, &cell)| {
      let cell_candidates = &self.candidates[cell.row][cell.col];
      let valid_cell_candidates = &valid_candidates[cell_index];
      if cell_candidates.len() == valid_cell_candidates.len() {
        return None
      }

      let invalid_values: Vec<u32> = cell_candidates.difference(valid_cell_candidates)
                                                    .into_iter()
                                                    .copied()
                                                    .collect();

      if invalid_values.is_empty() {
        return None
      }

      Some((cell, invalid_values))
    }).collect()
  }
}

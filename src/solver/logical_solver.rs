use std::collections::{HashSet, HashMap};
use std::ops::BitOr;
use std::rc::Rc;
use crate::types::{Area, CellPosition, InvalidStateReason, SolutionStep, SolutionType, SudokulogicalSolveResult};
use crate::solver::Solver;
use self::combinations::cell_combination_logic::CellsCacheKey;
use self::technique::Technique;
use itertools::Itertools;

pub mod technique;
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
pub mod phistomefel_ring;
pub mod nishio_forcing_chains;
pub mod renban_candidates;
pub mod palindrome_values;
pub mod palindrome_candidates;
pub mod adhoc_naked_set;

const DEBUG: bool = false;
const DISPLAY_STEPS: bool = false;

impl Solver {
  pub fn logical_solve(&mut self) -> SudokulogicalSolveResult {
    let mut solution_type = SolutionType::Full;
    let mut solution_steps = vec![];
    let mut solution_steps_group_count = 0;

    let check = self.check_partially_solved();
    if !check.0 {
      if DEBUG {
        println!("Invalid initial grid");
      }
      return SudokulogicalSolveResult::no_solution(check.1.unwrap())
    }

    let mut empty_cell_count = self.compute_empty_cell_count();

    while empty_cell_count > 0 {
      // TODO: only check cells impacted by latest change
      let check = self.check_partially_solved();
      if !check.0 {
        if DEBUG {
          println!("Reached invalid state");
        }
        return SudokulogicalSolveResult::no_solution(check.1.unwrap())
      }

      // Some rules can find multiple steps at once
      let mut steps = self.find_next_steps();
      if steps.is_empty() {
        break
      }

      solution_steps_group_count += 1;

      if self.hint_mode {
        // In hint mode apply 1 step at a time
        steps.drain(1..);
      }
      if let Some(limit) = self.step_count_limit {
        if solution_steps_group_count >= limit {
          solution_steps.extend(steps);
          // Found all steps from initial grid, stop
          break
        }
      }

      let mut grid_step = false;
      for mut step in steps.into_iter() {
        let rule_check = self.apply_rule(&mut step);
        if !rule_check.0 {
          return SudokulogicalSolveResult::no_solution(rule_check.1.unwrap())
        }

        if step.is_grid_step() {
          grid_step = true;
        }

        solution_steps.push(step);
      }

      if self.hint_mode && grid_step {
        // Found the first filled digit, it's enough for a hint
        break
      }

      empty_cell_count = self.compute_empty_cell_count();
    }

    if empty_cell_count > 0 {
      solution_type = SolutionType::Partial;
    }

    let res = SudokulogicalSolveResult {
      solution_type,
      solution: Some(self.grid.to_vec()),
      steps: solution_steps.clone(),
      invalid_state_reason: None,
    };

    res
  }

  fn find_next_steps(&self) -> Vec<SolutionStep> {
    if self.hint_mode {
      // In this context we already know that there is a valid solution
      let steps = self.find_grid_steps();
      if !steps.is_empty() {
        return steps
      }
    }

    // This type of rule must be 1st to make sure all candidates are valid
    // before applying other techniques
    let validity_update_steps = self.find_candidate_validity_update_steps();
    // TODO: might want to tweak single_step_mode for validity_update_steps and nongrid_steps
    if !validity_update_steps.is_empty() && self.step_count_limit.is_none() {
      return validity_update_steps
    }

    let grid_steps = self.find_grid_steps();
    if !grid_steps.is_empty() && self.step_count_limit.is_none() {
      return grid_steps
    }

    let nongrid_steps = self.find_nongrid_steps();
    if !nongrid_steps.is_empty() && self.step_count_limit.is_none() {
      return nongrid_steps
    }

    vec![ validity_update_steps, grid_steps, nongrid_steps ].concat()
  }

  pub fn find_candidate_validity_update_steps(&self) -> Vec<SolutionStep> {
    let candidate_validity_techniques: Vec<&Rc<dyn Technique>> = self.techniques
      .iter()
      .filter(|technique| technique.is_candidate_validity_update_step())
      .collect();

    let steps = self.run_techniques(candidate_validity_techniques);

    steps
  }

  pub fn find_grid_steps(&self) -> Vec<SolutionStep> {
    let grid_techniques: Vec<&Rc<dyn Technique>> = self.techniques
      .iter()
      .filter(|technique| technique.is_grid_step())
      .collect();

    self.run_techniques(grid_techniques)
  }

  fn find_nongrid_steps(&self) -> Vec<SolutionStep> {
    let nongrid_techniques: Vec<&Rc<dyn Technique>> = self.techniques
      .iter()
      .filter(|technique| !technique.is_grid_step() &&
                          !technique.is_candidate_validity_update_step())
      .collect();

    let steps = self.run_techniques(nongrid_techniques);

    steps
  }

  fn run_techniques(&self, techniques: Vec<&Rc<dyn Technique>>) -> Vec<SolutionStep> {
    techniques.into_iter().fold(vec![], |mut all_steps, technique| {
      if !all_steps.is_empty() && self.step_count_limit.is_none() {
        return all_steps
      }
      let steps = technique.run(&self);
      all_steps.extend(steps);

      all_steps
    })
  }

  pub fn apply_rule(&mut self, step: &SolutionStep) -> (bool, Option<InvalidStateReason>) {
    if DISPLAY_STEPS {
      println!(
        "{:?} ({}) ({}) ({}): {}",
        step.rule,
        step.areas.iter().map(|x| format!("{:?}", x)).join(", "),
        step.cells.iter().map(|x| format!("({},{})", x.row, x.col)).join(" "),
        step.values.iter().map(|x| format!("{}", x)).join(", "),
        step.affected_cells.iter().map(|x| format!("({},{})", x.row, x.col)).join(" ")
      );
      if let Some(reason) = &step.invalid_state_reason {
        println!("{:?}", reason);
      }
    }

    let technique = self.techniques
      .iter()
      .find(|technique| technique.get_rule() == step.rule)
      .cloned()
      .unwrap_or_else(|| panic!("Can't find technique for rule {}", step.rule));

    let rule_check = technique.apply(&step, self);

    if DEBUG {
      // only after cell changes
      self.validate_candidates();
    }

    rule_check
  }

  pub fn apply_rules(&mut self, steps: &Vec<SolutionStep>) {
    for mut step in steps {
      self.apply_rule(&mut step);
    }
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

  // Returns cells in area (given by <area_cells>) except <cells> that have any of <values> candidates
  fn get_affected_by_area_cells_cells(&self, area_cells: &Vec<CellPosition>, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> Vec<CellPosition> {
    self.filter_cells_with_any_candidates(area_cells, values)
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
    let other_areas: Vec<Area> = areas.into_iter().filter(|area| *area != area_exception).collect();
    other_areas
  }

  // Note: update when adding new areas
  // Returns all areas that contain all <cells>
  // Used to see which areas are affected by all <cells>
  // Note: we want areas that need all <grid_size> and unique values
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

    let mut common_regions: HashSet<&usize> = self.grid_to_regions[cells[0].row][cells[0].col].iter().collect();
    for cell in cells[1..].iter() {
      let cell_regions: HashSet<&usize> = self.grid_to_regions[cell.row][cell.col].iter().collect();
      common_regions = common_regions.intersection(&cell_regions).copied().collect();
    }
    for &region_index in common_regions.into_iter().sorted() {
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

    let mut common_renbans: HashSet<&usize> = self.grid_to_renbans[cells[0].row][cells[0].col].iter().collect();
    for cell in cells[1..].iter() {
      let cell_renbans: HashSet<&usize> = self.grid_to_renbans[cell.row][cell.col].iter().collect();
      common_renbans = common_renbans.intersection(&cell_renbans).copied().collect();
    }
    for &renban_index in common_renbans {
      areas.push(Area::Renban(renban_index));
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
      let recomputed_cell_candidates = self.recompute_cell_candidates(cell);
      if !self.candidates[row][col].is_subset(&recomputed_cell_candidates) {
        println!("==> Invalid candidates for ({},{})!", row, col);
        println!("Saved candidates: {:?}", self.candidates[row][col]);
        println!("Real candidates: {:?}", recomputed_cell_candidates);
        panic!();
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
                                                    .sorted()
                                                    .collect();

      if invalid_values.is_empty() {
        return None
      }

      Some((cell, invalid_values))
    }).collect()
  }

  fn get_all_strong_links(&self) -> Vec<(Area, u32, CellPosition, CellPosition)> {
    self.get_all_proper_areas().iter().flat_map(|area| {
      let value_cells = self.compute_cells_by_value_in_area(area, &self.candidates);

      value_cells.into_iter().sorted().filter_map(|(value, cells)| {
        if cells.len() != 2 {
          return None
        }
        return Some(
          (area.clone(), value, cells[0], cells[1])
        )
      })
    }).collect()
  }

  fn get_all_strong_links_by_value(&self) -> HashMap<u32, Vec<(Area, u32, CellPosition, CellPosition)>> {
    self.get_all_strong_links()
      .iter()
      .cloned()
      .sorted_by_key(|link| (link.1, link.0.clone(), link.2, link.3))
      .group_by(|link| link.1)
      .into_iter()
      .map(|(value, group)| (value, group.collect()))
      .collect()
  }

  fn candidates_to_set(&self, cell: CellPosition) -> u32 {
    self.candidates[cell.row][cell.col].iter().fold(0, |acc, e| {
      acc.bitor(1 << e)
    })
  }

  fn cells_to_cache_key(&self, cells: &Vec<CellPosition>) -> CellsCacheKey {
    cells.into_iter().map(|cell| {
      (
        cell.row as u32 * (self.constraints.grid_size as u32 + 1) + cell.col as u32,
        self.grid[cell.row][cell.col],
        self.candidates_to_set(*cell),
      )
    }).collect()
  }
}

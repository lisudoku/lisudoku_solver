use std::collections::HashMap;
use std::rc::Rc;
use std::usize;
use crate::solver::checker::SolvedState;
use crate::solver::types::CandidateSet;
use crate::types::{Area, CellPosition, SolutionStep, SolutionType, SudokuLogicalSolveResult};
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
// pub mod phistomefel_ring;
pub mod nishio_forcing_chains;
pub mod renban_candidates;
pub mod palindrome_values;
pub mod palindrome_candidates;
pub mod adhoc_naked_set;

const DEBUG: bool = false;
const DISPLAY_STEPS: bool = false;

impl Solver {
  pub fn logical_solve(&mut self) -> SudokuLogicalSolveResult {
    let mut solution_type = SolutionType::Full;
    let mut solution_steps = vec![];
    let mut solution_steps_group_count = 0;

    let mut empty_cell_count = self.compute_empty_cell_count();

    while empty_cell_count > 0 {
      // TODO: only check cells impacted by latest change

      // Note: this check is needed on each step because
      // apply_rule only checks some contradictions
      let check = self.check_partially_solved();
      if !check.solved {
        return SudokuLogicalSolveResult::no_solution(
          check.invalid_state_reason.unwrap(),
          solution_steps,
        )
      }

      // Note: some rules can find multiple steps at once
      let mut steps = self.find_next_steps();

      if steps.is_empty() {
        // No more steps found, stop
        break
      }

      solution_steps_group_count += 1;

      if self.hint_mode {
        // In hint mode apply 1 step at a time
        steps.drain(1..);
      }

      let mut found_grid_step = false;
      for mut step in steps.into_iter() {
        if step.is_grid_step() {
          found_grid_step = true;
        }

        let rule_check = self.apply_rule(&mut step);

        if self.enriched_steps_enabled {
          self.enrich_solution_step(&mut step);
        }

        solution_steps.push(step);

        if !rule_check.solved {
          return SudokuLogicalSolveResult::no_solution(
            rule_check.invalid_state_reason.unwrap(),
            solution_steps,
          )
        }
      }

      if solution_steps_group_count >= self.step_count_limit.unwrap_or(usize::MAX) {
        // Reached step count limit, stop
        break
      }

      if self.hint_mode && found_grid_step {
        // Found the first filled digit, it's enough for a hint
        break
      }

      empty_cell_count = self.compute_empty_cell_count();
    }

    // Check whether final state is valid
    let check = self.check_partially_solved();
    if !check.solved {
      return SudokuLogicalSolveResult::no_solution(
        check.invalid_state_reason.unwrap(),
        solution_steps,
      )
    }

    if empty_cell_count > 0 {
      solution_type = SolutionType::Partial;
    }

    let res = SudokuLogicalSolveResult {
      solution_type,
      solution: Some(self.grid.clone()),
      steps: solution_steps,
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

  fn find_candidate_validity_update_steps(&self) -> Vec<SolutionStep> {
    let candidate_validity_techniques: Vec<&Rc<dyn Technique>> = self.techniques
      .iter()
      .filter(|technique| technique.is_candidate_validity_update_step())
      .collect();

    let steps = self.run_nongrid_techniques(candidate_validity_techniques);

    steps
  }

  pub(crate) fn find_grid_steps(&self) -> Vec<SolutionStep> {
    let grid_techniques: Vec<&Rc<dyn Technique>> = self.techniques
      .iter()
      .filter(|technique| technique.is_grid_step())
      .collect();

    self.run_grid_techniques(grid_techniques)
  }

  fn find_nongrid_steps(&self) -> Vec<SolutionStep> {
    let nongrid_techniques: Vec<&Rc<dyn Technique>> = self.techniques
      .iter()
      .filter(|technique| !technique.is_grid_step() &&
                          !technique.is_candidate_validity_update_step())
      .collect();

    let steps = self.run_nongrid_techniques(nongrid_techniques);

    steps
  }

  fn run_grid_techniques(&self, techniques: Vec<&Rc<dyn Technique>>) -> Vec<SolutionStep> {
    let mut all_steps: Vec<SolutionStep> = vec![];

    for technique in &techniques {
      let steps = technique.run(&self);
      all_steps.extend(steps);
    }

    if !all_steps.is_empty() && self.step_count_limit.is_none() {
      // In this mode we only want to return one step so pick the simplest
      let best_step = all_steps.into_iter().min_by_key(|step| {
        techniques
          .iter()
          .find(|technique| technique.get_rule() == step.rule)
          .unwrap()
          .complexity(step, self)
      });

      return vec![best_step.unwrap()];
    }

    all_steps
  }

  fn run_nongrid_techniques(&self, techniques: Vec<&Rc<dyn Technique>>) -> Vec<SolutionStep> {
    let mut all_steps: Vec<SolutionStep> = vec![];

    for technique in techniques {
      let steps = technique.run(&self);

      // Usually we want to stop after finding any steps, except with step_count_limit
      if !steps.is_empty() && self.step_count_limit.is_none() {
        return steps
      }

      all_steps.extend(steps);
    }

    all_steps
  }

  /// Updates grid and candidates with the deduction from `step`
  pub(crate) fn apply_rule(&mut self, step: &SolutionStep) -> SolvedState {
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

  #[cfg(test)]
  pub(crate) fn apply_rules(&mut self, steps: &Vec<SolutionStep>) {
    for mut step in steps {
      self.apply_rule(&mut step);
    }
  }

  /// Returns cells that are seen by all <cells> with any of <values> candidates
  fn cells_peers_with_any_candidates(&self, cells: &Vec<CellPosition>, values: &CandidateSet) -> Vec<CellPosition> {
    self
      .cell_peers_with_candidates(&cells[0], values)
      .into_iter()
      .filter(|cell| {
        cells[1..].iter().all(|other_cell| {
          self.puzzle.cells_are_peers(cell, other_cell)
        })
      })
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
      .map(|row| {
        row
          .iter()
          .map(|cell| if *cell == 0 { 1 } else { 0 })
          .sum::<usize>()
      })
      .sum()
  }

  fn any_cells_with_candidates(&self, cells: &Vec<CellPosition>, values: &CandidateSet) -> bool {
    cells.iter().any(|cell| !self.candidates[cell.row][cell.col].is_disjoint(&values))
  }

  fn any_cells_with_other_candidates(&self, cells: &Vec<CellPosition>, values: &CandidateSet) -> bool {
    cells.iter().any(|cell| self.candidates[cell.row][cell.col].difference(&values).len() > 0)
  }

  fn validate_candidates(&self) {
    if !self.candidates_active {
      return
    }
    for cell in &self.all_empty_cells() {
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

  fn cell_candidates_diff(&self, cells: &Vec<CellPosition>, valid_candidates: Vec<CandidateSet>) -> Vec<(CellPosition, Vec<u32>)> {
    cells.into_iter().enumerate().filter_map(|(cell_index, &cell)| {
      let cell_candidates = &self.candidates[cell.row][cell.col];
      let valid_cell_candidates = &valid_candidates[cell_index];
      if cell_candidates.len() == valid_cell_candidates.len() {
        return None
      }

      let invalid_values: Vec<u32> = cell_candidates
        .difference(valid_cell_candidates)
        .into_iter()
        .sorted()
        .collect();

      if invalid_values.is_empty() {
        return None
      }

      Some((cell, invalid_values))
    }).collect()
  }

  fn all_strong_links(&self) -> Vec<(Area, u32, CellPosition, CellPosition)> {
    self.puzzle.all_house_areas().iter().flat_map(|area| {
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

  fn all_strong_links_by_value(&self) -> HashMap<u32, Vec<(Area, u32, CellPosition, CellPosition)>> {
    self.all_strong_links()
      .iter()
      .cloned()
      .sorted_by_key(|link| (link.1, link.0.clone(), link.2, link.3))
      .group_by(|link| link.1)
      .into_iter()
      .map(|(value, group)| (value, group.collect()))
      .collect()
  }

  fn candidates_to_set(&self, cell: CellPosition) -> u32 {
    self.candidates[cell.row][cell.col].to_bitmask()
  }

  fn cells_to_cache_key(&self, cells: &Vec<CellPosition>) -> CellsCacheKey {
    cells.into_iter().map(|cell| {
      (
        cell.row as u32 * (self.puzzle.grid_size as u32 + 1) + cell.col as u32,
        self.grid[cell.row][cell.col],
        self.candidates_to_set(*cell),
      )
    }).collect()
  }

  fn enrich_solution_step(&self, step: &mut SolutionStep) {
    step.grid = Some(self.grid.clone());
    if self.candidates_active {
      let candidates: Vec<Vec<Vec<u32>>> = self.candidates.iter().map(|row|
        row.iter().map(|cell|
          cell.clone().into_iter().sorted().collect()
        ).collect()
      ).collect();
      step.candidates = Some(candidates);
    }
  }
}

use crate::{solver::{GridCheckResult, SolveState, logical::{caches::LogicalCaches, techniques::{Technique, TechniqueDefinition, TechniqueKind, TechniquePhase}}, types::CandidateSet}, types::{Area, CellPosition, SolutionStep, SolutionType, SudokuConstraints, SudokuLogicalSolveResult}};
use std::collections::HashMap;
use itertools::Itertools;
use strum::EnumCount;

const DEBUG: bool = false;
const DISPLAY_STEPS: bool = false;

pub struct LogicalSolver {
  pub(crate) state: SolveState,
  pub(crate) all_techniques_by_kind: [Technique; TechniqueKind::COUNT],
  pub(crate) enabled_techniques_by_phase: [Vec<Technique>; TechniquePhase::COUNT],
  pub(crate) caches: LogicalCaches,
  pub(crate) hint_mode: bool,
  pub(crate) step_count_limit: Option<usize>,
  pub(crate) enriched_steps_enabled: bool,
  /// Skips as many non-essential things as possible,
  /// used for constraints propagation inside brute-force solver
  pub(crate) performance_mode: bool,
}

impl LogicalSolver {
  pub fn new(input_constraints: SudokuConstraints) -> LogicalSolver {
    Self::from_solve_state(SolveState::new(input_constraints))
  }

  pub(crate) fn from_solve_state(state: SolveState) -> LogicalSolver {
    LogicalSolver{
      state: state,
      all_techniques_by_kind: Self::initialize_techniques_by_kind(Self::all_techniques()),
      enabled_techniques_by_phase: Self::group_techniques_by_phase(Self::all_techniques()),
      caches: LogicalCaches::new(),
      hint_mode: false,
      step_count_limit: None,
      enriched_steps_enabled: false,
      performance_mode: false,
    }
  }

  fn initialize_techniques_by_kind(techniques: impl Iterator<Item = Technique>) -> [Technique; TechniqueKind::COUNT] {
    let mut technique_slots: [Option<Technique>; TechniqueKind::COUNT] = Default::default();
    for technique in techniques {
      let kind_index = technique.kind() as usize;
      assert!(
        technique_slots[kind_index].replace(technique).is_none(),
        "duplicate index {kind_index}",
      );
    }
    technique_slots.map(|technique| technique.expect("missing index"))
  }

  pub(crate) fn group_techniques_by_phase(techniques: impl Iterator<Item = Technique>) -> [Vec<Technique>; TechniquePhase::COUNT] {
    let mut enabled_techniques_by_phase: [Vec<Technique>; TechniquePhase::COUNT] = Default::default();
    for technique in techniques {
      let phase_index = technique.phase() as usize;
      enabled_techniques_by_phase[phase_index].push(technique);
    }
    enabled_techniques_by_phase
  }

  pub fn solve(&mut self) -> SudokuLogicalSolveResult {
    let mut solution_type = SolutionType::Full;
    let mut solution_steps = vec![];
    let mut solution_steps_group_count = 0;
    let mut found_grid_step = false;

    while self.state.any_empty_cells() {
      // TODO: only check cells impacted by latest change

      if !self.performance_mode {
        // Note: this check is needed on each step because
        // apply_step only checks some contradictions
        let check = self.state.check_partially_solved();
        if !check.solved {
          return SudokuLogicalSolveResult::no_solution(
            check.invalid_state_reason.unwrap(),
            solution_steps,
          )
        }
      }

      // Note: some techniques can find multiple steps at once
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

      for mut step in steps.into_iter() {
        if step.is_grid_step() {
          found_grid_step = true;
        }

        let grid_check = self.apply_step(&mut step);

        if self.enriched_steps_enabled {
          self.enrich_solution_step(&mut step);
        }

        solution_steps.push(step);

        if !grid_check.solved {
          return SudokuLogicalSolveResult::no_solution(
            grid_check.invalid_state_reason.unwrap(),
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
    }

    // Check whether final state is valid
    if !self.performance_mode || (!solution_steps.is_empty() && found_grid_step) {
      let check = self.state.check_partially_solved();
      if !check.solved {
        return SudokuLogicalSolveResult::no_solution(
          check.invalid_state_reason.unwrap(),
          solution_steps,
        )
      }
    }

    if self.state.any_empty_cells() {
      solution_type = SolutionType::Partial;
    }

    let res = SudokuLogicalSolveResult {
      solution_type,
      solution: if self.performance_mode { None } else { Some(self.state.grid().clone()) },
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

    // This type of technique must be 1st to make sure all candidates are valid
    // before applying other techniques
    let validity_update_steps = self.find_candidate_validity_update_steps();
    // TODO: might want to tweak single_step_mode for validity_update_steps and candidate_elimination_steps
    if !validity_update_steps.is_empty() && self.step_count_limit.is_none() {
      return validity_update_steps
    }

    let grid_steps = self.find_grid_steps();
    if !grid_steps.is_empty() && self.step_count_limit.is_none() {
      return grid_steps
    }

    let candidate_elimination_steps = self.find_candidate_elimination_steps();
    if !candidate_elimination_steps.is_empty() && self.step_count_limit.is_none() {
      return candidate_elimination_steps
    }

    vec![ validity_update_steps, grid_steps, candidate_elimination_steps ].concat()
  }

  fn find_candidate_validity_update_steps(&self) -> Vec<SolutionStep> {
    let candidate_validity_techniques = &self.enabled_techniques_by_phase[TechniquePhase::CandidateValidity as usize];

    self.run_nongrid_techniques(candidate_validity_techniques)
  }

  pub(crate) fn find_grid_steps(&self) -> Vec<SolutionStep> {
    let grid_techniques = &self.enabled_techniques_by_phase[TechniquePhase::Grid as usize];

    self.run_grid_techniques(grid_techniques)
  }

  fn find_candidate_elimination_steps(&self) -> Vec<SolutionStep> {
    let candidate_elimination_techniques = &self.enabled_techniques_by_phase[TechniquePhase::CandidateElimination as usize];

    self.run_nongrid_techniques(&candidate_elimination_techniques)
  }

  fn run_grid_techniques(&self, techniques: &[Technique]) -> Vec<SolutionStep> {
    let mut all_steps: Vec<SolutionStep> = vec![];

    for technique in techniques {
      let steps = technique.run(&self);
      all_steps.extend(steps);
      if self.performance_mode && !all_steps.is_empty() {
        // In this case we don't care which is the simplest, return the first one found
        return all_steps[..1].to_vec()
      }
    }

    // Careful, returning multiple grid steps at the same time can cause bugs
    // because after applying them the latter ones might become invalid.
    // This can be prevented by checking validity after applying all of them, but
    // you pay the performance cost, so it depends on the context.
    if !all_steps.is_empty() && self.step_count_limit.is_none() {
      // In this mode we only want to return one step so pick the simplest
      let best_step = all_steps.into_iter().min_by_key(|step| {
        techniques
          .iter()
          .find(|technique| technique.kind() == step.technique_kind)
          .unwrap()
          .complexity(step, self)
      });

      return vec![best_step.unwrap()];
    }

    all_steps
  }

  fn run_nongrid_techniques(&self, techniques: &[Technique]) -> Vec<SolutionStep> {
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
  pub(crate) fn apply_step(&mut self, step: &SolutionStep) -> GridCheckResult {
    if DISPLAY_STEPS {
      println!(
        "{:?} ({}) ({}) ({}): {}",
        step.technique_kind,
        step.areas.iter().map(|x| format!("{:?}", x)).join(", "),
        step.cells.iter().map(|x| format!("({},{})", x.row, x.col)).join(" "),
        step.values.iter().map(|x| format!("{}", x)).join(", "),
        step.affected_cells.iter().map(|x| format!("({},{})", x.row, x.col)).join(" ")
      );
      if let Some(reason) = &step.invalid_state_reason {
        println!("{:?}", reason);
      }
    }

    let technique = &self.all_techniques_by_kind[step.technique_kind as usize];

    let grid_check = technique.apply(&step, &mut self.state, self.step_count_limit);

    if DEBUG {
      // only after cell changes
      self.validate_candidates();
    }

    grid_check
  }

  #[cfg(test)]
  pub(crate) fn apply_steps(&mut self, steps: &[SolutionStep]) {
    for mut step in steps {
      self.apply_step(&mut step);
    }
  }

  /// Returns cells that are seen by all `cells` with any of `values` candidates
  pub(crate) fn cells_peers_with_any_candidates(&self, cells: &[CellPosition], values: &CandidateSet) -> Vec<CellPosition> {
    self
      .state
      .cell_peers_with_candidates(&cells[0], values)
      .into_iter()
      .filter(|cell| {
        cells[1..].iter().all(|other_cell| {
          self.state.puzzle.cells_are_peers(cell, other_cell)
        })
      })
      .collect()
  }

  pub(crate) fn any_cells_with_candidates(&self, cells: &[CellPosition], values: &CandidateSet) -> bool {
    cells.iter().any(|cell| !self.state.candidates[cell.row][cell.col].is_disjoint(&values))
  }

  pub(crate) fn any_cells_with_other_candidates(&self, cells: &[CellPosition], values: &CandidateSet) -> bool {
    cells.iter().any(|cell| self.state.candidates[cell.row][cell.col].difference(&values).len() > 0)
  }

  fn validate_candidates(&self) {
    if !self.state.candidates_active {
      return
    }
    for cell in self.state.all_empty_cells() {
      let CellPosition { row, col } = cell;
      let recomputed_cell_candidates = self.state.recompute_cell_candidates(&cell);
      if !self.state.candidates[row][col].is_subset(&recomputed_cell_candidates) {
        println!("==> Invalid candidates for ({},{})!", row, col);
        println!("Saved candidates: {:?}", self.state.candidates[row][col]);
        println!("Real candidates: {:?}", recomputed_cell_candidates);
        panic!();
      }
    }
  }

  /// Returns the list of invalid candidates for each cell
  pub(crate) fn cell_candidates_diff(&self, cells: &[CellPosition], valid_candidates: &[CandidateSet]) -> Vec<(CellPosition, Vec<u32>)> {
    cells.iter().enumerate().filter_map(|(cell_index, &cell)| {
      if self.state.value_at(&cell) != 0 {
        return None
      }
      let cell_candidates = &self.state.candidates[cell.row][cell.col];
      let valid_cell_candidates = &valid_candidates[cell_index];
      assert!(
        valid_cell_candidates.is_subset(cell_candidates),
        "Cell {:?}: valid_candidates={:?} should be a subset of candidates={:?}",
        cell, valid_cell_candidates, cell_candidates,
      );
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

  pub(crate) fn deduplicate_eliminations<'a>(
    &'a self,
    invalid_candidates: Vec<(CellPosition, Vec<u32>)>,
    cell_previous_invalid_values: &'a mut Vec<CandidateSet>,
  ) -> impl Iterator<Item = (CellPosition, Vec<u32>)> + 'a {
    let grid_size = self.state.puzzle.grid_size;
    invalid_candidates.into_iter().filter_map(move |(cell, invalid_values)| {
      let cell_index = cell.to_index(grid_size);
      let new_invalid_values = CandidateSet::from_iter(invalid_values).difference(&cell_previous_invalid_values[cell_index]);
      if new_invalid_values.is_empty() {
        return None
      }
      cell_previous_invalid_values[cell_index] = cell_previous_invalid_values[cell_index].union(&new_invalid_values);
      Some((cell, new_invalid_values.into_iter().collect_vec()))
    })
  }

  pub(crate) fn deduplicate_flat_eliminations<'a>(
    &'a self,
    invalid_candidates: Vec<(CellPosition, u32)>,
    cell_previous_invalid_values: &'a mut Vec<CandidateSet>,
  ) -> impl Iterator<Item = (CellPosition, u32)> + 'a {
    self.deduplicate_eliminations(
      invalid_candidates.into_iter().into_group_map().into_iter().sorted().collect_vec(),
      cell_previous_invalid_values,
    ).flat_map(|(cell, invalid_values)| {
      invalid_values.into_iter().map(move |value| (cell, value))
    })
  }

  pub(crate) fn all_strong_links(&self) -> Vec<(Area, u32, CellPosition, CellPosition)> {
    self.state.puzzle.all_house_areas().flat_map(|area| {
      self.state.compute_cells_by_value_in_area(&area, &self.state.candidates)
        .filter_map(move |(value, cells)| {
          if cells.len() != 2 {
            return None
          }
          return Some(
            (area.clone(), value, cells[0], cells[1])
          )
        })
    }).collect()
  }

  pub(crate) fn all_strong_links_by_value(&self) -> HashMap<u32, Vec<(Area, u32, CellPosition, CellPosition)>> {
    self.all_strong_links()
      .iter()
      .cloned()
      .sorted_by_key(|link| (link.1, link.0.clone(), link.2, link.3))
      .chunk_by(|link| link.1)
      .into_iter()
      .map(|(value, group)| (value, group.collect()))
      .collect()
  }

  fn enrich_solution_step(&self, step: &mut SolutionStep) {
    step.grid = Some(self.state.grid().clone());
    if self.state.candidates_active {
      let candidates: Vec<Vec<Vec<u32>>> = self.state.candidates.iter().map(|row|
        row.iter().map(|cell|
          cell.clone().into_iter().collect()
        ).collect()
      ).collect();
      step.candidates = Some(candidates);
    }
  }
}

impl Clone for LogicalSolver {
  fn clone(&self) -> Self {
    Self {
      state: self.state.clone(),
      all_techniques_by_kind: self.all_techniques_by_kind.clone(),
      enabled_techniques_by_phase: self.enabled_techniques_by_phase.clone(),
      hint_mode: self.hint_mode.clone(),
      step_count_limit: self.step_count_limit.clone(),
      enriched_steps_enabled: self.enriched_steps_enabled,
      performance_mode: self.performance_mode,
      caches: self.caches.clone(),
    }
  }
}

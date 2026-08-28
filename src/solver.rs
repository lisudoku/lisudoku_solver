use crate::solver::constraints::fixed_numbers::FixedNumbersConstraint;
use crate::solver::constraints::{CandidateMode, ConstraintDefinition, PeerMode};
use crate::solver::logical_solver::kropki_chain_candidates::KropkiChainCombinationLogicFactory;
use crate::solver::types::{CandidateSet, Puzzle};
use crate::types::{Area, CellPosition, Grid, KropkiDotType, SudokuConstraints};
use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;
use itertools::Itertools;
use self::logical_solver::advanced_candidates::CellEliminationsResult;
use self::logical_solver::arrow_candidates::ArrowCombinationLogicFactory;
use self::logical_solver::combinations::cell_combination_logic::CellsCacheKey;
pub use self::checker::SolvedState;
pub use self::logical_solver::technique::Technique;

pub(crate) mod checker;
pub(crate) mod logical_solver;
pub(crate) mod constraints;
mod types;
mod brute_solver;
pub(crate) mod builder;

pub struct Solver {
  pub(crate) puzzle: Puzzle,
  pub techniques: Vec<Rc<dyn Technique>>,
  pub grid: Grid,
  candidates_active: bool,
  candidates: Vec<Vec<CandidateSet>>,
  hint_mode: bool,
  step_count_limit: Option<usize>,
  enriched_steps_enabled: bool,
  // TODO: think about how to organize these caches
  arrow_combinatons_logic_factory: RefCell<ArrowCombinationLogicFactory>,
  cell_eliminations_cache: RefCell<HashMap<CellsCacheKey, CellEliminationsResult>>,
  area_kropki_ccs_cache: RefCell<HashMap<(Area, KropkiDotType, bool), Vec<(Vec<CellPosition>,Vec<usize>)>>>,
  kropki_chain_combinatons_logic_factory: RefCell<KropkiChainCombinationLogicFactory>,
}

impl Clone for Solver {
  fn clone(&self) -> Self {
    Self {
      puzzle: self.puzzle.clone(),
      techniques: self.techniques.clone(),
      grid: self.grid.clone(),
      candidates_active: self.candidates_active.clone(),
      candidates: self.candidates.clone(),
      hint_mode: self.hint_mode.clone(),
      step_count_limit: self.step_count_limit.clone(),
      enriched_steps_enabled: self.enriched_steps_enabled,
      arrow_combinatons_logic_factory: RefCell::new(ArrowCombinationLogicFactory::new()),
      cell_eliminations_cache: self.cell_eliminations_cache.clone(),
      area_kropki_ccs_cache: self.area_kropki_ccs_cache.clone(),
      kropki_chain_combinatons_logic_factory: RefCell::new(KropkiChainCombinationLogicFactory::new()),
    }
  }
}

impl Solver {
  pub fn new(input_constraints: SudokuConstraints) -> Solver {
    let grid = FixedNumbersConstraint::build_initial_grid_from_fixed_numbers(
      input_constraints.fixed_numbers.as_ref().unwrap_or(&vec![]),
      input_constraints.grid_size,
    );
    let candidates = Self::build_initial_candidates(input_constraints.grid_size);

    Solver {
      puzzle: Puzzle::try_from(input_constraints).expect("Error while parsing constraints"),
      grid,
      candidates_active: false,
      candidates,
      hint_mode: false,
      step_count_limit: None,
      techniques: Self::default_techniques(),
      enriched_steps_enabled: false,
      arrow_combinatons_logic_factory: RefCell::new(ArrowCombinationLogicFactory::new()),
      cell_eliminations_cache: RefCell::new(HashMap::new()),
      area_kropki_ccs_cache: RefCell::new(HashMap::new()),
      kropki_chain_combinatons_logic_factory: RefCell::new(KropkiChainCombinationLogicFactory::new()),
    }
  }

  fn build_initial_candidates(grid_size: usize) -> Vec<Vec<CandidateSet>> {
    vec![ vec![ CandidateSet::new(); grid_size ]; grid_size ]
  }

  /// Returns a nxn grid containing each cell's candidate set
  fn compute_all_cell_candidates(&self) -> Vec<Vec<CandidateSet>> {
    if self.candidates_active {
      return self.candidates.clone();
    }
    let mut candidates: Vec<Vec<CandidateSet>> = vec![
      vec![ CandidateSet::new(); self.puzzle.grid_size ];
      self.puzzle.grid_size
    ];
    for cell in &self.all_empty_cells() {
      candidates[cell.row][cell.col] = self.compute_cell_candidates(cell);
    }
    candidates
  }

  /// Returns a <cell>'s candidate set (cached)
  fn compute_cell_candidates(&self, cell: &CellPosition) -> CandidateSet {
    if self.grid[cell.row][cell.col] != 0 {
      return CandidateSet::new()
    }

    if self.candidates_active {
      return self.candidates[cell.row][cell.col].clone();
    }

    self.recompute_cell_candidates(cell)
  }

  /// Computes and returns a <cell>'s candidate set.
  /// Note: We don't apply all restrictions at this level (e.g. thermo, palindrome)
  fn recompute_cell_candidates(&self, cell: &CellPosition) -> CandidateSet {
    let mut candidates = self.full_candidate_set();

    for peer in self.puzzle.all_cell_peers(cell, PeerMode::Full, CandidateMode::Simple) {
      let value = self.grid[peer.row][peer.col];
      if value != 0 {
        candidates.remove(&value);
      }
    }

    for constraint in &self.puzzle.constraints {
      constraint.restrict_cell_special_candidates(cell, &self.grid, &mut candidates);
    }

    candidates
  }

  fn full_candidate_set(&self) -> CandidateSet {
    CandidateSet::all_candidates(self.puzzle.grid_size as u32)
  }

  fn area_values(&self, area: &Area) -> Vec<u32> {
    self.puzzle.area_cells(&area).into_iter().map(|cell| {
      let value = self.grid[cell.row][cell.col];
      value
    }).collect()
  }

  fn filter_empty_cells(&self, cells: &Vec<CellPosition>) -> Vec<CellPosition> {
    cells.into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).copied().collect()
  }

  fn area_empty_cells(&self, area: &Area) -> Vec<CellPosition> {
    self.puzzle.area_cells(area).into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).collect()
  }

  fn all_empty_cells(&self) -> Vec<CellPosition> {
    self.puzzle.all_grid_cells().into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).collect()
  }

  /// Returns the candidate set for the given <area>
  fn compute_area_candidates_union(&self, area: &Area) -> CandidateSet {
    let mut area_candidates = CandidateSet::new();
    for cell in self.puzzle.area_cells(area) {
      let cell_candidates = self.compute_cell_candidates(&cell);
      area_candidates = area_candidates.union(&cell_candidates);
    }
    area_candidates
  }

  fn area_cells_with_candidate(&self, area: &Area, value: u32) -> Vec<CellPosition> {
    self.area_cells_with_candidates(area, &CandidateSet::from_iter([ value ].into_iter()))
  }

  fn filter_cells_with_any_candidates(&self, cells: &Vec<CellPosition>, values: &CandidateSet) -> Vec<CellPosition> {
    cells
      .iter()
      .filter(|&cell| !self.compute_cell_candidates(cell).is_disjoint(values))
      .copied()
      .collect()
  }

  fn area_cells_with_candidates(&self, area: &Area, values: &CandidateSet) -> Vec<CellPosition> {
    let area_cells = self.area_empty_cells(area);
    self.filter_cells_with_any_candidates(&area_cells, values)
  }

  fn compute_cells_by_value_in_area(&self, area: &Area, candidates: &Vec<Vec<CandidateSet>>) -> HashMap<u32, Vec<CellPosition>> {
    let mut value_cells: HashMap<u32, Vec<CellPosition>> = HashMap::new();
    for cell in self.area_empty_cells(area) {
      for value in (&candidates[cell.row][cell.col]).into_iter().sorted() {
        let entry = value_cells.entry(value).or_insert(vec![]);
        entry.push(cell);
      }
    }
    value_cells
  }

  fn cell_peers_with_candidate(&self, cell: &CellPosition, candidate_value: u32) -> Vec<CellPosition> {
    let candidates = CandidateSet::from([ candidate_value ]);
    self.cell_peers_with_candidates(cell, &candidates)
  }

  /// Returns cells that <cell> sees which have any of <values> candidates
  fn cell_peers_with_candidates(&self, cell: &CellPosition, values: &CandidateSet) -> Vec<CellPosition> {
    let peers = self.puzzle.all_cell_peers_unconstrained(cell);
    self.filter_cells_with_any_candidates(&peers, values)
  }

  fn count_empty_cells_in_list(&self, cells: &Vec<CellPosition>) -> usize {
    cells.into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).count()
  }
}

#[cfg(test)]
mod tests;

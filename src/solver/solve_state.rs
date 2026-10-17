use crate::{solver::{constraints::{CandidateMode, ConstraintDefinition, PeerMode, fixed_numbers::FixedNumbersConstraint}, types::{CandidateSet, CellPositionSet, Puzzle}}, types::{Area, CellPosition, Grid, SudokuConstraints}};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct SolveState {
  pub puzzle: Puzzle,
  pub grid: Grid,
  pub candidates: Vec<Vec<CandidateSet>>,
  pub candidates_active: bool,
}

impl SolveState {
  pub fn from_constraints_and_grid(constraints: SudokuConstraints, grid: Grid) -> SolveState {
    let candidates = vec![
      vec![ CandidateSet::new(); constraints.grid_size ];
      constraints.grid_size
    ];

    SolveState {
      puzzle: Puzzle::try_from(constraints).expect("Error while parsing constraints"),
      grid,
      candidates_active: false,
      candidates,
    }
  }

  pub fn new(constraints: SudokuConstraints) -> SolveState {
    let grid = FixedNumbersConstraint::build_initial_grid_from_fixed_numbers(
      constraints.fixed_numbers.as_ref().unwrap_or(&vec![]),
      constraints.grid_size,
    );

    Self::from_constraints_and_grid(constraints, grid)
  }

  /// Returns a nxn grid containing each cell's candidate set
  pub fn compute_all_cell_candidates(&self) -> Vec<Vec<CandidateSet>> {
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
  pub fn compute_cell_candidates(&self, cell: &CellPosition) -> CandidateSet {
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
  pub fn recompute_cell_candidates(&self, cell: &CellPosition) -> CandidateSet {
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

  pub fn full_candidate_set(&self) -> CandidateSet {
    CandidateSet::all_candidates(self.puzzle.grid_size as u32)
  }

  pub fn area_values(&self, area: &Area) -> Vec<u32> {
    self.puzzle.area_cells(&area).into_iter().map(|cell| {
      let value = self.grid[cell.row][cell.col];
      value
    }).collect()
  }

  pub fn filter_empty_cells(&self, cells: &Vec<CellPosition>) -> Vec<CellPosition> {
    cells.into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).copied().collect()
  }

  pub fn area_empty_cells(&self, area: &Area) -> Vec<CellPosition> {
    self.puzzle.area_cells(area).into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).collect()
  }

  pub fn all_empty_cells(&self) -> Vec<CellPosition> {
    self.puzzle.all_grid_cells().into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).collect()
  }

  /// Returns the candidate set for the given <area>
  pub fn compute_area_candidates_union(&self, area: &Area) -> CandidateSet {
    let mut area_candidates = CandidateSet::new();
    for cell in self.puzzle.area_cells(area) {
      let cell_candidates = self.compute_cell_candidates(&cell);
      area_candidates = area_candidates.union(&cell_candidates);
    }
    area_candidates
  }

  pub fn area_cells_with_candidate(&self, area: &Area, value: u32) -> Vec<CellPosition> {
    self.area_cells_with_candidates(area, &CandidateSet::from_iter([ value ].into_iter()))
  }

  pub fn filter_cells_with_any_candidates(&self, cells: &Vec<CellPosition>, values: &CandidateSet) -> Vec<CellPosition> {
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

  fn compute_cells_by_value(&self, cells: &Vec<CellPosition>, candidates: &Vec<Vec<CandidateSet>>) -> HashMap<u32, Vec<CellPosition>> {
    let mut value_cells: Vec<CellPositionSet> = vec![
      CellPositionSet::new(self.puzzle.grid_size);
      self.puzzle.grid_size
    ];
    for cell in self.filter_empty_cells(cells).into_iter() {
      for value in &candidates[cell.row][cell.col] {
        value_cells[value as usize - 1].insert(cell);
      }
    }
    value_cells
      .into_iter()
      .enumerate()
      .filter(|e| e.1.len() > 0)
      .map(|(value_index, cell_position_set)|
        (value_index as u32 + 1, cell_position_set.into_iter().collect())
      )
      .collect()
  }

  pub fn compute_cells_by_value_in_area(&self, area: &Area, candidates: &Vec<Vec<CandidateSet>>) -> HashMap<u32, Vec<CellPosition>> {
    let cells = self.puzzle.area_cells(area);
    self.compute_cells_by_value(&cells, candidates)
  }

  pub fn cell_peers_with_candidate(&self, cell: &CellPosition, candidate_value: u32) -> Vec<CellPosition> {
    let candidates = CandidateSet::from([ candidate_value ]);
    self.cell_peers_with_candidates(cell, &candidates)
  }

  /// Returns cells that <cell> sees which have any of <values> candidates
  pub fn cell_peers_with_candidates(&self, cell: &CellPosition, values: &CandidateSet) -> Vec<CellPosition> {
    let peers = self.puzzle.all_cell_peers_unconstrained(cell);
    self.filter_cells_with_any_candidates(&peers, values)
  }

  pub fn count_empty_cells_in_list(&self, cells: &Vec<CellPosition>) -> usize {
    cells.into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).count()
  }
}

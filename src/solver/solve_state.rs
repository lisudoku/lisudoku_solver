use crate::{solver::{constraints::{ConstraintDefinition, fixed_numbers::FixedNumbersConstraint}, types::{CandidateGrid, CandidateSet, CellPositionSet, Puzzle}}, types::{Area, CellPosition, Grid, SudokuConstraints}};

#[derive(Clone, Default)]
pub(crate) struct SolveState {
  pub(crate) puzzle: Puzzle,
  grid: Grid,
  pub(crate) candidates: CandidateGrid,
  pub(crate) candidates_active: bool,
  empty_cells: CellPositionSet,
  region_area_values: Vec<CandidateSet>,
  /// Same as the logical_solver field. Skips as many non-essential things as possible.
  pub(crate) performance_mode: bool,
}

impl SolveState {
  pub(crate) fn from_constraints_and_grid(constraints: SudokuConstraints, grid: Grid) -> SolveState {
    let grid_size = constraints.grid_size;

    let puzzle = Puzzle::try_from(constraints).expect("Error while parsing constraints");

    let mut empty_cells = CellPositionSet::new(grid_size);
    for cell in CellPosition::all_cells_in_grid_size(grid_size) {
      if grid[cell.row][cell.col] == 0 {
        empty_cells.insert(cell);
      }
    }

    let mut region_area_values: Vec<CandidateSet> = vec![CandidateSet::new(); puzzle.all_region_areas().count()];

    for cell in CellPosition::all_cells_in_grid_size(grid_size) {
      let cell_index = cell.to_index(grid_size);
      for area_id in &puzzle.cell_area_ids[cell_index] {
        let value = grid[cell.row][cell.col];
        if value != 0 {
          region_area_values[*area_id].insert(value);
        }
      }
    }

    let candidates = CandidateGrid::new(grid_size);

    SolveState {
      puzzle,
      grid,
      candidates_active: false,
      candidates,
      empty_cells,
      region_area_values,
      performance_mode: false,
    }
  }

  pub(crate) fn new(constraints: SudokuConstraints) -> SolveState {
    let grid = FixedNumbersConstraint::build_initial_grid_from_fixed_numbers(
      constraints.fixed_numbers.as_ref().unwrap_or(&vec![]),
      constraints.grid_size,
    );

    Self::from_constraints_and_grid(constraints, grid)
  }

  pub(crate) fn set_value(&mut self, cell: &CellPosition, value: u32) {
    let previous_value = self.grid[cell.row][cell.col];

    assert_ne!(
      self.grid[cell.row][cell.col] == 0,
      value == 0,
      "cannot set value from {} to {}", self.grid[cell.row][cell.col], value,
    );

    self.grid[cell.row][cell.col] = value;

    let cell_index = cell.to_index(self.puzzle.grid_size);
    for &area_id in &self.puzzle.cell_area_ids[cell_index] {
      if value == 0 {
        self.region_area_values[area_id].remove(&previous_value);
      } else {
        self.region_area_values[area_id].insert(value);
      }
    }

    if value == 0 {
      self.empty_cells.insert(*cell);
    } else {
      self.empty_cells.remove(cell);
    }
  }

  pub(crate) fn clear_value(&mut self, cell: &CellPosition) {
    self.set_value(cell, 0);
  }

  pub(crate) fn grid(&self) -> &Grid {
    &self.grid
  }

  pub(crate) fn value_at(&self, cell: &CellPosition) -> u32 {
    self.grid[cell.row][cell.col]
  }

  /// Returns a nxn grid containing each cell's candidate set
  pub(crate) fn compute_all_cell_candidates(&self) -> CandidateGrid {
    if self.candidates_active {
      return self.candidates.clone();
    }
    let mut candidates = CandidateGrid::new(self.puzzle.grid_size);
    for cell in self.all_empty_cells() {
      candidates[cell.row][cell.col] = self.compute_cell_candidates(&cell);
    }
    candidates
  }

  /// Returns `cell`'s candidate set (cached)
  pub(crate) fn compute_cell_candidates(&self, cell: &CellPosition) -> CandidateSet {
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
  /// Note: We don't call cell_peers, we only check region area membership, so any
  /// custom cell_peers implementation needs to be included in restrict_cell_special_candidates too.
  pub(crate) fn recompute_cell_candidates(&self, cell: &CellPosition) -> CandidateSet {
    let cell_index = cell.to_index(self.puzzle.grid_size);
    let mut cell_candidates = CandidateSet::all_candidates(self.puzzle.grid_size as u32);
    for &area_id in &self.puzzle.cell_area_ids[cell_index] {
      cell_candidates = cell_candidates.difference(&self.region_area_values[area_id]);
    }
    for constraint in &self.puzzle.constraints {
      constraint.restrict_cell_special_candidates(&cell, &self.grid, &mut cell_candidates);
    }

    cell_candidates
  }

  pub(crate) fn full_candidate_set(&self) -> CandidateSet {
    CandidateSet::all_candidates(self.puzzle.grid_size as u32)
  }

  pub(crate) fn area_values(&self, area: &Area) -> Vec<u32> {
    self.puzzle.area_cells(&area).into_iter().map(|cell| {
      let value = self.grid[cell.row][cell.col];
      value
    }).collect()
  }

  pub(crate) fn filter_empty_cells(&self, cells: &Vec<CellPosition>) -> Vec<CellPosition> {
    cells.into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).copied().collect()
  }

  pub(crate) fn area_empty_cells(&self, area: &Area) -> Vec<CellPosition> {
    self.puzzle.area_cells(area).into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).collect()
  }

  pub(crate) fn all_empty_cells(&self) -> impl Iterator<Item = CellPosition> + '_ {
    self.empty_cells.into_iter()
  }

  pub(crate) fn any_empty_cells(&self) -> bool {
    !self.empty_cells.is_empty()
  }

  /// Returns the candidate set for the given <area>
  pub(crate) fn compute_area_candidates_union(&self, area: &Area) -> CandidateSet {
    let mut area_candidates = CandidateSet::new();
    for cell in self.puzzle.area_cells(area) {
      let cell_candidates = self.compute_cell_candidates(&cell);
      area_candidates = area_candidates.union(&cell_candidates);
    }
    area_candidates
  }

  pub(crate) fn area_cells_with_candidate(&self, area: &Area, value: u32) -> Vec<CellPosition> {
    self.area_cells_with_candidates(area, &CandidateSet::from_iter([value]))
  }

  pub(crate) fn filter_cells_with_any_candidates(&self, cells: &Vec<CellPosition>, values: &CandidateSet) -> Vec<CellPosition> {
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

  pub(crate) fn compute_cells_by_value_in_area(&self, area: &Area, candidates: &CandidateGrid) -> impl Iterator<Item = (u32, Vec<CellPosition>)> + '_ {
    let area_cells = self.puzzle.area_cells(area);
    let empty_area_cells = self.filter_empty_cells(&area_cells);
    let mut cell_indices_with_value: Vec<CandidateSet> = vec![
      CandidateSet::new();
      self.puzzle.grid_size
    ];

    for (index, cell) in empty_area_cells.iter().enumerate() {
      let cell_candidates = candidates[cell.row][cell.col];
      for value in cell_candidates {
        cell_indices_with_value[value as usize - 1].insert(index as u32);
      }
    }

    cell_indices_with_value
      .into_iter()
      .enumerate()
      .filter(|e| e.1.len() > 0)
      .map(move |(value_index, cell_indices)|
        (
          value_index as u32 + 1,
          cell_indices.into_iter().map(|cell_index| empty_area_cells[cell_index as usize]).collect()
        )
      )
  }

  pub(crate) fn cell_peers_with_candidate(&self, cell: &CellPosition, candidate_value: u32) -> Vec<CellPosition> {
    let candidates = CandidateSet::from([ candidate_value ]);
    self.cell_peers_with_candidates(cell, &candidates)
  }

  /// Returns cells that <cell> sees which have any of <values> candidates
  pub(crate) fn cell_peers_with_candidates(&self, cell: &CellPosition, values: &CandidateSet) -> Vec<CellPosition> {
    let peers = self.puzzle.all_cell_peers_unconstrained(cell);
    self.filter_cells_with_any_candidates(&peers, values)
  }

  pub(crate) fn count_empty_cells_in_list(&self, cells: &Vec<CellPosition>) -> usize {
    cells.into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).count()
  }

  pub(crate) fn update_candidates(&mut self, cells: &Vec<CellPosition>, value: u32) {
    for cell in cells {
      self.candidates[cell.row][cell.col].remove(&value);
    }
  }
}

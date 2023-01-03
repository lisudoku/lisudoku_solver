use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};
use itertools::Itertools;

use crate::types::{SudokuConstraints, SudokuGrid, Grid, Area, CellPosition, CellDirection};

mod checker;
mod intuitive_solver;
mod brute_solver;

const KNIGHT_MOVES: [CellDirection; 8] = [
  CellDirection { row: 1, col: 2 },
  CellDirection { row: 1, col: -2 },
  CellDirection { row: -1, col: 2 },
  CellDirection { row: -1, col: -2 },
  CellDirection { row: 2, col: 1 },
  CellDirection { row: 2, col: -1 },
  CellDirection { row: -2, col: 1 },
  CellDirection { row: -2, col: -1 },
];

pub struct Solver {
  pub constraints: SudokuConstraints,
  pub grid: Grid,
  pub solution: Option<Grid>,
  grid_to_region: Vec<Vec<usize>>,
  grid_to_thermo: Vec<Vec<usize>>,
  candidates_active: bool,
  candidates: Vec<Vec<HashSet<u32>>>,
}

impl Solver {
  pub fn new(constraints: SudokuConstraints, input_grid: Option<SudokuGrid>) -> Solver {
    let mut grid_to_region = vec![ vec![ usize::MAX; constraints.grid_size ]; constraints.grid_size ];
    for (index, region) in constraints.regions.iter().enumerate() {
      for cell in region {
        grid_to_region[cell.row][cell.col] = index;
      }
    }

    let mut grid_to_thermo = vec![ vec![ usize::MAX; constraints.grid_size ]; constraints.grid_size ];
    for (index, thermo) in constraints.thermos.iter().enumerate() {
      for cell in thermo {
        grid_to_thermo[cell.row][cell.col] = index;
      }
    }

    let grid = if input_grid.is_some() {
      input_grid.unwrap().values
    } else {
      let mut initial_grid = vec![ vec![ 0; constraints.grid_size ]; constraints.grid_size ];
      for fixed_number in &constraints.fixed_numbers {
        initial_grid[fixed_number.position.row][fixed_number.position.col] = fixed_number.value;
      }
      initial_grid
    };

    let candidates = vec![ vec![ HashSet::new(); constraints.grid_size ]; constraints.grid_size ];

    Solver {
      constraints,
      grid,
      solution: None,
      grid_to_region,
      grid_to_thermo,
      candidates_active: false,
      candidates,
    }
  }

  fn compute_area_cell_candidates(&self, area: &Area, cell: &CellPosition) -> HashSet<u32> {
    match area {
      &Area::Thermo(thermo_index) => self.compute_thermo_cell_candidates(thermo_index, cell),
      Area::Grid => unimplemented!(),
      _ => self.compute_generic_area_cell_candidates(area),
    }
  }

  fn compute_generic_area_cell_candidates(&self, area: &Area) -> HashSet<u32> {
    let mut set = self.compute_all_candidates();
    for CellPosition { row, col } in self.get_area_cells(area) {
      if self.grid[row][col] != 0 {
        set.remove(&self.grid[row][col]);
      }
    }
    set
  }

  // This could be made more intelligent, but we leave the tricks to intuitive_solver
  fn compute_thermo_cell_candidates(&self, thermo_index: usize, area_cell: &CellPosition) -> HashSet<u32> {
    let thermo = &self.constraints.thermos[thermo_index];

    let mut after = false;
    let mut max_before = 0;
    let mut min_after = self.constraints.grid_size as u32 + 1;

    for cell in thermo {
      if area_cell.row == cell.row && area_cell.col == cell.col {
        after = true;
        continue
      }
      let value = self.grid[cell.row][cell.col];
      if value == 0 {
        continue
      }

      if after {
        min_after = min(min_after, value);
      } else {
        max_before = max(max_before, value);
      }
    }

    let set: HashSet<u32> = (max_before+1..min_after).collect();

    set
  }

  fn compute_cell_candidates(&self, cell: &CellPosition) -> HashSet<u32> {
    if self.candidates_active {
      return self.candidates[cell.row][cell.col].clone();
    }

    self.recompute_cell_candidates(cell)
  }

  fn recompute_cell_candidates(&self, cell: &CellPosition) -> HashSet<u32> {
    // Note: we don't restrict thermo candidates at this level
    let areas = self.get_cell_areas(cell, false);
    let mut candidates = self.compute_all_candidates();
    for area in &areas {
      let area_set = self.compute_area_cell_candidates(area, cell);
      candidates = candidates.intersection(&area_set).cloned().collect();
    }
    if self.constraints.anti_knight {
      let mut knight_set = self.compute_all_candidates();
      for peer in self.get_knight_peers(&cell) {
        let value = self.grid[peer.row][peer.col];
        if value == 0 {
          continue
        }
        knight_set.remove(&value);
      }
      candidates = candidates.intersection(&knight_set).cloned().collect();
    }

    candidates
  }

  fn compute_all_candidates(&self) -> HashSet<u32> {
    (1..self.constraints.grid_size as u32 + 1).collect::<HashSet<u32>>()
  }

  // Note: update when adding new areas
  fn get_cell_areas(&self, cell: &CellPosition, include_thermo: bool) -> Vec<Area> {
    let &CellPosition { row, col } = cell;
    let region_index = self.grid_to_region[row][col];
    let mut areas = vec![ Area::Row(row), Area::Column(col), Area::Region(region_index) ];
    if self.constraints.primary_diagonal && row == col {
      areas.push(Area::PrimaryDiagonal);
    }
    if self.constraints.secondary_diagonal && row == self.constraints.grid_size - 1 - col {
      areas.push(Area::SecondaryDiagonal);
    }
    let thermo_index = self.grid_to_thermo[row][col];
    if include_thermo && thermo_index != usize::MAX {
      areas.push(Area::Thermo(thermo_index));
      // TODO: handle intersecting thermos
    }

    areas
  }

  // Note: update when adding new areas
  fn get_all_areas(&self, include_thermo: bool) -> Vec<Area> {
    let mut areas = vec![];
    areas.extend(self.get_row_areas());
    areas.extend(self.get_col_areas());
    for region_index in 0..self.constraints.regions.len() {
      areas.push(Area::Region(region_index));
    }
    if self.constraints.primary_diagonal {
      areas.push(Area::PrimaryDiagonal);
    }
    if self.constraints.secondary_diagonal {
      areas.push(Area::SecondaryDiagonal);
    }
    if include_thermo {
      for thermo_index in 0..self.constraints.thermos.len() {
        areas.push(Area::Thermo(thermo_index));
      }
    }

    areas
  }

  fn get_row_areas(&self) -> Vec<Area> {
    (0..self.constraints.grid_size).map(|row| Area::Row(row)).collect()
  }

  fn get_col_areas(&self) -> Vec<Area> {
    (0..self.constraints.grid_size).map(|col| Area::Column(col)).collect()
  }

  fn get_area_cells(&self, area: &Area) -> Vec<CellPosition> {
    match area {
      Area::Grid => self.get_grid_cells(),
      Area::Row(row) => self.get_row_cells(*row),
      Area::Column(col) => self.get_col_cells(*col),
      Area::Region(region_index) => self.constraints.regions[*region_index].to_vec(),
      Area::Thermo(thermo_index) => self.constraints.thermos[*thermo_index].to_vec(),
      Area::PrimaryDiagonal => self.get_primary_diagonal_cells(),
      Area::SecondaryDiagonal => self.get_secondary_diagonal_cells(),
    }
  }

  fn get_empty_area_cells(&self, area: &Area) -> Vec<CellPosition> {
    self.get_area_cells(area).into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).collect()
  }

  fn get_all_empty_cells(&self) -> Vec<CellPosition> {
    self.get_empty_area_cells(&Area::Grid)
  }

  fn get_all_cells_with_candidate(&self, value: u32) -> Vec<CellPosition> {
    self.get_all_empty_cells()
        .into_iter()
        .filter(|&CellPosition { row, col }| self.candidates[row][col].contains(&value))
        .collect()
  }

  fn get_grid_cells(&self) -> Vec<CellPosition> {
    (0..self.constraints.grid_size).flat_map(|row| {
      (0..self.constraints.grid_size).map(|col| {
        CellPosition::new(row, col)
      }).collect::<Vec<CellPosition>>()
    }).collect()
  }

  fn get_row_cells(&self, row: usize) -> Vec<CellPosition> {
    (0..self.constraints.grid_size).map(|col| CellPosition::new(row, col)).collect()
  }

  fn get_col_cells(&self, col: usize) -> Vec<CellPosition> {
    (0..self.constraints.grid_size).map(|row| CellPosition::new(row, col)).collect()
  }

  fn get_primary_diagonal_cells(&self) -> Vec<CellPosition> {
    (0..self.constraints.grid_size).map(|index| CellPosition::new(index, index)).collect()
  }

  fn get_secondary_diagonal_cells(&self) -> Vec<CellPosition> {
    (0..self.constraints.grid_size).map(|index| {
      CellPosition::new(index, self.constraints.grid_size - 1 - index)
    }).collect()
  }

  #[allow(dead_code)]
  fn compute_area_candidates_union(&self, area: &Area) -> HashSet<u32> {
    let mut area_candidates: HashSet<u32> = HashSet::new();
    for cell in self.get_area_cells(area) {
      let cell_candidates = self.compute_cell_candidates(&cell);
      area_candidates = area_candidates.union(&cell_candidates).cloned().collect();
    }
    area_candidates
  }

  fn get_area_cells_with_candidate(&self, area: &Area, value: u32) -> Vec<CellPosition> {
    self.get_area_cells_with_candidates(area, &HashSet::from([ value ]))
  }

  fn get_area_cells_with_candidates(&self, area: &Area, values: &HashSet<u32>) -> Vec<CellPosition> {
    self.get_empty_area_cells(area)
        .into_iter()
        .filter(|cell| !self.compute_cell_candidates(&cell).is_disjoint(values))
        .collect()
  }

  fn compute_cells_by_value_in_area(&self, area: &Area, candidates: &Vec<Vec<HashSet<u32>>>) -> HashMap<u32, Vec<CellPosition>> {
    let mut value_cells: HashMap<u32, Vec<CellPosition>> = HashMap::new();
    for cell in self.get_empty_area_cells(area) {
      for value in &candidates[cell.row][cell.col] {
        let entry = value_cells.entry(*value).or_insert(vec![]);
        entry.push(cell);
      }
    }
    value_cells
  }

  fn get_cell_peers_with_candidates(&self, cell: &CellPosition, values: &HashSet<u32>) -> Vec<CellPosition> {
    self.get_cell_peers(cell)
        .into_iter()
        .filter(|cell| !self.compute_cell_candidates(&cell).is_disjoint(values))
        .collect()
  }

  // Note: update when adding constraints
  fn get_cell_peers(&self, cell: &CellPosition) -> Vec<CellPosition> {
    let mut peers: Vec<CellPosition> = self.get_cell_areas(cell, true)
      .iter()
      .flat_map(|area| self.get_area_cells(area))
      .collect();

    if self.constraints.anti_knight {
      peers.extend(self.get_knight_peers(cell));
    }

    peers.into_iter()
         .filter(|other_cell| other_cell != cell)
         .unique()
         .collect()
  }

  fn get_knight_peers(&self, cell: &CellPosition) -> Vec<CellPosition> {
    KNIGHT_MOVES.iter().filter_map(|direction| {
      let prow = cell.row as isize + direction.row;
      let pcol = cell.col as isize + direction.col;
      if prow < 0 || prow >= self.constraints.grid_size as isize ||
         pcol < 0 || pcol >= self.constraints.grid_size as isize {
        return None
      }
      let peer = CellPosition {
        row: prow as usize,
        col: pcol as usize,
      };
      Some(peer)
    }).collect()
  }
}

#[cfg(test)]
mod tests;

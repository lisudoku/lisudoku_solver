use crate::types::{SudokuConstraints, SudokuGrid, Grid, Area, CellPosition, CellDirection, KillerCage, KropkiDot, KropkiDotType};
use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};
use std::ops::BitAnd;
use itertools::Itertools;

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

const ADJACENT_MOVES: [CellDirection; 4] = [
  CellDirection { row: 0, col: 1 },
  CellDirection { row: 0, col: -1 },
  CellDirection { row: 1, col: 0 },
  CellDirection { row: -1, col: 0 },
];

pub struct Solver {
  pub constraints: SudokuConstraints,
  pub grid: Grid,
  pub solution: Option<Grid>,
  grid_to_region: Vec<Vec<usize>>,
  grid_to_thermos: Vec<Vec<Vec<usize>>>,
  grid_to_killer_cage: Vec<Vec<usize>>,
  grid_to_kropki_dots: Vec<Vec<Vec<usize>>>,
  candidates_active: bool,
  candidates: Vec<Vec<HashSet<u32>>>,
  hint_mode: bool,
}

impl Solver {
  pub fn new(mut constraints: SudokuConstraints, input_grid: Option<SudokuGrid>) -> Solver {
    let mut grid_to_region = vec![ vec![ usize::MAX; constraints.grid_size ]; constraints.grid_size ];
    for (index, region) in constraints.regions.iter().enumerate() {
      for cell in region {
        grid_to_region[cell.row][cell.col] = index;
      }
    }

    let mut grid_to_thermos = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, thermo) in constraints.thermos.iter().enumerate() {
      for cell in thermo {
        grid_to_thermos[cell.row][cell.col].push(index);
      }
    }

    let mut grid_to_killer_cage = vec![ vec![ usize::MAX; constraints.grid_size ]; constraints.grid_size ];
    for (index, killer_cage) in constraints.killer_cages.iter().enumerate() {
      for cell in &killer_cage.region {
        grid_to_killer_cage[cell.row][cell.col] = index;
      }
    }

    let mut grid_to_kropki_dots = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, kropki_dot) in constraints.kropki_dots.iter().enumerate() {
      let KropkiDot { dot_type: _, cell_1, cell_2 } = kropki_dot;
      grid_to_kropki_dots[cell_1.row][cell_1.col].push(index);
      grid_to_kropki_dots[cell_2.row][cell_2.col].push(index);
    }
    if constraints.kropki_negative {
      for row in 0..constraints.grid_size {
        for col in 0..constraints.grid_size {
          let cell = CellPosition::new(row, col);
          let adjacent_cells: HashSet<CellPosition> = Self::get_adjacent_cells(cell, constraints.grid_size).into_iter().collect();
          let dot_cells: HashSet<CellPosition> = grid_to_kropki_dots[row][col].iter()
            .map(|&kropki_dot_index| {
              let kropki_dot = &constraints.kropki_dots[kropki_dot_index];
              let other_cell = kropki_dot.other_cell(&cell);
              other_cell
            })
            .collect();
          let negative_cells: HashSet<CellPosition> = adjacent_cells.difference(&dot_cells).copied().collect();
          for negative_cell in negative_cells {
            let kropki_dot_index = constraints.kropki_dots.len();
            grid_to_kropki_dots[cell.row][cell.col].push(kropki_dot_index);
            grid_to_kropki_dots[negative_cell.row][negative_cell.col].push(kropki_dot_index);
            constraints.kropki_dots.push(KropkiDot {
              dot_type: KropkiDotType::Negative,
              cell_1: cell,
              cell_2: negative_cell,
            })
          }
        }
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
      grid_to_thermos,
      grid_to_killer_cage,
      grid_to_kropki_dots,
      candidates_active: false,
      candidates,
      hint_mode: false,
    }
  }

  pub fn with_hint_mode(mut self) -> Self {
    self.hint_mode = true;
    self
  }

  fn get_adjacent_cells(cell: CellPosition, grid_size: usize) -> Vec<CellPosition> {
    ADJACENT_MOVES.iter().filter_map(|direction| {
      let prow = cell.row as isize + direction.row;
      let pcol = cell.col as isize + direction.col;
      if prow < 0 || prow >= grid_size as isize ||
         pcol < 0 || pcol >= grid_size as isize {
        return None
      }
      let peer = CellPosition {
        row: prow as usize,
        col: pcol as usize,
      };
      Some(peer)
    }).collect()
  }

  fn compute_area_cell_candidates(&self, area: &Area, cell: &CellPosition) -> HashSet<u32> {
    match area {
      #[allow(unused_parens)]
      (
        &Area::Row(_) | &Area::Column(_) | &Area::Region(_) |
        &Area::PrimaryDiagonal | &Area::SecondaryDiagonal
      ) => self.compute_generic_area_cell_candidates(area),
      &Area::Thermo(thermo_index) => self.compute_thermo_cell_candidates(thermo_index, cell),
      &Area::KillerCage(killer_cage_index) => self.compute_killer_cell_candidates(killer_cage_index),
      &Area::KropkiDot(kropki_dot_index) => self.compute_kropki_cell_candidates(kropki_dot_index),
      &Area::Grid => unimplemented!(),
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

  fn compute_killer_cell_candidates(&self, killer_cage_index: usize) -> HashSet<u32> {
    let area = Area::KillerCage(killer_cage_index);
    let mut set = self.compute_generic_area_cell_candidates(&area);

    let killer_cage = &self.constraints.killer_cages[killer_cage_index];
    if let Some(sum) = killer_cage.sum {
      for value in (sum+1)..(self.constraints.grid_size as u32 + 1) {
        set.remove(&value);
      }
    }

    set
  }

  fn compute_kropki_cell_candidates(&self, _kropki_dot_index: usize) -> HashSet<u32> {
    // Do not enforce kropki cell candidates directly, use an explicit rule for that
    self.compute_all_candidates()
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
    let killer_cage_index = self.grid_to_killer_cage[row][col];
    if killer_cage_index != usize::MAX {
      areas.push(Area::KillerCage(killer_cage_index));
    }
    if include_thermo {
      for &thermo_index in &self.grid_to_thermos[row][col] {
        areas.push(Area::Thermo(thermo_index));
      }
    }

    areas
  }

  // Note: update when adding new areas
  fn get_all_areas(&self, include_thermo: bool, include_killer: bool, include_kropki: bool) -> Vec<Area> {
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
    if include_killer {
      for killer_cage_index in 0..self.constraints.killer_cages.len() {
        areas.push(Area::KillerCage(killer_cage_index));
      }
    }
    if include_kropki {
      for kropki_dot_index in 0..self.constraints.kropki_dots.len() {
        areas.push(Area::KropkiDot(kropki_dot_index));
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
      &Area::Grid => self.get_grid_cells(),
      &Area::Row(row) => self.get_row_cells(row),
      &Area::Column(col) => self.get_col_cells(col),
      &Area::Region(region_index) => self.constraints.regions[region_index].to_vec(),
      &Area::Thermo(thermo_index) => self.constraints.thermos[thermo_index].to_vec(),
      &Area::KillerCage(killer_cage_index) => {
        self.constraints.killer_cages[killer_cage_index].region.to_vec()
      },
      &Area::KropkiDot(kropki_dot_index) => {
        let kropki_dot = &self.constraints.kropki_dots[kropki_dot_index];
        vec![ kropki_dot.cell_1, kropki_dot.cell_2 ]
      },
      &Area::PrimaryDiagonal => self.get_primary_diagonal_cells(),
      &Area::SecondaryDiagonal => self.get_secondary_diagonal_cells(),
    }
  }

  fn get_empty_area_cells(&self, area: &Area) -> Vec<CellPosition> {
    self.get_area_cells(area).into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).collect()
  }

  fn get_all_empty_cells(&self) -> Vec<CellPosition> {
    self.get_empty_area_cells(&Area::Grid)
  }

  #[allow(dead_code)]
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

  fn filter_cells_with_any_candidates(&self, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> Vec<CellPosition> {
    cells.iter()
        .filter(|&cell| !self.compute_cell_candidates(cell).is_disjoint(values))
        .copied()
        .collect()
  }

  fn filter_cells_with_subset_candidates(&self, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> Vec<CellPosition> {
    cells.iter()
        .filter(|&cell| self.compute_cell_candidates(cell).is_subset(values))
        .copied()
        .collect()
  }

  fn get_area_cells_with_candidates(&self, area: &Area, values: &HashSet<u32>) -> Vec<CellPosition> {
    let area_cells = self.get_empty_area_cells(area);
    self.filter_cells_with_any_candidates(&area_cells, values)
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
    let peers = self.get_cell_peers(cell);
    self.filter_cells_with_any_candidates(&peers, values)
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

  fn is_empty_area_subset(&self, small_area: &Area, big_area: &Area) -> bool {
    let small_set: HashSet<CellPosition> = self.get_empty_area_cells(small_area).into_iter().collect();
    if small_set.is_empty() {
      return false
    }

    let big_set: HashSet<CellPosition> = self.get_area_cells(big_area).into_iter().collect();
    big_set.is_superset(&small_set)
  }

  fn get_subset_area_sum(&self, killer_cage: &KillerCage, big_area: &Area) -> u32 {
    let big_set: HashSet<CellPosition> = self.get_area_cells(big_area).into_iter().collect();
    let killer_cells_sum: u32 = killer_cage.region.iter().map(|&cell| {
      if !big_set.contains(&cell) {
        self.grid[cell.row][cell.col]
      } else {
        0
      }
    }).sum();
    killer_cage.sum.unwrap() - killer_cells_sum
  }

  #[allow(dead_code)]
  fn bit_mask_to_hash_set(&self, combination_mask: u32) -> HashSet<u32> {
    (1..self.constraints.grid_size+1).filter_map(|value| {
      if combination_mask.bitand(1 << value) > 0 {
        Some(value as u32)
      } else {
        None
      }
    }).collect()
  }
}

#[cfg(test)]
mod tests;

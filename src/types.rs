use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokuConstraints {
  pub grid_size: usize,
  pub fixed_numbers: Vec<FixedNumber>,
  pub regions: Vec<Region>,
  pub thermos: Vec<Thermo>,
  pub primary_diagonal: bool,
  pub secondary_diagonal: bool,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct FixedNumber {
  pub position: CellPosition,
  pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellPosition {
  pub row: usize,
  pub col: usize,
}

impl CellPosition {
  pub fn new(row: usize, col: usize) -> CellPosition {
    CellPosition {
      row,
      col,
    }
  }
}

pub type Region = Vec<CellPosition>;

pub type Grid = Vec<Vec<u32>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokuGrid {
  pub values: Grid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokuIntuitiveSolveResult {
  pub solution_type: SolutionType,
  pub solution: Option<Grid>,
  pub steps: Vec<SolutionStep>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SolutionType {
  Full,
  Partial,
  None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokuBruteSolveResult {
  pub solution_count: u32,
  pub solution: Option<Grid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolutionStep {
  pub rule: Rule,
  pub cells: Vec<CellPosition>,
  pub values: Vec<u32>,
  pub areas: Vec<Area>,
  pub affected_cells: Vec<CellPosition>,
  pub candidates: Option<Vec<Vec<HashSet<u32>>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Rule {
  NakedSingle, // 1 Cell Position, 1 value + who it is constrained by
  HiddenSingle, // 1 Cell Position, 1 value, the row/col/region + who it is constrained by
  Thermo,
  Candidates,
  ThermoCandidates,
  LockedCandidatesPairs, // 2 CellPositions + what they affect
  NakedPairs, // 2 Cell Positions, 2 values + what they affect
  HiddenPairs,
  LockedCandidatesTriples,
  NakedTriples, // 2 Cell Positions, 2 values + what they affect
  HiddenTriples,
  XWing,
  YWing,
  XYWing,
  Swordfish, // ???
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Area {
  Grid,
  Row(usize),
  Column(usize),
  Region(usize),
  Thermo(usize),
  PrimaryDiagonal,
  SecondaryDiagonal,
}

pub type Thermo = Vec<CellPosition>;

impl SudokuConstraints {
  #[cfg(test)]
  pub fn new(grid_size: usize, fixed_numbers: Vec<FixedNumber>) -> SudokuConstraints {
    SudokuConstraints {
      grid_size,
      fixed_numbers,
      regions: SudokuConstraints::default_regions(grid_size),
      thermos: vec![],
      primary_diagonal: false,
      secondary_diagonal: false,
    }
  }

  #[allow(dead_code)]
  pub fn default_regions(grid_size: usize) -> Vec<Region> {
    let (region_height, region_width) = SudokuConstraints::compute_region_sizes(grid_size);

    let mut regions: Vec<Region> = vec![];
    for region_row_index in 0..(grid_size / region_height) {
      for region_col_index in 0..(grid_size / region_width) {
        let mut region: Region = vec![];
        for row_index in 0..region_height {
          for col_index in 0..region_width {
            let cell = CellPosition {
              row: region_row_index * region_height + row_index,
              col: region_col_index * region_width + col_index,
            };
            region.push(cell);
          }
        }
        regions.push(region);
      }
    }

    regions
  }

  pub fn compute_region_sizes(grid_size: usize) -> (usize, usize) {
    if grid_size == 4 {
      (2, 2)
    } else if grid_size == 6 {
      (2, 3)
    } else {
      (3, 3)
    }
  }
}

impl FixedNumber {
  #[cfg(test)]
  pub fn new(row: usize, col: usize, value: u32) -> FixedNumber {
    FixedNumber {
      position: CellPosition {
        row,
        col,
      },
      value,
    }
  }
}

impl SudokuIntuitiveSolveResult {
  pub fn no_solution() -> SudokuIntuitiveSolveResult {
    SudokuIntuitiveSolveResult {
      solution_type: SolutionType::None,
      solution: None,
      steps: vec![],
    }
  }
}

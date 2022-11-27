use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokuConstraints {
  pub grid_size: usize,
  pub fixed_numbers: Vec<FixedNumber>,
  pub regions: Vec<Region>,
  pub thermos: Vec<Thermo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixedNumber {
  pub position: CellPosition,
  pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct CellPosition {
  pub row: usize,
  pub col: usize,
}

pub type Region = Vec<CellPosition>;

pub type Grid = Vec<Vec<u32>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokuGrid {
  pub values: Grid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokuSolveResult {
  pub solution_count: u32,
  pub solution: Grid,
  pub steps: Vec<SolutionStep>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolutionStep {
  pub rule: Rule,
  pub cells: Vec<CellPosition>,
  pub values: Vec<u32>,
  pub areas: Vec<Area>,
  pub affected_cells: Vec<CellPosition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Rule {
  ObviousSingle, // 1 Cell Position, 1 value + who it is constrained by
  HiddenSingle, // 1 Cell Position, 1 value, the row/col/region + who it is constrained by
  NakedPairs, // 2 Cell Positions, 2 values + what they affect
  PointingPairs, // 2 CellPositions + what they affect
  XWing, // 4 CellPositions
  YWing, // 4 CellPositions
  Swordfish, // ???
  Thermo,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Area {
  Row(usize),
  Column(usize),
  Region(usize),
  Thermo(usize),
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

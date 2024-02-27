use std::{collections::HashSet, fmt::{self, Display, Debug}};
use itertools::Itertools;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SudokuConstraints {
  pub grid_size: usize,
  pub fixed_numbers: Vec<FixedNumber>,
  pub regions: Vec<Region>,
  pub extra_regions: Vec<Region>,
  pub killer_cages: Vec<KillerCage>,
  pub thermos: Vec<Thermo>,
  pub arrows: Vec<Arrow>,
  pub primary_diagonal: bool,
  pub secondary_diagonal: bool,
  pub anti_knight: bool,
  pub anti_king: bool,
  pub kropki_dots: Vec<KropkiDot>,
  pub kropki_negative: bool,
  pub odd_cells: Vec<CellPosition>,
  pub even_cells: Vec<CellPosition>,
  pub top_bottom: bool,
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

  pub fn to_string(&self) -> String {
    format!("R{}C{}", self.row + 1, self.col + 1)
  }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellDirection {
  pub row: isize,
  pub col: isize,
}

pub type Region = Vec<CellPosition>;

pub type Grid = Vec<Vec<u32>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KillerCage {
  pub sum: Option<u32>,
  pub region: Region,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KropkiDot {
  pub dot_type: KropkiDotType,
  pub cell_1: CellPosition,
  pub cell_2: CellPosition,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum KropkiDotType {
  Consecutive,
  Double,
  Negative,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokuGrid {
  pub values: Grid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SudokulogicalSolveResult {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SolutionStep {
  pub rule: Rule,
  pub cells: Vec<CellPosition>,
  pub values: Vec<u32>,
  pub areas: Vec<Area>,
  pub affected_cells: Vec<CellPosition>,
  pub candidates: Option<Vec<Vec<HashSet<u32>>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum Rule {
  NakedSingle, // 1 Cell Position, 1 value + who it is constrained by
  HiddenSingle, // 1 Cell Position, 1 value, the row/col/region + who it is constrained by
  Thermo,
  Kropki,
  Candidates,
  AdvancedCandidates,
  ThermoCandidates,
  KillerCandidates,
  ArrowCandidates,
  ArrowAdvancedCandidates,
  CommonPeerEliminationArrow,
  Killer45,
  KropkiChainCandidates,
  KropkiAdvancedCandidates,
  TopBottomCandidates,
  LockedCandidatesPairs, // 2 CellPositions + what they affect
  NakedPairs, // 2 Cell Positions, 2 values + what they affect
  HiddenPairs,
  CommonPeerElimination, // cells = have common peers, affected_cells = would eliminate them
  CommonPeerEliminationKropki,
  LockedCandidatesTriples,
  NakedTriples, // 2 Cell Positions, 2 values + what they affect
  HiddenTriples,
  XWing,
  XYWing,
  Swordfish, // ???
  TurbotFish,
  EmptyRectangles,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Area {
  Grid,
  Row(usize),
  Column(usize),
  Region(usize),
  Thermo(usize),
  Arrow(usize),
  KillerCage(usize),
  KropkiDot(usize),
  PrimaryDiagonal,
  SecondaryDiagonal,
}

pub type Thermo = Vec<CellPosition>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arrow {
  pub circle_cells: Vec<CellPosition>,
  pub arrow_cells: Vec<CellPosition>,
}

impl Display for Rule {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      Debug::fmt(self, f)
  }
}

impl Arrow {
  pub fn all_cells(&self) -> Vec<CellPosition> {
    [
      self.arrow_cells.to_vec(),
      self.circle_cells.iter().sorted().copied().collect()
    ].concat()
  }
}

impl SudokuConstraints {
  pub fn new(grid_size: usize, fixed_numbers: Vec<FixedNumber>) -> SudokuConstraints {
    SudokuConstraints {
      grid_size,
      fixed_numbers,
      regions: SudokuConstraints::default_regions(grid_size),
      extra_regions: vec![],
      killer_cages: vec![],
      thermos: vec![],
      arrows: vec![],
      primary_diagonal: false,
      secondary_diagonal: false,
      anti_knight: false,
      anti_king: false,
      kropki_dots: vec![],
      kropki_negative: false,
      odd_cells: vec![],
      even_cells: vec![],
      top_bottom: false,
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

  #[cfg(test)]
  pub fn with_top_bottom(mut self) -> Self {
    self.top_bottom = true;
    self
  }

  #[cfg(test)]
  pub fn with_anti_king(mut self) -> Self {
    self.anti_king = true;
    self
  }

  pub fn to_lz_string(&self) -> String {
    let json = serde_json::to_string(&self).unwrap();
    let lz_string = lz_str::compress_to_base64(&json);
    lz_string
  }

  pub fn to_grid_string(&self) -> String {
    SudokuGrid::from_fixed_numbers(self.grid_size, &self.fixed_numbers).to_string(Some("\n"))
  }

  pub fn to_import_string(&self) -> String {
    self.to_grid_string().replace("\n", "")
  }
}

impl FixedNumber {
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

impl KropkiDot {
  #[cfg(test)]
  pub fn consecutive(cell_1: CellPosition, cell_2: CellPosition) -> KropkiDot {
    KropkiDot {
      dot_type: KropkiDotType::Consecutive,
      cell_1,
      cell_2,
    }
  }

  #[cfg(test)]
  pub fn double(cell_1: CellPosition, cell_2: CellPosition) -> KropkiDot {
    KropkiDot {
      dot_type: KropkiDotType::Double,
      cell_1,
      cell_2,
    }
  }

  pub fn other_cell(&self, cell: &CellPosition) -> CellPosition {
    if self.cell_1.eq(cell) {
      self.cell_2
    } else {
      self.cell_1
    }
  }

  pub fn check_values(&self, value1: u32, value2: u32) -> bool {
    value1 == 0 ||
      value2 == 0 ||
      (
        self.dot_type != KropkiDotType::Negative && (
          self.apply_operation(value1) == value2 ||
          self.apply_operation(value2) == value1
        )
      ) ||
      (
        self.dot_type == KropkiDotType::Negative &&
        value1 + 1 != value2 && value2 + 1 != value1 &&
        value1 * 2 != value2 && value2 * 2 != value1
      )
  }

  fn apply_operation(&self, value: u32) -> u32 {
    match self.dot_type {
      KropkiDotType::Consecutive => value + 1,
      KropkiDotType::Double => value * 2,
      KropkiDotType::Negative => unimplemented!(),
    }
  }
}

impl SudokulogicalSolveResult {
  pub fn no_solution() -> SudokulogicalSolveResult {
    SudokulogicalSolveResult {
      solution_type: SolutionType::None,
      solution: None,
      steps: vec![],
    }
  }
}

impl SolutionStep {
  pub fn new(
    rule: Rule, cells: Vec<CellPosition>, values: Vec<u32>, areas: Vec<Area>, affected_cells: Vec<CellPosition>
  ) -> SolutionStep {
    SolutionStep {
      rule,
      cells,
      values,
      areas,
      affected_cells,
      candidates: None,
    }
  }

  pub fn is_grid_step(&self) -> bool {
    [ Rule::NakedSingle, Rule::HiddenSingle, Rule::Thermo ].contains(&self.rule)
  }
}

impl SudokuGrid {
  pub fn new(grid: Grid) -> Self {
    SudokuGrid {
      values: grid,
    }
  }

  pub fn from_string(grid_str: String) -> Self {
    let grid_size = f32::sqrt(grid_str.len() as f32) as usize;
    assert_eq!(grid_size * grid_size, grid_str.len(), "Invalid grid passed");
    let grid_chars = grid_str.chars().collect_vec();
    let grid: Grid = (0..grid_size).map(|row| {
      (0..grid_size).map(|col| {
        let index = row * grid_size + col;
        grid_chars[index].to_digit(10).unwrap()
      }).collect()
    }).collect();
    Self {
      values: grid,
    }
  }

  pub fn from_fixed_numbers(grid_size: usize, fixed_numbers: &Vec<FixedNumber>) -> Self {
    let mut grid: Grid = vec![ vec![ 0; grid_size ]; grid_size ];
    for fixed_number in fixed_numbers {
      let cell = fixed_number.position;
      grid[cell.row][cell.col] = fixed_number.value;
    }
    SudokuGrid::new(grid)
  }

  pub fn to_fixed_numbers(&self) -> Vec<FixedNumber> {
    let mut fixed_numbers = vec![];
    for (row_index, row) in self.values.iter().enumerate() {
      for (col_index, &value) in row.iter().enumerate() {
        if value != 0 {
          fixed_numbers.push(FixedNumber::new(row_index, col_index, value));
        }
      }
    }
    fixed_numbers
  }

  pub fn to_string(&self, separator: Option<&str>) -> String {
    self.values
      .iter()
      .map(|row| {
        row.iter()
          .map(|digit| digit.to_string() )
          .collect::<Vec<String>>()
          .join("")
      })
      .collect::<Vec<String>>()
      .join(separator.unwrap_or(""))
  }
}

impl Area {
  pub fn to_string(&self) -> String {
    match self {
      Area::Row(row) => format!("row {}", row + 1),
      Area::Column(col) => format!("column {}", col + 1),
      Area::Region(region) => format!("box {}", region + 1),
      Area::Grid | Area::Thermo(_) | Area::Arrow(_) | Area::KillerCage(_) | Area::KropkiDot(_) |
        Area::PrimaryDiagonal | Area::SecondaryDiagonal => unimplemented!(),
    }
  }
}

// https://www.reddit.com/r/sudoku/comments/11kpwbt/fun_puzzle_link_in_comment/
#[test]
fn check_sudoku_constraints_import_string() {
  let fixed_numbers = SudokuGrid::new(vec![
    vec![ 2, 0, 3, 0, 0, 0, 1, 0, 7 ],
    vec![ 9, 1, 0, 0, 4, 0, 0, 2, 6 ],
    vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 3, 0, 8, 0, 1, 0, 7, 0, 9 ],
    vec![ 1, 0, 0, 0, 0, 0, 0, 0, 2 ],
    vec![ 0, 0, 2, 0, 0, 0, 8, 0, 0 ],
    vec![ 0, 0, 4, 1, 6, 7, 2, 0, 0 ],
    vec![ 7, 0, 0, 0, 8, 0, 0, 0, 1 ],
    vec![ 8, 0, 0, 2, 0, 9, 0, 0, 3 ],
  ]).to_fixed_numbers();
  let constraints = SudokuConstraints::new(9, fixed_numbers);
  assert_eq!(constraints.to_import_string(), String::from(
    "203000107910040026000000000308010709100000002002000800004167200700080001800209003"
  ))
}

#[test]
fn check_sudoku_grid_from_string() {
  let grid_str = String::from("203000107910040026000000000308010709100000002002000800004167200700080001800209003");
  let grid = SudokuGrid::from_string(grid_str).values;
  let expected_grid = vec![
    vec![ 2, 0, 3, 0, 0, 0, 1, 0, 7 ],
    vec![ 9, 1, 0, 0, 4, 0, 0, 2, 6 ],
    vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 3, 0, 8, 0, 1, 0, 7, 0, 9 ],
    vec![ 1, 0, 0, 0, 0, 0, 0, 0, 2 ],
    vec![ 0, 0, 2, 0, 0, 0, 8, 0, 0 ],
    vec![ 0, 0, 4, 1, 6, 7, 2, 0, 0 ],
    vec![ 7, 0, 0, 0, 8, 0, 0, 0, 1 ],
    vec![ 8, 0, 0, 2, 0, 9, 0, 0, 3 ],
  ];
  assert_eq!(grid, expected_grid);
}

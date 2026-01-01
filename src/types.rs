use std::fmt::{self, Display, Debug};
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use tsify::Tsify;
use derive_more::{Deref, DerefMut};

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SudokuConstraints {
  pub grid_size: usize,
  #[tsify(optional)]
  pub fixed_numbers: Option<Vec<FixedNumber>>,
  #[tsify(optional)]
  pub regions: Option<Vec<Region>>,
  #[tsify(optional)]
  pub extra_regions: Option<Vec<Region>>,
  #[tsify(optional)]
  pub killer_cages: Option<Vec<KillerCage>>,
  #[tsify(optional)]
  pub thermos: Option<Vec<Thermo>>,
  #[tsify(optional)]
  pub arrows: Option<Vec<Arrow>>,
  #[tsify(optional)]
  pub primary_diagonal: Option<bool>,
  #[tsify(optional)]
  pub secondary_diagonal: Option<bool>,
  #[tsify(optional)]
  pub anti_knight: Option<bool>,
  #[tsify(optional)]
  pub anti_king: Option<bool>,
  #[tsify(optional)]
  pub kropki_dots: Option<Vec<KropkiDot>>,
  #[tsify(optional)]
  pub kropki_negative: Option<bool>,
  #[tsify(optional)]
  pub odd_cells: Option<Vec<CellPosition>>,
  #[tsify(optional)]
  pub even_cells: Option<Vec<CellPosition>>,
  #[tsify(optional)]
  pub top_bottom: Option<bool>,
  #[tsify(optional)]
  pub renbans: Option<Vec<Renban>>,
  #[tsify(optional)]
  pub palindromes: Option<Vec<Palindrome>>,
}

#[derive(Debug, Clone)]
pub struct NormalizedSudokuConstraints {
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
  pub renbans: Vec<Renban>,
  pub palindromes: Vec<Palindrome>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Tsify)]
#[tsify(from_wasm_abi)]
pub struct FixedNumber {
  pub position: CellPosition,
  pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Tsify)]
#[tsify(from_wasm_abi)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Tsify, Deref, DerefMut)]
#[tsify(from_wasm_abi)]
pub struct Region(pub Vec<CellPosition>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Tsify, Deref, DerefMut)]
#[tsify(from_wasm_abi)]
pub struct Grid(pub Vec<Vec<u32>>);

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(from_wasm_abi)]
pub struct KillerCage {
  pub sum: Option<u32>,
  pub region: Region,
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct KropkiDot {
  pub dot_type: KropkiDotType,
  pub cell_1: CellPosition,
  pub cell_2: CellPosition,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Tsify)]
#[tsify(from_wasm_abi)]
pub enum KropkiDotType {
  Consecutive,
  Double,
  Negative,
}

#[derive(Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SudokuLogicalSolveResult {
  pub solution_type: SolutionType,
  #[tsify(optional)]
  pub solution: Option<Grid>,
  pub steps: Vec<SolutionStep>,
  #[tsify(optional)]
  pub invalid_state_reason: Option<InvalidStateReason>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Tsify)]
#[tsify(into_wasm_abi)]
pub enum SolutionType {
  Full,
  Partial,
  None,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct InvalidStateReason {
  pub state_type: InvalidStateType,
  pub area: Area,
  pub values: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Tsify)]
#[tsify(into_wasm_abi)]
pub enum InvalidStateType {
  CellNoCandidates,
  CellEmpty,
  CellInvalidValue,
  AreaValueConflict,
  AreaCandidates,
  AreaConstraint,
}

#[derive(Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SudokuBruteSolveResult {
  pub solution_count: u32,
  #[tsify(optional)]
  pub solution: Option<Grid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SolutionStep {
  pub rule: Rule,
  // Used for grid steps (the first cell should have <value>) or e.g. Rule::XWing to
  // show which cells build the X shape
  pub cells: Vec<CellPosition>,
  // The meaning can vary between rules
  pub values: Vec<u32>,
  // Used in rules that are applies inside an area. Could be multiple e.g. Rule::XWing
  pub areas: Vec<Area>,
  // Used for non-grid steps. The meaning can vary between rules,
  // but we will remove candidates from these cells.
  pub affected_cells: Vec<CellPosition>,
  #[tsify(optional)]
  pub grid: Option<Grid>,
  #[tsify(optional)]
  pub candidates: Option<Vec<Vec<Vec<u32>>>>,
  // Used for Rule::NishioForcingChains
  #[tsify(optional)]
  pub invalid_state_reason: Option<InvalidStateReason>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq, Hash, Tsify)]
#[tsify(into_wasm_abi)]
pub enum Rule {
  // Easy
  NakedSingle, // 1 Cell Position, 1 value + who it is constrained by
  HiddenSingle, // 1 Cell Position, 1 value, the row/col/region + who it is constrained by
  Thermo,
  Kropki,
  Candidates,
  ThermoCandidates,
  KillerCandidates,
  ArrowCandidates,
  RenbanCandidates,
  PalindromeValues,
  PalindromeCandidates,
  // Medium
  ArrowAdvancedCandidates,
  Killer45,
  KropkiChainCandidates,
  KropkiAdvancedCandidates,
  TopBottomCandidates,
  LockedCandidatesPairs, // 2 CellPositions + what they affect
  NakedPairs, // 2 Cell Positions, 2 values + what they affect
  HiddenPairs,
  // Hard
  CommonPeerElimination, // cells = have common peers, affected_cells = would eliminate them
  CommonPeerEliminationKropki,
  CommonPeerEliminationArrow,
  LockedCandidatesTriples,
  NakedTriples, // 2 Cell Positions, 2 values + what they affect
  HiddenTriples,
  XWing,
  XYWing,
  Swordfish, // ???
  TurbotFish,
  EmptyRectangles,
  AdhocNakedSet,
  PhistomefelRing,
  NishioForcingChains,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Tsify)]
#[tsify(into_wasm_abi)]
pub enum Area {
  Grid,
  Adhoc(Vec<CellPosition>),
  Cell(usize, usize),
  Row(usize),
  Column(usize),
  Region(usize),
  Thermo(usize),
  Arrow(usize),
  KillerCage(usize),
  KropkiDot(usize),
  PrimaryDiagonal,
  SecondaryDiagonal,
  Renban(usize),
  Palindrome(usize),
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify, Deref)]
#[tsify(from_wasm_abi)]
pub struct Thermo(pub Vec<CellPosition>);

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Arrow {
  pub circle_cells: Vec<CellPosition>,
  pub arrow_cells: Vec<CellPosition>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify, Deref)]
#[tsify(from_wasm_abi)]
pub struct Renban(pub Vec<CellPosition>);

#[derive(Serialize, Deserialize, Debug, Clone, Tsify, Deref)]
#[tsify(from_wasm_abi)]
pub struct Palindrome(pub Vec<CellPosition>);

#[derive(Debug)]
pub enum ConstraintError {
  InvalidValue {
    field: &'static str,
    message: &'static str,
  },
}

impl TryFrom<SudokuConstraints> for NormalizedSudokuConstraints {
  type Error = ConstraintError;

  fn try_from(src: SudokuConstraints) -> Result<Self, Self::Error> {
    if ![4, 6, 9].contains(&src.grid_size) {
      return Err(ConstraintError::InvalidValue { field: "grid_size", message: "Can only be 4, 6 or 9" })
    }

    Ok(NormalizedSudokuConstraints {
      grid_size: src.grid_size,
      fixed_numbers: src.fixed_numbers.unwrap_or_default(),
      regions: src.regions.unwrap_or(SudokuConstraints::default_regions(src.grid_size)),
      extra_regions: src.extra_regions.unwrap_or_default(),
      killer_cages: src.killer_cages.unwrap_or_default(),
      thermos: src.thermos.unwrap_or_default(),
      arrows: src.arrows.unwrap_or_default(),
      primary_diagonal: src.primary_diagonal.unwrap_or_default(),
      secondary_diagonal: src.secondary_diagonal.unwrap_or_default(),
      anti_knight: src.anti_knight.unwrap_or_default(),
      anti_king: src.anti_king.unwrap_or_default(),
      kropki_dots: src.kropki_dots.unwrap_or_default(),
      kropki_negative: src.kropki_negative.unwrap_or_default(),
      odd_cells: src.odd_cells.unwrap_or_default(),
      even_cells: src.even_cells.unwrap_or_default(),
      top_bottom: src.top_bottom.unwrap_or_default(),
      renbans: src.renbans.unwrap_or_default(),
      palindromes: src.palindromes.unwrap_or_default(),
    })
  }
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
  pub fn new(grid_size: usize) -> SudokuConstraints {
    SudokuConstraints {
      grid_size,
      fixed_numbers: None,
      regions: None,
      extra_regions: None,
      killer_cages: None,
      thermos: None,
      arrows: None,
      primary_diagonal: None,
      secondary_diagonal: None,
      anti_knight: None,
      anti_king: None,
      kropki_dots: None,
      kropki_negative: None,
      odd_cells: None,
      even_cells: None,
      top_bottom: None,
      renbans: None,
      palindromes: None,
    }
  }

  #[allow(dead_code)]
  pub fn default_regions(grid_size: usize) -> Vec<Region> {
    let (region_height, region_width) = SudokuConstraints::compute_region_sizes(grid_size);

    let mut regions: Vec<Region> = vec![];
    for region_row_index in 0..(grid_size / region_height) {
      for region_col_index in 0..(grid_size / region_width) {
        let mut region = Region(vec![]);
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
  pub fn with_fixed_numbers(mut self, fixed_numbers: Vec<FixedNumber>) -> Self {
    self.fixed_numbers = Some(fixed_numbers);
    self
  }

  #[cfg(test)]
  pub fn with_fixed_numbers_grid(mut self, fixed_numbers_grid: Grid) -> Self {
    self.fixed_numbers = Some(fixed_numbers_grid.to_fixed_numbers());
    self
  }

  #[cfg(test)]
  pub fn with_regions(mut self, regions: Vec<Region>) -> Self {
    self.regions = Some(regions);
    self
  }

  #[cfg(test)]
  pub fn with_extra_regions(mut self, extra_regions: Vec<Region>) -> Self {
    self.extra_regions = Some(extra_regions);
    self
  }

  #[cfg(test)]
  pub fn with_killer_cages(mut self, killer_cages: Vec<KillerCage>) -> Self {
    self.killer_cages = Some(killer_cages);
    self
  }

  #[cfg(test)]
  pub fn with_kropki_dots(mut self, kropki_dots: Vec<KropkiDot>) -> Self {
    self.kropki_dots = Some(kropki_dots);
    self
  }

  #[cfg(test)]
  pub fn with_kropki_negative(mut self) -> Self {
    self.kropki_negative = Some(true);
    self
  }

  #[cfg(test)]
  pub fn with_odd_cells(mut self, odd_cells: Vec<CellPosition>) -> Self {
    self.odd_cells = Some(odd_cells);
    self
  }

  #[cfg(test)]
  pub fn with_even_cells(mut self, even_cells: Vec<CellPosition>) -> Self {
    self.even_cells = Some(even_cells);
    self
  }

  #[cfg(test)]
  pub fn with_palindromes(mut self, palindromes: Vec<Palindrome>) -> Self {
    self.palindromes = Some(palindromes);
    self
  }

  #[cfg(test)]
  pub fn with_renbans(mut self, renbans: Vec<Renban>) -> Self {
    self.renbans = Some(renbans);
    self
  }

  #[cfg(test)]
  pub fn with_top_bottom(mut self) -> Self {
    self.top_bottom = Some(true);
    self
  }

  #[cfg(test)]
  pub fn with_anti_king(mut self) -> Self {
    self.anti_king = Some(true);
    self
  }

  #[cfg(test)]
  pub fn with_anti_knight(mut self) -> Self {
    self.anti_knight = Some(true);
    self
  }

  #[cfg(test)]
  pub fn with_primary_diagonal(mut self) -> Self {
    self.primary_diagonal = Some(true);
    self
  }

  #[cfg(test)]
  pub fn with_secondary_diagonal(mut self) -> Self {
    self.secondary_diagonal = Some(true);
    self
  }

  #[cfg(test)]
  pub fn with_diagonals(self) -> Self {
    self.with_primary_diagonal().with_secondary_diagonal()
  }

  #[cfg(test)]
  pub fn with_thermos(mut self, thermos: Vec<Thermo>) -> Self {
    self.thermos = Some(thermos);
    self
  }

  #[cfg(test)]
  pub fn with_arrows(mut self, arrows: Vec<Arrow>) -> Self {
    self.arrows = Some(arrows);
    self
  }

  pub fn to_lz_string(&self) -> String {
    let json = serde_json::to_string(&self).unwrap();
    let lz_string = lz_str::compress_to_base64(&json);
    lz_string
  }

  pub fn to_grid_string(&self) -> String {
    Grid::from_fixed_numbers(self.grid_size, &self.fixed_numbers).to_string(Some("\n"))
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

impl SudokuLogicalSolveResult {
  pub fn no_solution(invalid_state_reason: InvalidStateReason, steps: Vec<SolutionStep>) -> SudokuLogicalSolveResult {
    SudokuLogicalSolveResult {
      solution_type: SolutionType::None,
      solution: None,
      steps: steps,
      invalid_state_reason: Some(invalid_state_reason),
    }
  }
}

impl SolutionStep {
  pub fn new(
    rule: Rule, cells: Vec<CellPosition>, values: Vec<u32>,
    areas: Vec<Area>, affected_cells: Vec<CellPosition>,
  ) -> SolutionStep {
    SolutionStep {
      rule,
      cells,
      values,
      areas,
      affected_cells,
      grid: None,
      candidates: None,
      invalid_state_reason: None,
    }
  }

  pub fn is_grid_step(&self) -> bool {
    [ Rule::NakedSingle, Rule::HiddenSingle, Rule::Thermo, Rule::PalindromeValues ].contains(&self.rule)
  }
}

impl Grid {
  pub fn from_string(grid_str: String) -> Self {
    let grid_size = f32::sqrt(grid_str.len() as f32) as usize;
    assert_eq!(grid_size * grid_size, grid_str.len(), "Invalid grid passed");
    let grid_chars = grid_str.chars().collect_vec();
    let grid: Vec<Vec<u32>> = (0..grid_size).map(|row| {
      (0..grid_size).map(|col| {
        let index = row * grid_size + col;
        grid_chars[index].to_digit(10).unwrap()
      }).collect()
    }).collect();
    Self(grid)
  }

  pub fn from_fixed_numbers(grid_size: usize, fixed_numbers: &Option<Vec<FixedNumber>>) -> Self {
    let mut grid = vec![ vec![ 0; grid_size ]; grid_size ];
    for fixed_number in fixed_numbers.as_ref().unwrap_or(&vec![]) {
      let cell = fixed_number.position;
      grid[cell.row][cell.col] = fixed_number.value;
    }
    Grid(grid)
  }

  pub fn to_fixed_numbers(&self) -> Vec<FixedNumber> {
    let mut fixed_numbers = vec![];
    for (row_index, row) in self.0.iter().enumerate() {
      for (col_index, &value) in row.iter().enumerate() {
        if value != 0 {
          fixed_numbers.push(FixedNumber::new(row_index, col_index, value));
        }
      }
    }
    fixed_numbers
  }

  pub fn to_string(&self, separator: Option<&str>) -> String {
    self.0
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
      Area::Grid | Area::Adhoc(_) | Area::Cell(_, _) |
        Area::Thermo(_) | Area::Arrow(_) |
        Area::KillerCage(_) | Area::KropkiDot(_) |
        Area::PrimaryDiagonal | Area::SecondaryDiagonal |
        Area::Renban(_) | Area::Palindrome(_) => unimplemented!(),
    }
  }
}

// https://www.reddit.com/r/sudoku/comments/11kpwbt/fun_puzzle_link_in_comment/
#[test]
fn check_sudoku_constraints_import_string() {
  let fixed_numbers = Grid(vec![
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
  let constraints = SudokuConstraints::new(9).with_fixed_numbers(fixed_numbers);
  assert_eq!(constraints.to_import_string(), String::from("\
    203000107\
    910040026\
    000000000\
    308010709\
    100000002\
    002000800\
    004167200\
    700080001\
    800209003\
  "));
}

#[test]
fn check_sudoku_grid_from_string() {
  let grid = Grid::from_string(String::from("\
    203000107\
    910040026\
    000000000\
    308010709\
    100000002\
    002000800\
    004167200\
    700080001\
    800209003\
  "));
  let expected_grid = Grid(vec![
    vec![ 2, 0, 3, 0, 0, 0, 1, 0, 7 ],
    vec![ 9, 1, 0, 0, 4, 0, 0, 2, 6 ],
    vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 3, 0, 8, 0, 1, 0, 7, 0, 9 ],
    vec![ 1, 0, 0, 0, 0, 0, 0, 0, 2 ],
    vec![ 0, 0, 2, 0, 0, 0, 8, 0, 0 ],
    vec![ 0, 0, 4, 1, 6, 7, 2, 0, 0 ],
    vec![ 7, 0, 0, 0, 8, 0, 0, 0, 1 ],
    vec![ 8, 0, 0, 2, 0, 9, 0, 0, 3 ],
  ]);
  assert_eq!(grid, expected_grid);
}

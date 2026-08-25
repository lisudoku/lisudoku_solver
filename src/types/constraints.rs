use std::fmt;

use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Tsify)]
#[tsify(from_wasm_abi)]
pub struct FixedNumber {
  pub position: CellPosition,
  pub value: u32,
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

  pub fn all_cells_in_grid_size(grid_size: usize) -> Vec<CellPosition> {
    (0..grid_size).flat_map(|row| {
      (0..grid_size).map(|col| {
        CellPosition::new(row, col)
      }).collect::<Vec<CellPosition>>()
    }).collect()
  }

  const ADJACENT_MOVES: [CellDirection; 4] = [
    CellDirection { row: 0, col: 1 },
    CellDirection { row: 0, col: -1 },
    CellDirection { row: 1, col: 0 },
    CellDirection { row: -1, col: 0 },
  ];

  /// Returns current cell's adjacent cells in 4 directions
  pub fn otrhogonal_neighbours(&self, grid_size: usize) -> Vec<CellPosition> {
    Self::ADJACENT_MOVES.iter().filter_map(|direction| {
      let prow = self.row as isize + direction.row;
      let pcol = self.col as isize + direction.col;
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
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellDirection {
  pub row: isize,
  pub col: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify, Deref, DerefMut)]
#[tsify(from_wasm_abi)]
pub struct Region(pub Vec<CellPosition>);

#[derive(Serialize, Deserialize/*, Debug */, Clone, PartialEq, Tsify, Deref, DerefMut)]
#[tsify(from_wasm_abi)]
pub struct Grid(pub Vec<Vec<u32>>);

impl Grid {
  pub fn from_string(grid_str_input: &str) -> Self {
    // Remove all whitespace so the function is easier to use
    let grid_str: String = grid_str_input.chars().filter(|c| !c.is_whitespace()).collect();
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

impl fmt::Debug for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "Grid:\n{}",
      self.to_string(Some("\n")),
    ))
  }
}

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Tsify, Copy)]
#[tsify(from_wasm_abi)]
pub enum KropkiDotType {
  Consecutive,
  Double,
  Negative,
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

impl Arrow {
  pub fn all_cells(&self) -> Vec<CellPosition> {
    [
      self.arrow_cells.to_vec(),
      self.circle_cells.iter().sorted().copied().collect()
    ].concat()
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify, Deref)]
#[tsify(from_wasm_abi)]
pub struct Renban(pub Vec<CellPosition>);

#[derive(Serialize, Deserialize, Debug, Clone, Tsify, Deref)]
#[tsify(from_wasm_abi)]
pub struct Palindrome(pub Vec<CellPosition>);

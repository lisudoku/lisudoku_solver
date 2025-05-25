use crate::solver::logical_solver::arrow_advanced_candidates::ArrowAdvancedCandidates;
use crate::solver::logical_solver::common_peer_elimination_arrow::CommonPeerEliminationArrow;
use crate::solver::logical_solver::kropki_advanced_candidates::KropkiAdvancedCandidates;
use crate::solver::logical_solver::nishio_forcing_chains::NishioForcingChains;
use crate::solver::logical_solver::renban_candidates::RenbanCandidates;
use crate::types::{Area, Arrow, CellDirection, CellPosition, Grid, KillerCage, KropkiDot, KropkiDotType, Rule, SudokuConstraints, SudokuGrid};
use std::cell::RefCell;
use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};
use std::ops::BitAnd;
use std::rc::Rc;
use itertools::Itertools;
use logical_solver::adhoc_naked_set::AdhocNakedSet;
use logical_solver::palindrome_candidates::PalindromeCandidates;
use logical_solver::palindrome_values::PalindromeValues;
use self::logical_solver::advanced_candidates::CellEliminationsResult;
use self::logical_solver::arrow_candidates::ArrowCombinationLogicFactory;
use self::logical_solver::candidates::Candidates;
use self::logical_solver::combinations::cell_combination_logic::CellsCacheKey;
use self::logical_solver::common_peer_elimination::CommonPeerElimination;
use self::logical_solver::common_peer_elimination_kropki::CommonPeerEliminationKropki;
use self::logical_solver::hidden_set::HiddenSet;
use self::logical_solver::hidden_singles::HiddenSingles;
use self::logical_solver::killer45::Killer45;
use self::logical_solver::killer_candidates::KillerCandidates;
use self::logical_solver::kropki_chain_candidates::KropkiChainCandidates;
use self::logical_solver::locked_candidates::LockedCandidates;
use self::logical_solver::naked_set::NakedSet;
use self::logical_solver::naked_singles::NakedSingle;
use self::logical_solver::technique::Technique;
use self::logical_solver::thermo_candidates::ThermoCandidates;
use self::logical_solver::thermo_steps::Thermo;
use self::logical_solver::top_bottom_candidates::TopBottomCandidates;
use self::logical_solver::x_wing::XWing;
use self::logical_solver::xy_wing::XYWing;
use self::logical_solver::turbot_fish::TurbotFish;
use self::logical_solver::empty_reclanges::EmptyRectangles;
use crate::solver::logical_solver::arrow_candidates::ArrowCandidates;

mod checker;
pub mod logical_solver;
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

const KING_MOVES: [CellDirection; 8] = [
  CellDirection { row: -1, col: -1 },
  CellDirection { row: -1, col: 0 },
  CellDirection { row: -1, col: 1 },
  CellDirection { row: 0, col: -1 },
  CellDirection { row: 0, col: 1 },
  CellDirection { row: 1, col: -1 },
  CellDirection { row: 1, col: 0 },
  CellDirection { row: 1, col: 1 },
];

const ADJACENT_MOVES: [CellDirection; 4] = [
  CellDirection { row: 0, col: 1 },
  CellDirection { row: 0, col: -1 },
  CellDirection { row: 1, col: 0 },
  CellDirection { row: -1, col: 0 },
];

pub struct Solver {
  pub constraints: SudokuConstraints,
  pub techniques: Vec<Rc<dyn Technique>>,
  pub grid: Grid,
  pub solution: Option<Grid>,
  grid_to_regions: Vec<Vec<Vec<usize>>>,
  grid_to_thermos: Vec<Vec<Vec<usize>>>,
  grid_to_killer_cage: Vec<Vec<usize>>,
  grid_to_kropki_dots: Vec<Vec<Vec<usize>>>,
  grid_to_odd_cells: Vec<Vec<bool>>,
  grid_to_even_cells: Vec<Vec<bool>>,
  grid_to_renbans: Vec<Vec<Vec<usize>>>,
  candidates_active: bool,
  candidates: Vec<Vec<HashSet<u32>>>,
  hint_mode: bool,
  step_count_limit: Option<usize>,
  arrow_combinatons_logic_factory: RefCell<ArrowCombinationLogicFactory>,
  cell_eliminations_cache: RefCell<HashMap<CellsCacheKey, CellEliminationsResult>>,
}

impl Clone for Solver {
  fn clone(&self) -> Self {
    Self {
      constraints: self.constraints.clone(),
      techniques: self.techniques.clone(),
      grid: self.grid.clone(),
      solution: self.solution.clone(),
      grid_to_regions: self.grid_to_regions.clone(),
      grid_to_thermos: self.grid_to_thermos.clone(),
      grid_to_killer_cage: self.grid_to_killer_cage.clone(),
      grid_to_kropki_dots: self.grid_to_kropki_dots.clone(),
      grid_to_odd_cells: self.grid_to_odd_cells.clone(),
      grid_to_even_cells: self.grid_to_even_cells.clone(),
      grid_to_renbans: self.grid_to_renbans.clone(),
      candidates_active: self.candidates_active.clone(),
      candidates: self.candidates.clone(),
      hint_mode: self.hint_mode.clone(),
      step_count_limit: self.step_count_limit.clone(),
      arrow_combinatons_logic_factory: RefCell::new(ArrowCombinationLogicFactory::new()),
      cell_eliminations_cache: self.cell_eliminations_cache.clone(),
    }
  }
}

impl Solver {
  pub fn new(mut constraints: SudokuConstraints, input_grid: Option<SudokuGrid>) -> Solver {
    // Assume all extra regions contain grid_size cells
    constraints.regions.extend(constraints.extra_regions.to_vec());

    let mut grid_to_regions = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, region) in constraints.regions.iter().enumerate() {
      for cell in region {
        grid_to_regions[cell.row][cell.col].push(index);
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
          for negative_cell in negative_cells.into_iter().sorted() {
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

    let mut grid_to_odd_cells = vec![ vec![ false; constraints.grid_size ]; constraints.grid_size ];
    for cell in &constraints.odd_cells {
      grid_to_odd_cells[cell.row][cell.col] = true;
    }

    let mut grid_to_even_cells = vec![ vec![ false; constraints.grid_size ]; constraints.grid_size ];
    for cell in &constraints.even_cells {
      grid_to_even_cells[cell.row][cell.col] = true;
    }

    let mut grid_to_renbans = vec![ vec![ vec![]; constraints.grid_size ]; constraints.grid_size ];
    for (index, renban) in constraints.renbans.iter().enumerate() {
      for cell in renban {
        grid_to_renbans[cell.row][cell.col].push(index);
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
      grid_to_regions,
      grid_to_thermos,
      grid_to_killer_cage,
      grid_to_kropki_dots,
      grid_to_odd_cells,
      grid_to_even_cells,
      grid_to_renbans,
      candidates_active: false,
      candidates,
      hint_mode: false,
      step_count_limit: None,
      techniques: Self::default_techniques(),
      arrow_combinatons_logic_factory: RefCell::new(ArrowCombinationLogicFactory::new()),
      cell_eliminations_cache: RefCell::new(HashMap::new()),
    }
  }

  pub fn with_hint_mode(mut self, flag: bool) -> Self {
    self.hint_mode = flag;
    self
  }

  pub fn with_step_count_limit(mut self, step_count_limit: usize) -> Self {
    self.step_count_limit = Some(step_count_limit);
    self
  }

  pub fn default_techniques() -> Vec<Rc<dyn Technique>> {
    vec![
      Rc::new(ThermoCandidates),
      Rc::new(KillerCandidates),
      Rc::new(KropkiChainCandidates::new(false)),
      Rc::new(KropkiChainCandidates::new(true)),
      Rc::new(TopBottomCandidates::new(false)),
      Rc::new(ArrowCandidates),
      Rc::new(RenbanCandidates),
      Rc::new(PalindromeValues),
      Rc::new(PalindromeCandidates),
      Rc::new(NakedSingle),
      Rc::new(HiddenSingles),
      Rc::new(Thermo),
      Rc::new(Candidates),
      Rc::new(Killer45),
      Rc::new(LockedCandidates::new(2)),
      Rc::new(NakedSet::new(2)),
      Rc::new(HiddenSet::new(2)),
      Rc::new(LockedCandidates::new(3)),
      Rc::new(NakedSet::new(3)),
      Rc::new(HiddenSet::new(3)),
      Rc::new(XWing),
      Rc::new(XYWing),
      Rc::new(CommonPeerElimination),
      Rc::new(CommonPeerEliminationKropki),
      Rc::new(KropkiAdvancedCandidates),
      Rc::new(ArrowAdvancedCandidates),
      Rc::new(CommonPeerEliminationArrow),
      Rc::new(AdhocNakedSet),
      Rc::new(TurbotFish),
      Rc::new(EmptyRectangles),
      // Rc::new(PhistomefelRing), // disabled for now...
      Rc::new(NishioForcingChains),
    ]
  }

  pub fn with_techniques(mut self, techniques: Vec<Rc<dyn Technique>>) -> Self {
    self.techniques = techniques;
    self
  }

  pub fn without_techniques(mut self, techniques: Vec<Rc<dyn Technique>>) -> Self {
    let rules: Vec<Rule> = techniques.into_iter().map(|t| t.get_rule()).collect();
    self.techniques = self.techniques.into_iter().filter(|t| !rules.contains(&t.get_rule())).collect();
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
        &Area::Adhoc(_) | &Area::Row(_) | &Area::Column(_) | &Area::Region(_) | &Area::Renban(_) |
        &Area::PrimaryDiagonal | &Area::SecondaryDiagonal
      ) => self.compute_generic_area_cell_candidates(area),
      &Area::Thermo(thermo_index) => self.compute_thermo_cell_candidates(thermo_index, cell),
      &Area::KillerCage(killer_cage_index) => self.compute_killer_cell_candidates(killer_cage_index),
      &Area::KropkiDot(_) => {
        // Do not enforce candidates directly, use an explicit rule for that
        self.compute_all_candidates()
      },
      &Area::Grid | &Area::Cell(_, _) | &Area::Arrow(_) | &Area::Palindrome(_) => unimplemented!(),
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
      for value in (sum+1)..=(self.constraints.grid_size as u32) {
        set.remove(&value);
      }
    }

    set
  }

  // This could be made more intelligent, but we leave the tricks to logical_solver
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

    let set: HashSet<u32> = (max_before+1..=min_after-1).collect();

    set
  }

  fn compute_cell_candidates(&self, cell: &CellPosition) -> HashSet<u32> {
    if self.grid[cell.row][cell.col] != 0 {
      return HashSet::new()
    }

    if self.candidates_active {
      return self.candidates[cell.row][cell.col].clone();
    }

    self.recompute_cell_candidates(cell)
  }

  // Note: update when adding constraints
  // We don't apply all restrictions at this level (e.g. thermo, palindrome)
  fn recompute_cell_candidates(&self, cell: &CellPosition) -> HashSet<u32> {
    let mut candidates = self.compute_all_candidates();
    for peer in self.get_cell_peers(cell, false) {
      let value = self.grid[peer.row][peer.col];
        if value == 0 {
          continue
        }
        candidates.remove(&value);
    }

    for area in &self.get_cell_areas(cell, false) {
      let area_set = self.compute_area_cell_candidates(area, cell);
      candidates = candidates.intersection(&area_set).cloned().collect();
    }

    if self.grid_to_odd_cells[cell.row][cell.col] {
      candidates = candidates.into_iter().filter(|value| value % 2 == 1).collect();
    }
    if self.grid_to_even_cells[cell.row][cell.col] {
      candidates = candidates.into_iter().filter(|value| value % 2 == 0).collect();
    }

    candidates
  }

  fn compute_all_candidates(&self) -> HashSet<u32> {
    (1..=self.constraints.grid_size as u32).collect::<HashSet<u32>>()
  }

  // Note: update when adding new areas
  // Note: we're mostly interested in areas with uniqueness constraints
  fn get_cell_areas(&self, cell: &CellPosition, include_thermo: bool) -> Vec<Area> {
    let mut areas: Vec<Area> = vec![];

    areas.extend(self.get_cell_classic_areas(cell));
    areas.extend(self.get_cell_special_areas(cell, include_thermo));

    areas
  }

  fn get_cell_classic_areas(&self, cell: &CellPosition) -> Vec<Area> {
    let &CellPosition { row, col } = cell;
    let mut areas = vec![ Area::Row(row), Area::Column(col) ];

    for &region_index in &self.grid_to_regions[row][col] {
      if region_index < self.constraints.grid_size {
        areas.push(Area::Region(region_index));
      }
    }

    areas
  }

  fn get_cell_special_areas(&self, cell: &CellPosition, include_thermo: bool) -> Vec<Area> {
    let &CellPosition { row, col } = cell;
    let mut areas: Vec<Area> = vec![];

    for &region_index in &self.grid_to_regions[row][col] {
      if region_index >= self.constraints.grid_size {
        areas.push(Area::Region(region_index));
      }
    }

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
    for &renban_index in &self.grid_to_renbans[row][col] {
      areas.push(Area::Renban(renban_index));
    }

    areas
  }

  // Note: update when adding new areas
  // Note: a lot of the time we don't want area that don't need all <grid_size> and unique values
  fn get_all_areas(
    &self, include_thermo: bool, include_killer: bool, include_kropki: bool, include_renban: bool,
    include_palindrome: bool,
  ) -> Vec<Area> {
    let mut areas = vec![];
    areas.extend(self.get_row_areas());
    areas.extend(self.get_col_areas());
    areas.extend(self.get_region_areas());
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
    if include_renban {
      for renban_index in 0..self.constraints.renbans.len() {
        areas.push(Area::Renban(renban_index));
      }
    }
    if include_palindrome {
      for palindrome_index in 0..self.constraints.palindromes.len() {
        areas.push(Area::Palindrome(palindrome_index));
      }
    }

    areas
  }

  fn get_all_proper_areas(&self) -> Vec<Area> {
    self.get_all_areas(false, false, false, false, false)
  }

  fn get_row_areas(&self) -> Vec<Area> {
    (0..self.constraints.grid_size).map(|row| Area::Row(row)).collect()
  }

  fn get_col_areas(&self) -> Vec<Area> {
    (0..self.constraints.grid_size).map(|col| Area::Column(col)).collect()
  }

  fn get_region_areas(&self) -> Vec<Area> {
    (0..self.constraints.regions.len()).map(|region_index| {
      Area::Region(region_index)
    }).collect()
  }

  fn get_area_cells(&self, area: &Area) -> Vec<CellPosition> {
    match area {
      &Area::Grid => self.get_grid_cells(),
      Area::Adhoc(cells) => cells.to_vec(),
      &Area::Cell(row, col) => vec![CellPosition::new(row, col)],
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
      &Area::Renban(renban_index) => self.constraints.renbans[renban_index].to_vec(),
      &Area::Palindrome(palindrome_index) => self.constraints.palindromes[palindrome_index].to_vec(),
      &Area::Arrow(_) => unimplemented!(),
    }
  }

  fn get_area_values(&self, area: &Area) -> Vec<u32> {
    self.get_area_cells(&area).into_iter().map(|cell| {
      let value = self.grid[cell.row][cell.col];
      value
    }).collect()
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
      for value in candidates[cell.row][cell.col].iter().sorted() {
        let entry = value_cells.entry(*value).or_insert(vec![]);
        entry.push(cell);
      }
    }
    value_cells
  }

  fn get_cell_peers_with_candidate(&self, cell: &CellPosition, value: u32) -> Vec<CellPosition> {
    let candidates: HashSet<u32> = HashSet::from([ value ]);
    self.get_cell_peers_with_candidates(cell, &candidates)
  }

  fn get_cell_peers_with_candidates(&self, cell: &CellPosition, values: &HashSet<u32>) -> Vec<CellPosition> {
    let peers = self.get_cell_peers(cell, true);
    self.filter_cells_with_any_candidates(&peers, values)
  }

  // Note: update when adding constraints
  // What it considers as a "peer" is subjective, it doesn't consider
  // all restrictions at this level (e.g. thermo, palindrome)
  fn get_cell_peers(&self, cell: &CellPosition, include_thermo: bool) -> Vec<CellPosition> {
    let mut peers: Vec<CellPosition> = vec![];

    peers.extend(self.get_cell_classic_peers(cell));
    peers.extend(self.get_cell_special_peers(cell, include_thermo));

    peers.into_iter()
         .filter(|other_cell| other_cell != cell)
         .unique()
         .collect()
  }

  fn get_cell_classic_peers(&self, cell: &CellPosition) -> Vec<CellPosition> {
    self.get_cell_classic_areas(cell)
      .iter()
      .flat_map(|area| self.get_area_cells(area))
      .collect()
  }

  fn get_cell_special_peers(&self, cell: &CellPosition, include_thermo: bool) -> Vec<CellPosition> {
    let mut peers: Vec<CellPosition> = self.get_cell_special_areas(cell, include_thermo)
      .iter()
      .flat_map(|area| self.get_area_cells(area))
      .collect();

    if self.constraints.anti_knight {
      peers.extend(self.get_knight_peers(cell));
    }

    if self.constraints.anti_king {
      peers.extend(self.get_king_peers(cell));
    }

    peers
  }

  // Returns peers that are special and are not peers through classical constraints
  fn get_cell_only_special_peers(&self, cell: &CellPosition, include_thermo: bool) -> Vec<CellPosition> {
    let special_peers = self.get_cell_special_peers(cell, include_thermo);
    let classic_peers = self.get_cell_classic_peers(cell);

    special_peers.into_iter().filter(|special_peer|
      !classic_peers.contains(special_peer)
    ).collect()
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

  fn get_king_peers(&self, cell: &CellPosition) -> Vec<CellPosition> {
    KING_MOVES.iter().filter_map(|direction| {
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
    (1..=self.constraints.grid_size).filter_map(|value| {
      if combination_mask.bitand(1 << value) > 0 {
        Some(value as u32)
      } else {
        None
      }
    }).collect()
  }

  fn arrow_circle_number(&self, arrow: &Arrow) -> (u32, bool) {
    let mut value: u32 = 0;
    let sorted_circle_cells: Vec<CellPosition> = arrow.circle_cells
      .iter()
      .cloned()
      .sorted_by_key(|cell| *cell)
      .collect();
    let mut full = true;
    for &CellPosition { row, col } in &sorted_circle_cells {
      value = 10 * value + self.grid[row][col];
      if self.grid[row][col] == 0 {
        full = false;
      }
    }
    (value, full)
  }

  fn arrow_arrow_sum(&self, arrow: &Arrow) -> (u32, bool) {
    let mut sum: u32 = 0;
    let mut full = true;
    for &CellPosition { row, col } in &arrow.arrow_cells {
      sum += self.grid[row][col];
      if self.grid[row][col] == 0 {
        full = false;
      }
    }
    (sum, full)
  }

  fn count_empty_cells_in_list(&self, cells: &Vec<CellPosition>) -> usize {
    cells.into_iter().filter(|cell| self.grid[cell.row][cell.col] == 0).count()
  }
}

#[cfg(test)]
mod tests;

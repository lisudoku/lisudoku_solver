use crate::{solver::{constraints::{CandidateMode, Constraint, ConstraintDefinition, ConstraintDefinitionExt, PeerMode, arrow::ArrowConstraint, context::ConstraintCellPeersContext, killer::KillerConstraint, kind::ConstraintKind, kropki::KropkiConstraint, palindrome::PalindromeConstraint, regions::RegionsConstraint, renban::RenbanConstraint, thermo::ThermoConstraint, top_bottom::TopBottomConstraint}, types::area_id_mapper::AreaIdMapper}, types::{Area, CellPosition, ConstraintError, SudokuConstraints}};
use std::{cell::{OnceCell, RefCell}, collections::{HashMap, HashSet}};
use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};

/// Validated constraint set derived from SudokuConstraints, without grid state.
#[derive(Clone, Default)]
pub(crate) struct Puzzle {
  pub(crate) grid_size: usize,
  pub(crate) constraints: Vec<Constraint>,
  pub(crate) constraints_by_kind: [Option<usize>; ConstraintKind::COUNT],
  pub(crate) cell_area_ids: Vec<Vec<usize>>,
  area_id_mapper: RefCell<AreaIdMapper>,
  common_region_areas_cache: RefCell<HashMap<Vec<CellPosition>, Vec<Area>>>,
  all_region_areas_cache: OnceCell<Vec<Area>>,
  all_house_areas_cache: OnceCell<Vec<Area>>,
  all_cell_peers_cache: RefCell<HashMap<(CellPosition, PeerMode, CandidateMode), Vec<CellPosition>>>,
  area_cells_cache: RefCell<Vec<Option<Vec<CellPosition>>>>,
  cell_only_special_peers_cache: RefCell<HashMap<CellPosition, Vec<CellPosition>>>,
}

impl TryFrom<SudokuConstraints> for Puzzle {
  type Error = ConstraintError;

  fn try_from(input: SudokuConstraints) -> Result<Self, Self::Error> {
    if ![4, 6, 9].contains(&input.grid_size) {
      return Err(ConstraintError::InvalidValue { field: "grid_size", message: "Can only be 4, 6 or 9" })
    }
    for fixed_number in input.fixed_numbers.iter().flatten() {
      if fixed_number.value  < 1 || fixed_number.value  > input.grid_size as u32 {
        return Err(ConstraintError::InvalidValue {
          field: "fixed_numbers",
          message: "Given digits must be between 1 and grid_size",
        })
      }
    }

    /* TODO: add exhaustive validations:
      * check that arrow circle cells are sorted or sort them
      * check regions count, sizes and overlap
      * etc. same for all other constraints in their builder
    */

    let mut constraints: Vec<Constraint> = vec![];
    let mut constraints_by_kind: [Option<usize>; ConstraintKind::COUNT] = [None; ConstraintKind::COUNT];
    for kind in ConstraintKind::iter() {
      let Some(constraint) = kind.builder().from_input(&input) else { continue };
      constraints_by_kind[kind as usize] = Some(constraints.len());
      constraints.push(constraint);
    }

    let mut puzzle = Puzzle {
      grid_size: input.grid_size,
      constraints,
      constraints_by_kind,
      cell_area_ids: vec![], // temporary
      area_id_mapper: RefCell::new(AreaIdMapper::default()),
      common_region_areas_cache: RefCell::new(HashMap::new()),
      all_region_areas_cache: OnceCell::new(),
      all_house_areas_cache: OnceCell::new(),
      all_cell_peers_cache: RefCell::new(HashMap::new()),
      area_cells_cache: RefCell::new(vec![]),
      cell_only_special_peers_cache: RefCell::new(HashMap::new()),
    };

    puzzle.initialize_derived_fields();

    Ok(puzzle)
  }
}

impl Puzzle {
  pub(crate) fn initialize_derived_fields(&mut self) {
    self.area_cells_cache = RefCell::new(vec![
      None; self.all_region_areas().count()
    ]);

    let mut cell_area_ids: Vec<Vec<usize>> = vec![vec![]; self.grid_size * self.grid_size];
    for area in self.all_region_areas() {
      let area_id = self.area_to_id(&area).expect("region areas should have valid area ids");
      for cell in self.area_cells(&area) {
        let cell_index = cell.to_index(self.grid_size);
        cell_area_ids[cell_index].push(area_id);
      }
    }
    self.cell_area_ids = cell_area_ids;
  }

  fn area_to_id(&self, area: &Area) -> Option<usize> {
    self.area_id_mapper.borrow_mut().area_to_id(area)
  }

  pub(crate) fn all_region_areas(&self) -> impl Iterator<Item = Area> + '_ {
    self.all_region_areas_cache
      .get_or_init(|| {
        self.constraints
          .iter()
          .flat_map(|constraint|
            constraint.region_areas()
          )
          .collect()
      })
      .iter()
      .cloned()
  }

  pub(crate) fn all_house_areas(&self) -> impl Iterator<Item = Area> + '_ {
    self.all_house_areas_cache
      .get_or_init(|| {
        self.constraints
          .iter()
          .flat_map(|constraint| {
            constraint.house_areas(self.grid_size)
          })
          .collect()
      })
      .iter()
      .cloned()
  }

  /// Returns the list of cells that are peers of the given <cell>
  /// What it considers as a "peer" is subjective, it doesn't consider
  /// all restrictions at this level (e.g. thermo, palindrome)
  pub(crate) fn all_cell_peers(&self, cell: &CellPosition, peer_mode: PeerMode, candidate_mode: CandidateMode) -> Vec<CellPosition> {
    if let Some(cached_result) = self.all_cell_peers_cache.borrow().get(&(*cell, peer_mode, candidate_mode)).cloned() {
      return cached_result
    }

    let mut peers: Vec<CellPosition> = vec![];

    for constraint in &self.constraints {
      let constraint_peers = constraint.cell_peers(
        cell,
        &ConstraintCellPeersContext{ peer_mode, candidate_mode, grid_size: self.grid_size },
      );
      peers.extend(constraint_peers);
    }

    peers = peers
      .into_iter()
      .filter(|other_cell| other_cell != cell)
      .unique()
      .collect();

    self.all_cell_peers_cache.borrow_mut().insert((*cell, peer_mode, candidate_mode), peers.clone());

    peers
  }

  pub(crate) fn find_common_region_areas_except(&self, cells: &Vec<CellPosition>, area_exception: Area) -> Vec<Area> {
    let areas = self.find_common_region_areas(cells);
    let other_areas: Vec<Area> = areas.into_iter().filter(|area| *area != area_exception).collect();
    other_areas
  }

  /// Returns all areas that contain all <cells>.
  /// Useful for finding areas that are affected by all <cells>.
  pub(crate) fn find_common_region_areas(&self, cells: &Vec<CellPosition>) -> Vec<Area> {
    if let Some(cached_result) = self.common_region_areas_cache.borrow().get(cells).cloned() {
      return cached_result
    }

    assert!(cells.len() >= 2);

    let cell1 = cells[0];

    let common_areas: Vec<Area> = self.constraints.iter().flat_map(|constraint| {
      let common_areas: Vec<Area> = constraint.cell_region_areas(&cell1, PeerMode::Full, CandidateMode::Simple);
      let mut common_areas_set: HashSet<Area> = HashSet::from_iter(common_areas);

      for cell in cells[1..].iter() {
        let areas: Vec<Area> = constraint.cell_region_areas(&cell, PeerMode::Full, CandidateMode::Simple);
        let areas_set: HashSet<Area> = HashSet::from_iter(areas);
        common_areas_set = common_areas_set.intersection(&areas_set).cloned().collect();
      }

      common_areas_set.into_iter().sorted().collect::<Vec<_>>()
    }).collect();

    self.common_region_areas_cache.borrow_mut().insert(cells.clone(), common_areas.clone());

    common_areas
  }

  pub(crate) fn cells_are_peers(&self, cell1: &CellPosition, cell2: &CellPosition) -> bool {
    cell1 != cell2 &&
    !self.find_common_region_areas(&vec![*cell1, *cell2]).is_empty()
  }

  pub(crate) fn all_cell_peers_unconstrained(&self, cell: &CellPosition) -> Vec<CellPosition> {
    self.all_cell_peers(cell, PeerMode::Full, CandidateMode::Full)
  }

  /// Returns the <cell> peers that are special and are not peers through classic constraints
  pub(crate) fn cell_only_special_peers(&self, cell: &CellPosition) -> Vec<CellPosition> {
    if let Some(cached_result) = self.cell_only_special_peers_cache.borrow().get(cell).cloned() {
      return cached_result
    }

    let special_peers = self.all_cell_peers(cell, PeerMode::Special, CandidateMode::Full);
    let classic_peers = self.all_cell_peers(cell, PeerMode::Classic, CandidateMode::Full);

    let only_special_peers: Vec<CellPosition> = special_peers
      .into_iter()
      .filter(|special_peer|
        !classic_peers.contains(special_peer)
      )
      .collect();

    self.cell_only_special_peers_cache.borrow_mut().insert(cell.clone(), only_special_peers.clone());

    only_special_peers
  }

  /// Returns the single constraint that owns `area`, if any.
  fn constraint_for_area(&self, area: &Area) -> Option<&Constraint> {
    self.constraints.iter().find(|constraint| match area {
      &Area::Row(_) | &Area::Column(_) | &Area::Region(_) => matches!(constraint, Constraint::Regions(_)),
      &Area::Thermo(_) => matches!(constraint, Constraint::Thermo(_)),
      &Area::Arrow(_) => matches!(constraint, Constraint::Arrow(_)),
      &Area::KillerCage(_) => matches!(constraint, Constraint::Killer(_)),
      &Area::PrimaryDiagonal => matches!(constraint, Constraint::PrimaryDiagonal(_)),
      &Area::SecondaryDiagonal => matches!(constraint, Constraint::SecondaryDiagonal(_)),
      &Area::KropkiDot(_) => matches!(constraint, Constraint::Kropki(_)),
      &Area::Renban(_) => matches!(constraint, Constraint::Renban(_)),
      &Area::Palindrome(_) => matches!(constraint, Constraint::Palindrome(_)),
      &Area::Cell(_, _) | &Area::Adhoc(_) | &Area::Grid => false,
    })
  }

  /// Returns the list of cells in an `area`
  pub(crate) fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    let area_id_opt = self.area_to_id(area);
    if let Some(area_id) = area_id_opt {
      let mut cache = self.area_cells_cache.borrow_mut();
      if area_id >= cache.len() {
        cache.resize(area_id + 1, None);
      } else {
        if let Some(cached_result) = cache[area_id].clone() {
          return cached_result
        }
      }
    }

    let cells = match area {
      Area::Adhoc(cells) => cells.clone(),
      Area::Cell(row, col) => vec![CellPosition::new(*row, *col)],
      Area::Grid => self.all_grid_cells().collect(),

      Area::Row(_)
      | Area::Column(_)
      | Area::Region(_)
      | Area::Thermo(_)
      | Area::Arrow(_)
      | Area::KillerCage(_)
      | Area::PrimaryDiagonal
      | Area::SecondaryDiagonal
      | Area::KropkiDot(_)
      | Area::Renban(_)
      | Area::Palindrome(_) => {
        self
          .constraint_for_area(area)
          .map(|constraint| constraint.area_cells(area))
          .unwrap_or_default()
      },
    };

    if let Some(area_id) = area_id_opt {
      self.area_cells_cache.borrow_mut()[area_id] = Some(cells.clone());
    }

    cells
  }

  pub(crate) fn all_grid_cells(&self) -> impl Iterator<Item = CellPosition> + '_ {
    CellPosition::all_cells_in_grid_size(self.grid_size)
  }

  pub(crate) fn constraint(&self, kind: ConstraintKind) -> Option<&Constraint> {
    self.constraints_by_kind[kind as usize].map(|index| &self.constraints[index])
  }

  pub(crate) fn thermo(&self) -> Option<&ThermoConstraint> {
    let Some(Constraint::Thermo(constraint)) = self.constraint(ConstraintKind::Thermo) else { return None };
    Some(constraint)
  }

  pub(crate) fn kropki(&self) -> Option<&KropkiConstraint> {
    let Some(Constraint::Kropki(constraint)) = self.constraint(ConstraintKind::Kropki) else { return None };
    Some(constraint)
  }

  pub(crate) fn killer(&self) -> Option<&KillerConstraint> {
    let Some(Constraint::Killer(constraint)) = self.constraint(ConstraintKind::Killer) else { return None };
    Some(constraint)
  }

  pub(crate) fn arrow(&self) -> Option<&ArrowConstraint> {
    let Some(Constraint::Arrow(constraint)) = self.constraint(ConstraintKind::Arrow) else { return None };
    Some(constraint)
  }

  pub(crate) fn top_bottom(&self) -> Option<&TopBottomConstraint> {
    let Some(Constraint::TopBottom(constraint)) = self.constraint(ConstraintKind::TopBottom) else { return None };
    Some(constraint)
  }

  pub(crate) fn renban(&self) -> Option<&RenbanConstraint> {
    let Some(Constraint::Renban(constraint)) = self.constraint(ConstraintKind::Renban) else { return None };
    Some(constraint)
  }

  pub(crate) fn palindrome(&self) -> Option<&PalindromeConstraint> {
    let Some(Constraint::Palindrome(constraint)) = self.constraint(ConstraintKind::Palindrome) else { return None };
    Some(constraint)
  }

  pub(crate) fn region(&self) -> Option<&RegionsConstraint> {
    let Some(Constraint::Regions(constraint)) = self.constraint(ConstraintKind::Regions) else { return None };
    Some(constraint)
  }
}

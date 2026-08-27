use crate::{solver::constraints::{CandidateMode, Constraint, ConstraintDefinition, ConstraintDefinitionExt, PeerMode, arrow::ArrowConstraint, context::ConstraintCellPeersContext, fixed_numbers::FixedNumbersConstraint, killer::KillerConstraint, kind::ConstraintKind, kropki::KropkiConstraint, palindrome::PalindromeConstraint, regions::RegionsConstraint, renban::RenbanConstraint, thermo::ThermoConstraint, top_bottom::TopBottomConstraint}, types::{Area, CellPosition, ConstraintError, SudokuConstraints}};
use std::{cell::RefCell, collections::{HashMap, HashSet}};
use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Clone)]
pub(crate) struct PuzzleDefinition {
  pub grid_size: usize,
  pub constraints: Vec<Constraint>,
  pub constraints_by_kind: [Option<usize>; ConstraintKind::COUNT],
  common_region_areas_cache: RefCell<HashMap<Vec<CellPosition>, Vec<Area>>>,
}

impl TryFrom<SudokuConstraints> for PuzzleDefinition {
  type Error = ConstraintError;

  fn try_from(input: SudokuConstraints) -> Result<Self, Self::Error> {
    if ![4, 6, 9].contains(&input.grid_size) {
      return Err(ConstraintError::InvalidValue { field: "grid_size", message: "Can only be 4, 6 or 9" })
    }

    let mut constraints: Vec<Constraint> = vec![];
    let mut constraints_by_kind: [Option<usize>; ConstraintKind::COUNT] = [None; ConstraintKind::COUNT];
    for kind in ConstraintKind::iter() {
      let Some(constraint) = kind.builder().from_input(&input) else { continue };
      constraints_by_kind[kind as usize] = Some(constraints.len());
      constraints.push(constraint);
    }

    Ok(PuzzleDefinition {
      grid_size: input.grid_size,
      constraints,
      constraints_by_kind,
      common_region_areas_cache: RefCell::new(HashMap::new()),
    })
  }
}

impl PuzzleDefinition {
  pub fn all_region_areas(&self) -> Vec<Area> {
    self.constraints.iter().flat_map(|constraint|
      constraint.region_areas()
    ).collect()
  }

  pub fn all_house_areas(&self) -> Vec<Area> {
    self.constraints.iter().flat_map(|constraint| {
      constraint.house_areas(self.grid_size)
    }).collect()
  }

  pub fn all_region_areas_except_kropki(&self) -> Vec<Area> {
    self.constraints.iter().flat_map(|constraint|
      constraint
        .region_areas()
        .into_iter()
        .filter(|area| !matches!(area, Area::KropkiDot(_)))
    ).collect()
  }

  /// Returns the list of cells that are peers of the given <cell>
  /// What it considers as a "peer" is subjective, it doesn't consider
  /// all restrictions at this level (e.g. thermo, palindrome)
  pub fn all_cell_peers(&self, cell: &CellPosition, peer_mode: PeerMode, candidate_mode: CandidateMode) -> Vec<CellPosition> {
    let mut peers: Vec<CellPosition> = vec![];

    for constraint in &self.constraints {
      let constraint_peers = constraint.cell_peers(
        cell,
        &ConstraintCellPeersContext{ peer_mode, candidate_mode, grid_size: self.grid_size },
      );
      peers.extend(constraint_peers);
    }

    peers
      .into_iter()
      .filter(|other_cell| other_cell != cell)
      .unique()
      .collect()
  }

  pub fn find_common_region_areas_except(&self, cells: &Vec<CellPosition>, area_exception: Area) -> Vec<Area> {
    let areas = self.find_common_region_areas(cells);
    let other_areas: Vec<Area> = areas.into_iter().filter(|area| *area != area_exception).collect();
    other_areas
  }

  /// Returns all areas that contain all <cells>.
  /// Useful for finding areas that are affected by all <cells>.
  pub fn find_common_region_areas(&self, cells: &Vec<CellPosition>) -> Vec<Area> {
    if let Some(cached_result) = self.common_region_areas_cache.borrow().get(cells).cloned() {
      return cached_result;
    }

    assert!(cells.len() >= 2);

    let cell1 = cells[0];

    let common_areas: Vec<Area> = self.constraints.iter().flat_map(|constraint| {
      let common_areas: Vec<Area> = constraint.cell_region_areas(&cell1, PeerMode::Full, CandidateMode::Simple);
      let mut common_areas_set: HashSet<Area> = HashSet::from_iter(common_areas.into_iter());

      for cell in cells[1..].iter() {
        let areas: Vec<Area> = constraint.cell_region_areas(&cell, PeerMode::Full, CandidateMode::Simple);
        let areas_set: HashSet<Area> = HashSet::from_iter(areas.into_iter());
        common_areas_set = common_areas_set.intersection(&areas_set).cloned().collect();
      }

      common_areas_set.into_iter().sorted().collect::<Vec<_>>()
    }).collect();

    self.common_region_areas_cache.borrow_mut().insert(cells.clone(), common_areas.clone());

    common_areas
  }

  pub fn cells_are_peers(&self, cell1: &CellPosition, cell2: &CellPosition) -> bool {
    cell1 != cell2 &&
    !self.find_common_region_areas(&vec![*cell1, *cell2]).is_empty()
  }

  pub fn all_cell_peers_unconstrained(&self, cell: &CellPosition) -> Vec<CellPosition> {
    self.all_cell_peers(cell, PeerMode::Full, CandidateMode::Full)
  }

  /// Returns the <cell> peers that are special and are not peers through classic constraints
  pub fn cell_only_special_peers(&self, cell: &CellPosition) -> Vec<CellPosition> {
    let special_peers = self.all_cell_peers(cell, PeerMode::Special, CandidateMode::Full);
    let classic_peers = self.all_cell_peers(cell, PeerMode::Classic, CandidateMode::Full);

    special_peers
      .into_iter()
      .filter(|special_peer|
        !classic_peers.contains(special_peer)
      )
      .collect()
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
  pub fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    match area {
      Area::Adhoc(cells) => cells.clone(),
      Area::Cell(row, col) => vec![CellPosition::new(*row, *col)],
      Area::Grid => self.all_grid_cells(),

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
    }
  }

  pub fn all_grid_cells(&self) -> Vec<CellPosition> {
    CellPosition::all_cells_in_grid_size(self.grid_size)
  }

  pub fn constraint(&self, kind: ConstraintKind) -> Option<&Constraint> {
    self.constraints_by_kind[kind as usize].map(|index| &self.constraints[index])
  }

  pub fn thermo(&self) -> Option<&ThermoConstraint> {
    let Some(Constraint::Thermo(constraint)) = self.constraint(ConstraintKind::Thermo) else { return None };
    Some(constraint)
  }

  pub fn kropki(&self) -> Option<&KropkiConstraint> {
    let Some(Constraint::Kropki(constraint)) = self.constraint(ConstraintKind::Kropki) else { return None };
    Some(constraint)
  }

  pub fn killer(&self) -> Option<&KillerConstraint> {
    let Some(Constraint::Killer(constraint)) = self.constraint(ConstraintKind::Killer) else { return None };
    Some(constraint)
  }

  pub fn arrow(&self) -> Option<&ArrowConstraint> {
    let Some(Constraint::Arrow(constraint)) = self.constraint(ConstraintKind::Arrow) else { return None };
    Some(constraint)
  }

  pub fn top_bottom(&self) -> Option<&TopBottomConstraint> {
    let Some(Constraint::TopBottom(constraint)) = self.constraint(ConstraintKind::TopBottom) else { return None };
    Some(constraint)
  }

  pub fn renban(&self) -> Option<&RenbanConstraint> {
    let Some(Constraint::Renban(constraint)) = self.constraint(ConstraintKind::Renban) else { return None };
    Some(constraint)
  }

  pub fn palindrome(&self) -> Option<&PalindromeConstraint> {
    let Some(Constraint::Palindrome(constraint)) = self.constraint(ConstraintKind::Palindrome) else { return None };
    Some(constraint)
  }

  pub fn fixed_number(&self) -> Option<&FixedNumbersConstraint> {
    let Some(Constraint::FixedNumbers(constraint)) = self.constraint(ConstraintKind::FixedNumber) else { return None };
    Some(constraint)
  }

  pub fn region(&self) -> Option<&RegionsConstraint> {
    let Some(Constraint::Regions(constraint)) = self.constraint(ConstraintKind::Regions) else { return None };
    Some(constraint)
  }
}

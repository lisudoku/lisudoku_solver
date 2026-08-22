use crate::{solver::{checker::SolvedState, constraints::{anti_king::AntiKingConstraint, anti_knight::AntiKnightConstraint, arrow::ArrowConstraint, context::{ConstraintCellPeersContext}, diagonals::{PrimaryDiagonalConstraint, SecondaryDiagonalConstraint}, fixed_numbers::FixedNumbersConstraint, killer::KillerConstraint, kropki::KropkiConstraint, odd_even::{EvenConstraint, OddConstraint}, palindrome::PalindromeConstraint, regions::RegionsConstraint, renban::RenbanConstraint, thermo::ThermoConstraint, top_bottom::TopBottomConstraint}, types::CandidateSet}, types::{Area, CellPosition, Grid, SudokuConstraints}};
use enum_dispatch::enum_dispatch;

pub mod kind;
pub(crate) mod context;
pub(crate) mod regions;
pub(crate) mod fixed_numbers;
pub(crate) mod thermo;
pub(crate) mod arrow;
pub(crate) mod killer;
pub(crate) mod anti_knight;
pub(crate) mod anti_king;
pub(crate) mod diagonals;
pub(crate) mod kropki;
pub(crate) mod odd_even;
pub(crate) mod top_bottom;
pub(crate) mod renban;
pub(crate) mod palindrome;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum CandidateMode {
  Simple,
  Full,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum PeerMode {
  Full,
  Special,
  Classic,
}

#[enum_dispatch(Constraint)]
pub(crate) trait ConstraintDefinition {
  /// Returns all areas with explicit uniqueness (e.g. not including thermo which has implicit uniqueness).
  /// They don't need to be houses, there is house_areas for that.
  fn region_areas(&self) -> Vec<Area>;

  /// Only checks "special" constraints because peer constraints are checked outside.
  /// Some constraints can also check candidates in partial solutions.
  fn check_grid_special_validity(&self, grid: &Grid, candidates: &Vec<Vec<CandidateSet>>) -> SolvedState;

  /// Returns all cells within the given <area>
  fn area_cells(&self, area: &Area) -> Vec<CellPosition>;

  /// Returns all areas with uniqueness properties that contain given cell.
  /// In some cases (based on candidate_mode parameter) we don't want to return all areas.
  /// For example thermo has implicit uniqueness which is why we exclue it for candidate value calculation,
  /// but include it when used for techniques like common peer elimination and adhoc naked sets.
  fn cell_region_areas(&self, cell: &CellPosition, peer_mode: PeerMode, candidate_mode: CandidateMode) -> Vec<Area>;

  /// Removes invalid cell candidates based on the current constraint.
  /// Only handles "special" rules because peers (returned by `cell_peers`) are enforced outside.
  fn restrict_cell_special_candidates(&self, cell: &CellPosition, grid: &Grid, candidates: &mut CandidateSet);

  /// Returns cells that are considered peers of the given <cell>.
  /// A peer does not have a precise definition. Two cells are peers if they affect each other's values.
  /// Sometimes even if two cells affect each other we will not consider them peers because
  /// it's more natural to do it through a Technique.
  fn cell_peers(&self, cell: &CellPosition, ctx: &ConstraintCellPeersContext) -> Vec<CellPosition>;
}

pub(crate) trait ConstraintDefinitionExt: ConstraintDefinition {
  /// Reusable default behaviour of `cell_peers` that looks at areas.
  /// We want something else when the constraint doesn't use areas (e.g. anti knight).
  fn default_cell_peers(&self, cell: &CellPosition, peer_mode: PeerMode, candidate_mode: CandidateMode) -> Vec<CellPosition>;

  /// Returns areas that are houses (unique regions that need a full set of <grid_size> digits)
  fn house_areas(&self, grid_size: usize) -> Vec<Area>;
}

impl<T: ConstraintDefinition> ConstraintDefinitionExt for T {
  fn default_cell_peers(&self, cell: &CellPosition, peer_mode: PeerMode, candidate_mode: CandidateMode) -> Vec<CellPosition> {
    let areas = self.cell_region_areas(cell, peer_mode, candidate_mode);

    areas.into_iter().flat_map(|area|
      self.area_cells(&area)
    ).collect()
  }

  fn house_areas(&self, grid_size: usize) -> Vec<Area> {
    self.region_areas()
      .into_iter()
      .filter(|area| self.area_cells(&area).len() == grid_size)
      .collect()
  }
}

pub(crate) trait ConstraintBuilder {
  fn from_input(&self, input: &SudokuConstraints) -> Option<Constraint>;
}

#[derive(Clone, Debug)]
#[enum_dispatch]
pub(crate) enum Constraint {
  Regions(RegionsConstraint),
  FixedNumbers(FixedNumbersConstraint),
  Thermo(ThermoConstraint),
  Arrow(ArrowConstraint),
  Killer(KillerConstraint),
  AntiKnight(AntiKnightConstraint),
  AntiKing(AntiKingConstraint),
  PrimaryDiagonal(PrimaryDiagonalConstraint),
  SecondaryDiagonal(SecondaryDiagonalConstraint),
  Kropki(KropkiConstraint),
  Odd(OddConstraint),
  Even(EvenConstraint),
  TopBottom(TopBottomConstraint),
  Renban(RenbanConstraint),
  Palindrome(PalindromeConstraint),
}

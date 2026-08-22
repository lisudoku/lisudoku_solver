use crate::types::{Area, CellPosition, Grid};
use std::fmt::{self, Debug, Display};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

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

#[derive(Debug)]
pub enum ConstraintError {
  InvalidValue {
    field: &'static str,
    message: &'static str,
  },
}

impl Display for Rule {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      Debug::fmt(self, f)
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

use crate::{solver::logical::techniques::TechniqueKind, types::{Area, CellPosition, Grid}};
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Serialize, Deserialize, Debug, Tsify)]
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
pub enum SolutionType {
  Full,
  Partial,
  None,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct InvalidStateReason {
  pub state_type: InvalidStateType,
  pub area: Area,
  pub values: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Tsify)]
pub enum InvalidStateType {
  CellNoCandidates,
  CellEmpty,
  CellInvalidValue,
  AreaValueConflict,
  AreaCandidates,
  AreaConstraint,
}

#[derive(Serialize, Deserialize, Debug, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct SudokuBruteSolveResult {
  pub solution_count: u32,
  #[tsify(optional)]
  pub solution: Option<Grid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolutionStep {
  pub technique_kind: TechniqueKind,
  /// Used for grid steps (the first cell should have `value`) or e.g. TechniqueKind::XWing to
  /// show which cells build the X shape
  pub cells: Vec<CellPosition>,
  /// The meaning can vary between techniques
  pub values: Vec<u32>,
  /// Used in techniques that are applied inside an area. Could be multiple e.g. TechniqueKind::XWing
  pub areas: Vec<Area>,
  /// Used for non-grid steps. The meaning can vary between techniques,
  /// but we will remove candidates from these cells.
  pub affected_cells: Vec<CellPosition>,
  #[tsify(optional)]
  pub grid: Option<Grid>,
  #[tsify(optional)]
  pub candidates: Option<Vec<Vec<Vec<u32>>>>,
  /// Used for NishioForcingChain and NakedSingles
  #[tsify(optional)]
  pub invalid_state_reason: Option<InvalidStateReason>,
}

#[derive(Debug)]
pub enum ConstraintError {
  InvalidValue {
    field: &'static str,
    message: &'static str,
  },
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
    technique_kind: TechniqueKind, cells: Vec<CellPosition>, values: Vec<u32>,
    areas: Vec<Area>, affected_cells: Vec<CellPosition>,
  ) -> SolutionStep {
    SolutionStep {
      technique_kind,
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
    [
      TechniqueKind::NakedSingle,
      TechniqueKind::HiddenSingle,
      TechniqueKind::ThermoNakedSingle,
      TechniqueKind::PalindromeValue,
    ].contains(&self.technique_kind)
  }
}

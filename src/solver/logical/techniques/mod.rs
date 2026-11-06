mod kind;
pub(crate) mod naked_single;
pub(crate) mod hidden_single;
pub(crate) mod thermo_naked_single;
pub(crate) mod candidate_initialization;
pub(crate) mod locked_candidate_set;
pub(crate) mod naked_set;
pub(crate) mod thermo_candidate_update;
pub(crate) mod hidden_set;
pub(crate) mod x_wing;
pub(crate) mod xy_wing;
pub(crate) mod common_peer_elimination;
pub(crate) mod sum_candidates;
pub(crate) mod killer_candidate_update;
pub(crate) mod killer45;
pub(crate) mod kropki_chain_candidate_update;
pub(crate) mod kropki_advanced_candidate_update;
pub(crate) mod kropki_common_peer_elimination;
pub(crate) mod turbot_fish;
pub(crate) mod top_bottom_candidate_update;
pub(crate) mod empty_rectangle;
pub(crate) mod combinations;
pub(crate) mod arrow_candidate_update;
pub(crate) mod advanced_candidates;
pub(crate) mod arrow_advanced_candidate_update;
pub(crate) mod arrow_common_peer_elimination;
// pub(crate) mod phistomefel_ring;
pub(crate) mod nishio_forcing_chain;
pub(crate) mod renban_candidate_update;
pub(crate) mod palindrome_value;
pub(crate) mod palindrome_candidate_update;
pub(crate) mod adhoc_naked_set;

use crate::{solver::{GridCheckResult, SolveState, logical::{logical_solver::LogicalSolver, techniques::{adhoc_naked_set::AdhocNakedSet, arrow_advanced_candidate_update::ArrowAdvancedCandidateUpdate, arrow_candidate_update::ArrowCandidateUpdate, arrow_common_peer_elimination::ArrowCommonPeerElimination, candidate_initialization::CandidateInitialization, common_peer_elimination::CommonPeerElimination, empty_rectangle::EmptyRectangle, hidden_set::HiddenSet, hidden_single::HiddenSingle, killer_candidate_update::KillerCandidateUpdate, killer45::Killer45, kropki_advanced_candidate_update::KropkiAdvancedCandidateUpdate, kropki_chain_candidate_update::KropkiChainCandidateUpdate, kropki_common_peer_elimination::KropkiCommonPeerElimination, locked_candidate_set::LockedCandidateSet, naked_set::NakedSet, naked_single::NakedSingle, nishio_forcing_chain::NishioForcingChain, palindrome_candidate_update::PalindromeCandidateUpdate, palindrome_value::PalindromeValue, renban_candidate_update::RenbanCandidateUpdate, thermo_candidate_update::ThermoCandidateUpdate, thermo_naked_single::ThermoNakedSingle, top_bottom_candidate_update::TopBottomCandidateUpdate, turbot_fish::TurbotFish, x_wing::XWing, xy_wing::XYWing}}, types::CandidateSet}, types::{Area, CellPosition, InvalidStateReason, InvalidStateType, SolutionStep}};
use enum_dispatch::enum_dispatch;

pub use kind::TechniqueKind;
use strum::{EnumCount, EnumIter};

#[enum_dispatch(Technique)]
pub(crate) trait TechniqueDefinition {
  fn kind(&self) -> TechniqueKind;

  fn phase(&self) -> TechniquePhase;

  fn step_application(&self) -> StepApplication {
    StepApplication::RemoveListedCandidates
  }

  fn complexity(&self, _step: &SolutionStep, _solver: &LogicalSolver) -> u32 { u32::MAX }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep>;

  // This is the default base implementation, but can be overridden
  // Return value indicates whether applying this step leads to a contradiction
  // (overriding cell value, things that check_partially_solved would catch only later)
  fn apply(&self, step: &SolutionStep, state: &mut SolveState, step_count_limit: Option<usize>) -> GridCheckResult {
    match self.step_application() {
      StepApplication::SetCellValue => {
        // This can happen if naked single finds cell without candidates
        if let Some(invalid_state_reason) = step.invalid_state_reason.clone() {
          return GridCheckResult::unsolved(invalid_state_reason)
        }

        if state.candidates_active {
          assert_eq!(step.cells.len(), 1);
        }
        let cell = step.cells[0];
        let CellPosition { row, col } = cell;
        let value = step.values[0];

        if step_count_limit.is_some() {
          // In this mode we can apply multiple steps at once and
          // we could find naked and hidden single for the same cell
          // So if we find a contradiction we should stop

          if state.value_at(&cell) != 0 && state.value_at(&cell) != value {
            return GridCheckResult::unsolved(
              InvalidStateReason {
                state_type: InvalidStateType::CellInvalidValue,
                area: Area::Cell(cell.row, cell.col),
                values: vec![value],
              }
            )
          }
        } else {
          assert!(
            state.value_at(&cell) == 0,
            "Attempted to overwrite non-empty cell from {} to {}",
            state.value_at(&cell), value,
          );
        }

        if state.value_at(&cell) == 0 {
          state.set_value(&cell, value);

          if state.candidates_active {
            state.candidates[row][col].clear();
            state.update_candidates(&step.affected_cells, value);
          }
        }
      },
      StepApplication::RemoveCorrespondingCandidates => {
        for (index, cell) in step.affected_cells.iter().enumerate() {
          let value = step.values[index];
          state.candidates[cell.row][cell.col].remove(&value);
        }
      },
      StepApplication::RemoveListedCandidates => {
        for &CellPosition { row, col } in &step.affected_cells {
          for value in &step.values {
            state.candidates[row][col].remove(value);
          }
        }
      },
      StepApplication::Custom => unreachable!("custom techniques must override apply"),
    }

    GridCheckResult::solved()
  }

  fn build_solution_step(
    &self,
    cells: Vec<CellPosition>, values: Vec<u32>, areas: Vec<Area>, affected_cells: Vec<CellPosition>
  ) -> SolutionStep {
    SolutionStep::new(self.kind(), cells, values, areas, affected_cells)
  }

  fn build_grid_solution_step(
    &self, cells: Vec<CellPosition>, values: Vec<u32>, areas: Vec<Area>, solver: &LogicalSolver,
  ) -> SolutionStep {
    assert!(self.step_application() == StepApplication::SetCellValue);
    let affected_cells = if solver.state.candidates_active {
      assert_eq!(cells.len(), 1);
      let cell = cells[0];
      let value = values[0];
      let values_set = &CandidateSet::from([value]);
      solver.state.cell_peers_with_candidates(&cell, values_set)
    } else {
      vec![]
    };
    self.build_solution_step(cells, values, areas, affected_cells)
  }

  fn build_simple_solution_step(
    &self,
    values: Vec<u32>, areas: Vec<Area>, affected_cells: Vec<CellPosition>
  ) -> SolutionStep {
    self.build_solution_step(vec![], values, areas, affected_cells)
  }

  fn build_invalid_cell_step(&self, cell: &CellPosition) -> SolutionStep {
    SolutionStep{
      technique_kind: TechniqueKind::NakedSingle,
      cells: vec![*cell],
      values: vec![],
      areas: vec![],
      affected_cells: vec![],
      grid: None,
      candidates: None,
      invalid_state_reason: Some(InvalidStateReason{
        state_type: InvalidStateType::CellNoCandidates,
        area: Area::Cell(cell.row, cell.col),
        values: vec![],
      }),
    }
  }
}

#[repr(usize)]
#[derive(EnumIter, EnumCount, PartialEq)]
pub(crate) enum TechniquePhase {
  // All XCandidateUpdate type techniques should use this because they need to run first.
  // These are the techniques that enforce constraints to make sure all candidates are
  // valid before applying other techniques.
  CandidateValidity,
  // It fills in a value in the grid
  Grid,
  // Removes candidates from cells
  CandidateElimination,
}

#[derive(PartialEq)]
pub(crate) enum StepApplication {
  SetCellValue,
  RemoveListedCandidates,
  RemoveCorrespondingCandidates,
  Custom,
}

#[derive(Clone, Debug)]
#[enum_dispatch]
pub(crate) enum Technique {
  NakedSingle(NakedSingle),
  HiddenSingle(HiddenSingle),
  ThermoNakedSingle(ThermoNakedSingle),
  CandidateInitialization(CandidateInitialization),
  ThermoCandidateUpdate(ThermoCandidateUpdate),
  KillerCandidateUpdate(KillerCandidateUpdate),
  ArrowCandidateUpdate(ArrowCandidateUpdate),
  RenbanCandidateUpdate(RenbanCandidateUpdate),
  PalindromeValue(PalindromeValue),
  PalindromeCandidateUpdate(PalindromeCandidateUpdate),
  ArrowAdvancedCandidateUpdate(ArrowAdvancedCandidateUpdate),
  Killer45(Killer45),
  KropkiChainCandidateUpdate(KropkiChainCandidateUpdate),
  KropkiAdvancedCandidateUpdate(KropkiAdvancedCandidateUpdate),
  TopBottomCandidateUpdate(TopBottomCandidateUpdate),
  LockedCandidateSet(LockedCandidateSet),
  NakedSet(NakedSet),
  HiddenSet(HiddenSet),
  CommonPeerElimination(CommonPeerElimination),
  KropkiCommonPeerElimination(KropkiCommonPeerElimination),
  ArrowCommonPeerElimination(ArrowCommonPeerElimination),
  XWing(XWing),
  XYWing(XYWing),
  TurbotFish(TurbotFish),
  EmptyRectangle(EmptyRectangle),
  AdhocNakedSet(AdhocNakedSet),
  NishioForcingChain(NishioForcingChain),
}

use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints, Palindrome}};

#[derive(Clone, Default, Debug)]
pub struct PalindromeConstraint {
  pub palindromes: Vec<Palindrome>,
}

impl ConstraintDefinition for PalindromeConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    for palindrome_index in 0..self.palindromes.len() {
      let area = Area::Palindrome(palindrome_index);
      let cells = self.area_cells(&area);
      let values: Vec<u32> = cells.into_iter().map(|cell| grid[cell.row][cell.col]).collect();
      let mut left = 0;
      let mut right = values.len() - 1;
      while left < right {
        if values[left] != 0 && values[right] != 0 && values[left] != values[right] {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::AreaConstraint,
              area: area.clone(),
              values: vec![left as u32, right as u32],
            }
          )
        }
        left += 1;
        right -= 1;
      }
    }

    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> { vec![] }

  fn area_cells(&self, area: &Area) -> Vec<CellPosition> {
    let Area::Palindrome(palindrome_index) = area else { return vec![] };

    self.palindromes[*palindrome_index].0.clone()
  }

  fn cell_region_areas(&self, _cell: &CellPosition, _peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> { vec![] }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {}

  fn cell_peers(&self, _cell: &CellPosition, _ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    vec![]
  }
}

pub struct PalindromeConstraintBuilder;

impl ConstraintBuilder for PalindromeConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    let palindromes = constraints.palindromes.clone().unwrap_or_default();

    if palindromes.is_empty() {
      return None
    }

    Some(Constraint::Palindrome(PalindromeConstraint{
      palindromes,
    }))
  }
}

pub static PALINDROME_CONSTRAINT_BUILDER: PalindromeConstraintBuilder = PalindromeConstraintBuilder;

use crate::{solver::{checker::SolvedState, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellDirection, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub struct AntiKingConstraint;

impl ConstraintDefinition for AntiKingConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> SolvedState {
    for cell in CellPosition::all_cells_in_grid_size(grid.len()) {
      let value = grid[cell.row][cell.col];
      if value == 0 {
        continue
      }

      for peer in self.cell_peers(&cell, &ConstraintCellPeersContext{ peer_mode: PeerMode::Full, candidate_mode: CandidateMode::Full, grid_size: grid.len() }) {
        let peer_value = grid[peer.row][peer.col];
        if peer_value == 0 {
          continue
        }
        if value == peer_value {
          return SolvedState::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }
          )
        }
      }
    }

    SolvedState::solved()
  }

  fn region_areas(&self) -> Vec<Area> { vec![] }

  fn area_cells(&self, _area: &Area) -> Vec<CellPosition> { vec![] }

  fn cell_region_areas(&self, _cell: &CellPosition, _peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> { vec![] }

  fn restrict_cell_special_candidates(&self, _cell: &CellPosition, _grid: &Grid, _candidates: &mut CandidateSet) {}

  fn cell_peers(&self, cell: &CellPosition, ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    if ctx.peer_mode == PeerMode::Classic {
      return vec![]
    }

    KING_MOVES.iter().filter_map(|direction| {
      let prow = cell.row as isize + direction.row;
      let pcol = cell.col as isize + direction.col;
      if prow < 0 || prow >= ctx.grid_size as isize ||
         pcol < 0 || pcol >= ctx.grid_size as isize {
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

pub struct AntiKingConstraintBuilder;

impl ConstraintBuilder for AntiKingConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if !constraints.anti_king.unwrap_or_default() {
      return None
    }

    Some(Constraint::AntiKing(AntiKingConstraint{}))
  }
}

pub static ANTI_KING_CONSTRAINT_BUILDER: AntiKingConstraintBuilder = AntiKingConstraintBuilder;

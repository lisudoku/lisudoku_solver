use crate::{solver::{GridCheckResult, constraints::{CandidateMode, Constraint, ConstraintBuilder, ConstraintDefinition, PeerMode, context::{ConstraintCellPeersContext}}, types::CandidateSet}, types::{Area, CellDirection, CellPosition, Grid, InvalidStateReason, InvalidStateType, SudokuConstraints}};

#[derive(Clone, Default, Debug)]
pub(crate) struct AntiKnightConstraint {
  knight_peers: Vec<Vec<CellPosition>>,
}

impl ConstraintDefinition for AntiKnightConstraint {
  fn check_grid_special_validity(&self, grid: &Grid, _candidates: &Vec<Vec<CandidateSet>>) -> GridCheckResult {
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
          return GridCheckResult::unsolved(
            InvalidStateReason {
              state_type: InvalidStateType::CellInvalidValue,
              area: Area::Cell(cell.row, cell.col),
              values: vec![value],
            }
          )
        }
      }
    }

    GridCheckResult::solved()
  }

  fn region_areas(&self) -> Vec<Area> { vec![] }

  fn area_cells(&self, _area: &Area) -> Vec<CellPosition> { vec![] }

  fn cell_region_areas(&self, _cell: &CellPosition, _peer_mode: PeerMode, _candidate_mode: CandidateMode) -> Vec<Area> { vec![] }

  fn restrict_cell_special_candidates(&self, cell: &CellPosition, grid: &Grid, candidates: &mut CandidateSet) {
    // Same logic as cell_peers, but avoiding a clone. We can reuse cell_peers when it returns an iterator.
    let cell_index = cell.to_index(grid.len());
    for peer_cell in &self.knight_peers[cell_index] {
      candidates.remove(&grid[peer_cell.row][peer_cell.col]);
    }
  }

  fn cell_peers(&self, cell: &CellPosition, ctx: &ConstraintCellPeersContext) -> Vec<CellPosition> {
    if ctx.peer_mode == PeerMode::Classic {
      return vec![]
    }

    let cell_index = cell.to_index(ctx.grid_size);
    self.knight_peers[cell_index].clone()
  }
}

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

pub(crate) struct AntiKnightConstraintBuilder;

impl ConstraintBuilder for AntiKnightConstraintBuilder {
  fn from_input(&self, constraints: &SudokuConstraints) -> Option<Constraint> {
    if !constraints.anti_knight.unwrap_or_default() {
      return None
    }

    let mut knight_peers = vec![vec![]; constraints.grid_size * constraints.grid_size];
    for (cell_index, cell) in CellPosition::all_cells_in_grid_size(constraints.grid_size).into_iter().enumerate() {
      for direction in &KNIGHT_MOVES {
        let prow = cell.row as isize + direction.row;
        let pcol = cell.col as isize + direction.col;
        if prow >= 0 && prow < constraints.grid_size as isize && pcol >= 0 && pcol < constraints.grid_size as isize {
          knight_peers[cell_index].push(CellPosition::new(prow as usize, pcol as usize));
        }
      }
    }

    Some(Constraint::AntiKnight(AntiKnightConstraint{
      knight_peers,
    }))
  }
}

pub(crate) static ANTI_KNIGHT_CONSTRAINT_BUILDER: AntiKnightConstraintBuilder = AntiKnightConstraintBuilder;

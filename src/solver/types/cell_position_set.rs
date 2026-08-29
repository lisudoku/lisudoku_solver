use bit_set::BitSet;

use crate::types::CellPosition;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CellPositionSet {
  grid_size: usize,
  set: BitSet,
}

impl CellPositionSet {
  pub fn new(grid_size: usize) -> CellPositionSet {
    CellPositionSet{
      grid_size,
      set: BitSet::with_capacity(grid_size * grid_size),
    }
  }

  fn cell_to_index(grid_size: usize, cell: CellPosition) -> usize {
    cell.row * grid_size + cell.col
  }

  pub fn len(&self) -> usize {
    self.set.count()
  }

  pub fn insert(&mut self, cell: CellPosition) {
    self.set.insert(Self::cell_to_index(self.grid_size, cell));
  }

  pub fn union_with(&mut self, other: &CellPositionSet) {
    self.set.union_with(&other.set);
  }
}

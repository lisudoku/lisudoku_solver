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

  fn cell_to_index(cell: CellPosition, grid_size: usize) -> usize {
    cell.row * grid_size + cell.col
  }

  fn index_to_cell(index: usize, grid_size: usize) -> CellPosition {
    CellPosition::new(index / grid_size, index % grid_size)
  }

  pub fn len(&self) -> usize {
    self.set.count()
  }

  pub fn insert(&mut self, cell: CellPosition) {
    self.set.insert(Self::cell_to_index(cell, self.grid_size));
  }

  pub fn union_with(&mut self, other: &CellPositionSet) {
    self.set.union_with(&other.set);
  }
}

pub struct CellPositionSetIter<'a> {
  grid_size: usize,
  indices: bit_set::Iter<'a, u32>,
}

impl Iterator for CellPositionSetIter<'_> {
  type Item = CellPosition;

  fn next(&mut self) -> Option<Self::Item> {
    self.indices.next().map(|index| CellPositionSet::index_to_cell(index, self.grid_size))
  }
}

impl<'a> IntoIterator for &'a CellPositionSet {
  type Item = CellPosition;
  type IntoIter = CellPositionSetIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    CellPositionSetIter {
      grid_size: self.grid_size,
      indices: self.set.iter(),
    }
  }
}

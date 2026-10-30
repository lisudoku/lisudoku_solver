use bit_set::BitSet;
use itertools::Itertools;
use std::fmt;

use crate::types::CellPosition;

#[derive(Default, Clone, PartialEq)]
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

  pub fn len(&self) -> usize {
    self.set.count()
  }

  pub fn is_empty(&self) -> bool {
    self.set.is_empty()
  }

  pub fn insert(&mut self, cell: CellPosition) {
    self.set.insert(cell.to_index(self.grid_size));
  }

  pub fn remove(&mut self, cell: &CellPosition) {
    self.set.remove(cell.to_index(self.grid_size));
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
    self.indices.next().map(|index| CellPosition::from_index(index, self.grid_size))
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

impl fmt::Debug for CellPositionSet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "CellPositionSet: {}",
      self.into_iter().collect_vec().into_iter().map(|cell| cell.to_string()).join(","),
    ))
  }
}

use core::fmt;
use std::{ops::BitOr};
use itertools::Itertools;

/// Specialized implementation of a set for candidate values.
/// I tried reusing BitSet here but it turned out slower.

#[derive(Default, Clone, PartialEq, Copy)]
pub(crate) struct CandidateSet {
  mask: u16,
}

impl CandidateSet {
  pub(crate) fn new() -> CandidateSet {
    Self::default()
  }

  pub(crate) fn all_candidates(grid_size: u32) -> CandidateSet {
    // Creates a mask containing all values from 1 to grid_size
    CandidateSet{
      mask: (1 << (grid_size + 1)) - 2,
    }
  }

  pub(crate) fn is_empty(&self) -> bool {
    self.mask == 0
  }

  pub(crate) fn clear(&mut self) {
    self.mask = 0;
  }

  pub(crate) fn contains(&self, value: &u32) -> bool {
    (self.mask & (1 << value)) > 0
  }

  pub(crate) fn insert(&mut self, value: u32) {
    self.mask |= 1 << value;
  }

  pub(crate) fn remove(&mut self, value: &u32) {
    self.mask &= !(1 << value);
  }

  /// Retains only the elements specified by the predicate.
  /// In other words, remove all elements e for which f(&e) returns false.
  pub(crate) fn retain<F>(&mut self, f: F)
  where
    F: FnMut(&u32) -> bool,
  {
    let other = CandidateSet::from_iter(self.into_iter().filter(f));
    self.mask = other.mask;
  }

  /// start and end are inclusive
  pub(crate) fn retain_range(&mut self, start: u32, end: u32) {
    if start > end {
      self.mask = 0;
      return
    }
    let range_mask = (1 << (end + 1)) - (1 << start);
    self.mask &= range_mask;
  }

  pub(crate) fn is_disjoint(&self, other: &CandidateSet) -> bool {
    (self.mask & other.mask) == 0
  }

  pub(crate) fn is_subset(&self, other: &CandidateSet) -> bool {
    (self.mask & other.mask) == self.mask
  }

  pub(crate) fn union(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet {
      mask: self.mask | other.mask,
    }
  }

  pub(crate) fn intersection(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet {
      mask: self.mask & other.mask,
    }
  }

  pub(crate) fn difference(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet {
      mask: self.mask & !other.mask,
    }
  }

  pub(crate) fn len(&self) -> usize {
    self.mask.count_ones() as usize
  }

  pub(crate) fn has_count_one(&self) -> bool {
    self.mask.count_ones() == 1
  }

  pub(crate) fn to_bitmask(&self) -> u32 {
    self.mask as u32
  }
}

impl FromIterator<u32> for CandidateSet {
  fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> CandidateSet {
    CandidateSet{
      mask: iter.into_iter().fold(0, |acc, e| {
        acc.bitor(1 << e)
      }),
    }
  }
}

impl<const N: usize> From<[u32; N]> for CandidateSet {
  fn from(values: [u32; N]) -> Self {
    CandidateSet::from_iter(values.into_iter())
  }
}

pub(crate) struct SetBitsIter {
    remaining: u16,
}

impl Iterator for SetBitsIter {
  type Item = u32; // bit position

  fn next(&mut self) -> Option<Self::Item> {
    if self.remaining == 0 {
      return None;
    }
    let bit = self.remaining.trailing_zeros();
    self.remaining &= self.remaining - 1; // clear lowest set bit
    Some(bit)
  }
}

impl IntoIterator for CandidateSet {
  type Item = u32;
  type IntoIter = SetBitsIter;

  fn into_iter(self) -> Self::IntoIter {
    SetBitsIter { remaining: self.mask }
  }
}

impl IntoIterator for &CandidateSet {
  type Item = u32;
  type IntoIter = SetBitsIter;

  fn into_iter(self) -> Self::IntoIter {
    SetBitsIter { remaining: self.mask }
  }
}

impl fmt::Debug for CandidateSet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "CandidateSet: {}",
      self.into_iter().sorted().join(","),
    ))
  }
}

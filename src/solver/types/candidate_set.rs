use core::fmt;
use std::{ops::BitOr};
use itertools::Itertools;

#[derive(Default, Clone, PartialEq, Copy)]
pub struct CandidateSet {
  mask: u16,
}

impl CandidateSet {
  pub fn new() -> CandidateSet {
    Self::default()
  }

  pub fn all_candidates(grid_size: u32) -> CandidateSet {
    CandidateSet::from_iter(1..=grid_size)
  }

  pub fn is_empty(&self) -> bool {
    self.mask == 0
  }

  pub fn clear(&mut self) {
    self.mask = 0;
  }

  pub fn contains(&self, value: &u32) -> bool {
    (self.mask & (1 << value)) > 0
  }

  pub fn insert(&mut self, value: u32) -> bool {
    if self.contains(&value) {
      return false
    }
    self.mask |= 1 << value;
    true
  }

  pub fn remove(&mut self, value: &u32) -> bool {
    if !self.contains(value) {
      return false
    }
    self.mask ^= 1 << value;
    true
  }

  /// Retains only the elements specified by the predicate.
  /// In other words, remove all elements e for which f(&e) returns false.
  pub fn retain<F>(&mut self, f: F)
  where
    F: FnMut(&u32) -> bool,
  {
    let other = CandidateSet::from_iter(self.into_iter().filter(f));
    self.mask = other.mask;
  }

  pub fn extend<I: IntoIterator<Item = u32>>(&mut self, iter: I) {
    self.mask = self.union(&CandidateSet::from_iter(iter)).mask;
  }

  pub fn is_disjoint(&self, other: &CandidateSet) -> bool {
    (self.mask & other.mask) == 0
  }

  pub fn is_subset(&self, other: &CandidateSet) -> bool {
    (self.mask & other.mask) == self.mask
  }

  pub fn union(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet {
      mask: self.mask | other.mask,
    }
  }

  pub fn intersection(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet {
      mask: self.mask & other.mask,
    }
  }

  pub fn difference(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet {
      mask: self.mask & (self.mask ^ other.mask),
    }
  }

  pub fn len(&self) -> usize {
    self.mask.count_ones() as usize
  }

  pub fn to_bitmask(&self) -> u32 {
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

pub struct SetBitsIter {
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

impl fmt::Debug for CandidateSet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "CandidateSet: {}",
      self.into_iter().sorted().join(","),
    ))
  }
}

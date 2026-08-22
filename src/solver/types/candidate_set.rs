use core::fmt;
use std::{collections::{HashSet, hash_set}, ops::BitOr};
use itertools::Itertools;

#[derive(Default, Clone, PartialEq)]
pub struct CandidateSet {
  values: HashSet<u32>,
}

impl CandidateSet {
  pub fn new() -> CandidateSet {
    Self::default()
  }

  pub fn all_candidates(grid_size: u32) -> CandidateSet {
    CandidateSet::from_iter(1..=grid_size)
  }

  pub fn is_empty(&self) -> bool {
    self.values.is_empty()
  }

  pub fn clear(&mut self) {
    self.values.clear()
  }

  pub fn contains(&self, value: &u32) -> bool {
    self.values.contains(value)
  }

  pub fn insert(&mut self, value: u32) -> bool {
    self.values.insert(value)
  }

  pub fn remove(&mut self, value: &u32) -> bool {
    self.values.remove(value)
  }

  /// Retains only the elements specified by the predicate.
  /// In other words, remove all elements e for which f(&e) returns false.
  pub fn retain<F>(&mut self, f: F)
  where
    F: FnMut(&u32) -> bool,
  {
    self.values.retain(f);
  }

  pub fn extend<I: IntoIterator<Item = u32>>(&mut self, iter: I) {
    self.values.extend(iter)
  }

  pub fn is_disjoint(&self, other: &CandidateSet) -> bool {
    self.values.is_disjoint(&other.values)
  }

  pub fn is_subset(&self, other: &CandidateSet) -> bool {
    self.values.is_subset(&other.values)
  }

  pub fn union(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet::from_iter(
      self.values.union(&other.values).cloned().into_iter()
    )
  }

  pub fn intersection(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet::from_iter(
      self.values.intersection(&other.values).cloned().into_iter()
    )
  }

  pub fn difference(&self, other: &CandidateSet) -> CandidateSet {
    CandidateSet::from_iter(
      self.values.difference(&other.values).cloned().into_iter()
    )
  }

  pub fn len(&self) -> usize {
    self.values.len()
  }

  pub fn to_bitmask(&self) -> u32 {
    self.values.iter().fold(0, |acc, e| {
      acc.bitor(1 << e)
    })
  }
}

impl FromIterator<u32> for CandidateSet {
  fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> CandidateSet {
    CandidateSet{
      values: HashSet::from_iter(iter),
    }
  }
}

impl<const N: usize> From<[u32; N]> for CandidateSet {
  fn from(values: [u32; N]) -> Self {
    CandidateSet::from_iter(values.into_iter())
  }
}

impl IntoIterator for CandidateSet {
  type Item = u32;
  type IntoIter = hash_set::IntoIter<u32>;

  fn into_iter(self) -> Self::IntoIter {
    self.values.into_iter()
  }
}

impl<'a> IntoIterator for &'a CandidateSet {
  type Item = &'a u32;
  type IntoIter = hash_set::Iter<'a, u32>;

  fn into_iter(self) -> Self::IntoIter {
    self.values.iter()
  }
}

impl fmt::Debug for CandidateSet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!(
      "CandidateSet: {}",
      self.values.iter().sorted().join(","),
    ))
  }
}

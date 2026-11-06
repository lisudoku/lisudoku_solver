use super::CandidateSet;
use derive_more::{Deref, DerefMut};

/// A square grid of candidate sets, indexed by `[row][col]`.
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub(crate) struct CandidateGrid(Vec<Vec<CandidateSet>>);

impl CandidateGrid {
  pub(crate) fn new(grid_size: usize) -> Self {
    Self(vec![
      vec![CandidateSet::new(); grid_size];
      grid_size
    ])
  }
}

impl From<Vec<Vec<CandidateSet>>> for CandidateGrid {
  fn from(vec_2d: Vec<Vec<CandidateSet>>) -> Self {
    Self(vec_2d)
  }
}

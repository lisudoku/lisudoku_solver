use crate::{solver::constraints::{CandidateMode, PeerMode}};

pub(crate) struct ConstraintCellPeersContext {
  pub(crate) grid_size: usize,
  pub(crate) peer_mode: PeerMode,
  pub(crate) candidate_mode: CandidateMode,
}

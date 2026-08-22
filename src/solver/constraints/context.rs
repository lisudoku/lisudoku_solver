use crate::{solver::constraints::{CandidateMode, PeerMode}};

pub struct ConstraintCellPeersContext {
  pub grid_size: usize,
  pub peer_mode: PeerMode,
  pub candidate_mode: CandidateMode,
}

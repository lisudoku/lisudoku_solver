use std::fmt;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};
use tsify::Tsify;

#[repr(usize)]
#[derive(EnumIter, EnumCount, Serialize, Deserialize,  Debug, PartialEq, PartialOrd, Ord, Copy, Clone, Eq, Hash, Tsify)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub enum TechniqueKind {
  // Easy
  NakedSingle,
  HiddenSingle,
  ThermoNakedSingle,
  KropkiPairCandidateUpdate,
  CandidateInitialization,
  ThermoCandidateUpdate,
  KillerCandidateUpdate,
  ArrowCandidateUpdate,
  RenbanCandidateUpdate,
  PalindromeValue,
  PalindromeCandidateUpdate,
  // Medium
  ArrowAdvancedCandidateUpdate,
  Killer45,
  KropkiChainCandidateUpdate,
  KropkiAdvancedCandidateUpdate,
  TopBottomCandidateUpdate,
  LockedCandidatePair,
  NakedPair,
  HiddenPair,
  // Hard
  CommonPeerElimination,
  KropkiCommonPeerElimination,
  ArrowCommonPeerElimination,
  LockedCandidateTriple,
  NakedTriple,
  HiddenTriple,
  XWing,
  XYWing,
  TurbotFish,
  EmptyRectangle,
  AdhocNakedSet,
  NishioForcingChain,
}

impl fmt::Display for TechniqueKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      fmt::Debug::fmt(self, f)
  }
}

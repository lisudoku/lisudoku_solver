use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{Technique, TechniqueDefinition, TechniqueKind, adhoc_naked_set::AdhocNakedSet, arrow_advanced_candidate_update::ArrowAdvancedCandidateUpdate, arrow_candidate_update::ArrowCandidateUpdate, candidate_initialization::CandidateInitialization, common_peer_elimination::CommonPeerElimination, arrow_common_peer_elimination::ArrowCommonPeerElimination, kropki_common_peer_elimination::KropkiCommonPeerElimination, empty_rectangle::EmptyRectangle, hidden_set::HiddenSet, hidden_single::HiddenSingle, killer_candidate_update::KillerCandidateUpdate, killer45::Killer45, kropki_advanced_candidate_update::KropkiAdvancedCandidateUpdate, kropki_chain_candidate_update::KropkiChainCandidateUpdate, locked_candidate_set::LockedCandidateSet, naked_set::NakedSet, naked_single::NakedSingle, nishio_forcing_chain::NishioForcingChain, palindrome_candidate_update::PalindromeCandidateUpdate, palindrome_value::PalindromeValue, renban_candidate_update::RenbanCandidateUpdate, thermo_candidate_update::ThermoCandidateUpdate, thermo_naked_single::ThermoNakedSingle, top_bottom_candidate_update::TopBottomCandidateUpdate, turbot_fish::TurbotFish, x_wing::XWing, xy_wing::XYWing}}};
use std::collections::HashSet;
use itertools::Itertools;

impl LogicalSolver {
  pub fn with_hint_mode(mut self, flag: bool) -> Self {
    self.hint_mode = flag;
    self
  }

  pub fn with_step_count_limit(mut self, step_count_limit: usize) -> Self {
    self.step_count_limit = Some(step_count_limit);
    self
  }

  pub fn with_enriched_steps(mut self, enriched_steps_enabled: bool) -> Self {
    self.enriched_steps_enabled = enriched_steps_enabled;
    self
  }

  pub fn with_performance_mode(mut self) -> Self {
    self.performance_mode = true;
    self.state.performance_mode = true;
    self
  }

  pub fn with_techniques(mut self, technique_kinds: Vec<TechniqueKind>) -> Self {
    let technique_kinds_set: HashSet<TechniqueKind> = HashSet::from_iter(technique_kinds);
    self.enabled_techniques_by_phase = Self::group_techniques_by_phase(
      Self::all_techniques().filter(|t| technique_kinds_set.contains(&t.kind()))
    );
    self
  }

  pub fn with_techniques_order(mut self, technique_kinds: Vec<TechniqueKind>) -> Self {
    self.enabled_techniques_by_phase = Self::group_techniques_by_phase(
      technique_kinds
        .into_iter()
        .unique()
        .map(|technique_kind| {
          self.all_techniques_by_kind[technique_kind as usize].clone()
        })
    );
    self
  }

  pub fn without_techniques(mut self, technique_kinds: Vec<TechniqueKind>) -> Self {
    let technique_kinds_set: HashSet<TechniqueKind> = HashSet::from_iter(technique_kinds);
    for techniques in &mut self.enabled_techniques_by_phase {
      techniques.retain(|t| !technique_kinds_set.contains(&t.kind()));
    }
    self
  }

  pub(crate) fn all_techniques() -> impl Iterator<Item = Technique> {
    [
      ThermoCandidateUpdate.into(),
      KillerCandidateUpdate.into(),
      KropkiChainCandidateUpdate::new(false).into(),
      KropkiChainCandidateUpdate::new(true).into(),
      TopBottomCandidateUpdate::new(false).into(),
      ArrowCandidateUpdate.into(),
      RenbanCandidateUpdate.into(),
      PalindromeValue.into(),
      PalindromeCandidateUpdate.into(),
      HiddenSingle.into(),
      NakedSingle.into(),
      ThermoNakedSingle.into(),
      // Medium
      CandidateInitialization.into(),
      Killer45.into(),
      LockedCandidateSet::new(2).into(),
      NakedSet::new(2).into(),
      HiddenSet::new(2).into(),
      // Hard
      LockedCandidateSet::new(3).into(),
      NakedSet::new(3).into(),
      HiddenSet::new(3).into(),
      XWing.into(),
      XYWing.into(),
      CommonPeerElimination.into(),
      KropkiCommonPeerElimination.into(),
      KropkiAdvancedCandidateUpdate.into(),
      ArrowAdvancedCandidateUpdate.into(),
      ArrowCommonPeerElimination.into(),
      AdhocNakedSet.into(),
      TurbotFish.into(),
      EmptyRectangle.into(),
      NishioForcingChain.into(),
    ].into_iter()
  }
}

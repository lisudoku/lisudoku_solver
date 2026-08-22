use crate::{solver::{Solver, constraints::fixed_numbers::FixedNumbersConstraint, logical_solver::{adhoc_naked_set::AdhocNakedSet, arrow_advanced_candidates::ArrowAdvancedCandidates, arrow_candidates::ArrowCandidates, candidates::Candidates, common_peer_elimination::CommonPeerElimination, common_peer_elimination_arrow::CommonPeerEliminationArrow, common_peer_elimination_kropki::CommonPeerEliminationKropki, empty_reclanges::EmptyRectangles, hidden_set::HiddenSet, hidden_singles::HiddenSingles, killer_candidates::KillerCandidates, killer45::Killer45, kropki_advanced_candidates::KropkiAdvancedCandidates, kropki_chain_candidates::KropkiChainCandidates, locked_candidates::LockedCandidates, naked_set::NakedSet, naked_singles::NakedSingle, nishio_forcing_chains::NishioForcingChains, palindrome_candidates::PalindromeCandidates, palindrome_values::PalindromeValues, renban_candidates::RenbanCandidates, technique::Technique, thermo_candidates::ThermoCandidates, thermo_steps::Thermo, top_bottom_candidates::TopBottomCandidates, turbot_fish::TurbotFish, x_wing::XWing, xy_wing::XYWing}, types::CandidateSet}, types::{FixedNumber, Grid, Rule}};
use std::rc::Rc;

impl Solver {
  pub fn with_reset_grid(self) -> Self {
    let fixed_numbers: &Vec<FixedNumber> = if let Some(constraint) = self.puzzle.fixed_number() {
      &constraint.fixed_numbers
    } else {
      &vec![]
    };
    let initial_grid = FixedNumbersConstraint::build_initial_grid_from_fixed_numbers(
      fixed_numbers,
      self.puzzle.grid_size,
    );
    self.with_grid(initial_grid)
  }

  // TODO: should just pass grid to check_solved instead? and init the grid when starting a solve
  pub fn with_grid(mut self, grid: Grid) -> Self {
    self.grid = grid;
    self
  }

  pub fn with_reset_candidates(self) -> Self {
    let initial_candidates = Self::build_initial_candidates(self.puzzle.grid_size);
    self.with_candidates_active(false).with_candidates(initial_candidates)
  }

  pub fn with_candidates(mut self, candidates: Vec<Vec<CandidateSet>>) -> Self {
    self.candidates = candidates;
    self
  }

  pub fn with_candidates_active(mut self, candidates_active: bool) -> Self {
    self.candidates_active = candidates_active;
    self
  }

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

  pub fn with_techniques(mut self, techniques: Vec<Rc<dyn Technique>>) -> Self {
    self.techniques = techniques;
    self
  }

  pub fn without_techniques(mut self, techniques: Vec<Rc<dyn Technique>>) -> Self {
    let rules: Vec<Rule> = techniques.into_iter().map(|t| t.get_rule()).collect();
    self.techniques = self.techniques.into_iter().filter(|t| !rules.contains(&t.get_rule())).collect();
    self
  }

  pub fn default_techniques() -> Vec<Rc<dyn Technique>> {
    vec![
      Rc::new(ThermoCandidates),
      Rc::new(KillerCandidates),
      Rc::new(KropkiChainCandidates::new(false)),
      Rc::new(KropkiChainCandidates::new(true)),
      Rc::new(TopBottomCandidates::new(false)),
      Rc::new(ArrowCandidates),
      Rc::new(RenbanCandidates),
      Rc::new(PalindromeValues),
      Rc::new(PalindromeCandidates),
      Rc::new(HiddenSingles),
      Rc::new(NakedSingle),
      Rc::new(Thermo),
      Rc::new(Candidates),
      Rc::new(Killer45),
      Rc::new(LockedCandidates::new(2)),
      Rc::new(NakedSet::new(2)),
      Rc::new(HiddenSet::new(2)),
      Rc::new(LockedCandidates::new(3)),
      Rc::new(NakedSet::new(3)),
      Rc::new(HiddenSet::new(3)),
      Rc::new(XWing),
      Rc::new(XYWing),
      Rc::new(CommonPeerElimination),
      Rc::new(CommonPeerEliminationKropki),
      Rc::new(KropkiAdvancedCandidates),
      Rc::new(ArrowAdvancedCandidates),
      Rc::new(CommonPeerEliminationArrow),
      Rc::new(AdhocNakedSet),
      Rc::new(TurbotFish),
      Rc::new(EmptyRectangles),
      // Rc::new(PhistomefelRing), // disabled for now...
      Rc::new(NishioForcingChains),
    ]
  }
}

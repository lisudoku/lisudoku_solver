use crate::{solver::logical::techniques::{advanced_candidates::CellEliminationsResult, arrow_candidate_update::ArrowCombinationLogicFactory, combinations::cell_combination_logic::CellsCacheKey, kropki_chain_candidate_update::KropkiChainCombinationLogicFactory}, types::{Area, CellPosition, KropkiDotType}};
use std::{cell::RefCell, collections::HashMap};

pub(crate) struct LogicalCaches {
  pub(crate) arrow_combinatons_logic_factory: RefCell<ArrowCombinationLogicFactory>,
  pub(crate) cell_eliminations_cache: RefCell<HashMap<CellsCacheKey, CellEliminationsResult>>,
  pub(crate) area_kropki_ccs_cache: RefCell<HashMap<(Area, KropkiDotType, bool), Vec<(Vec<CellPosition>,Vec<usize>)>>>,
  pub(crate) kropki_chain_combinatons_logic_factory: RefCell<KropkiChainCombinationLogicFactory>,
}

impl LogicalCaches {
  pub(crate) fn new() -> LogicalCaches {
    LogicalCaches{
      arrow_combinatons_logic_factory: RefCell::new(ArrowCombinationLogicFactory::new()),
      cell_eliminations_cache: RefCell::new(HashMap::new()),
      area_kropki_ccs_cache: RefCell::new(HashMap::new()),
      kropki_chain_combinatons_logic_factory: RefCell::new(KropkiChainCombinationLogicFactory::new()),
    }
  }
}

impl Clone for LogicalCaches {
  fn clone(&self) -> Self {
    Self{
      arrow_combinatons_logic_factory: RefCell::new(ArrowCombinationLogicFactory::new()),
      cell_eliminations_cache: self.cell_eliminations_cache.clone(),
      area_kropki_ccs_cache: self.area_kropki_ccs_cache.clone(),
      kropki_chain_combinatons_logic_factory: RefCell::new(KropkiChainCombinationLogicFactory::new()),
    }
  }
}

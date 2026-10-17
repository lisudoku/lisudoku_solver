use crate::{solver::logical::techniques::{advanced_candidates::CellEliminationsResult, arrow_candidates::ArrowCombinationLogicFactory, combinations::cell_combination_logic::CellsCacheKey, kropki_chain_candidates::KropkiChainCombinationLogicFactory}, types::{Area, CellPosition, KropkiDotType}};
use std::{cell::RefCell, collections::HashMap};

pub(crate) struct LogicalCaches {
  pub arrow_combinatons_logic_factory: RefCell<ArrowCombinationLogicFactory>,
  pub cell_eliminations_cache: RefCell<HashMap<CellsCacheKey, CellEliminationsResult>>,
  pub area_kropki_ccs_cache: RefCell<HashMap<(Area, KropkiDotType, bool), Vec<(Vec<CellPosition>,Vec<usize>)>>>,
  pub kropki_chain_combinatons_logic_factory: RefCell<KropkiChainCombinationLogicFactory>,
}

impl LogicalCaches {
  pub fn new() -> LogicalCaches {
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

use crate::solver::Solver;
use crate::solver::logical_solver::combinations::cell_combinations_runner::CellCombinationsRunner;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use super::technique::Technique;
use std::collections::{HashMap, HashSet};

// Eliminate 1 candidate from a cell because all arrow combinations would eliminate it
// Normal locked candidates don't pick this up because they think ALL candidate combinations are possible
pub struct CommonPeerEliminationArrow;

impl Technique for CommonPeerEliminationArrow {
  fn get_rule(&self) -> Rule { Rule::CommonPeerEliminationArrow }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }
    if solver.constraints.arrows.is_empty() {
      return vec![]
    }

    solver.constraints.arrows.iter().enumerate().filter_map(|(arrow_index, arrow)| {
      let cells = arrow.all_cells();

      let mut arrow_combinatons_logic_factory = solver.arrow_combinatons_logic_factory.borrow_mut();
      let combination_logic = arrow_combinatons_logic_factory.create(arrow, solver);
      let mut runner = CellCombinationsRunner::new(solver, Box::new(combination_logic));
      let (_, combinations) = runner.run();

      let cell_peers: Vec<Vec<CellPosition>> = cells.iter().map(|cell| {
        solver.get_cell_peers(cell, true)
      }).collect();
  
      let mut cell_eliminations: HashSet<(CellPosition, u32)> = HashSet::new();

      for (idx, combination) in combinations.iter().enumerate() {
        let mut changed_cells: HashMap<CellPosition, HashSet<u32>> = HashMap::new();
        for cell_index in 0..cells.len() {
          let cell_value = combination[cell_index];
          for peer_cell in &cell_peers[cell_index] {
            if solver.candidates[peer_cell.row][peer_cell.col].contains(&cell_value) {
              let entry = changed_cells.entry(*peer_cell).or_insert(HashSet::new());
              entry.insert(cell_value);
            }
          }
        }
        
        let updates: HashSet<(CellPosition, u32)> = changed_cells.into_iter().flat_map(|(cell, candidates)| {
          candidates.into_iter().map(|c| (cell, c)).collect::<Vec<_>>()
        }).collect();

        if idx == 0 {
          cell_eliminations = updates;
        } else {
          cell_eliminations = cell_eliminations.intersection(&updates).copied().collect();
        }
      }

      if cell_eliminations.is_empty() {
        return None
      }

      // TODO: may eliminate same candidate twice?
      Some(
        SolutionStep {
          rule: self.get_rule(),
          cells: vec![],
          values: cell_eliminations.iter().map(|(_, c)| c).copied().collect(), // values in the same order as affected_cells
          areas: vec![ Area::Arrow(arrow_index) ],
          affected_cells: cell_eliminations.iter().map(|(cell, _)| cell).copied().collect(),
          candidates: None,
        }
      )
    }).collect()
  }

  fn apply_corresponding_indices(&self) -> bool { true }
}

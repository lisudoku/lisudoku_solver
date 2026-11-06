use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::types::CandidateSet;
use crate::types::{SolutionStep, CellPosition, Area};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

// Cell can’t be X because it eliminates all X candidates from a region
// It is a more general version of the Locked CandidateInitialization
// For diagonal puzzles the technique is called Crossover
// For antiknight puzzles the technique is called L technique

#[derive(Clone, Debug)]
pub struct CommonPeerElimination;

impl TechniqueDefinition for CommonPeerElimination {
  fn kind(&self) -> TechniqueKind { TechniqueKind::CommonPeerElimination }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    for area in solver.state.puzzle.all_house_areas() {
      let cells_by_value = solver.state.compute_cells_by_value_in_area(&area, &solver.state.candidates);
      for (value, cells) in cells_by_value {
        let step = self.find_common_peer_elimination_cells_with_value(solver, &area, cells, value);
        if step.is_some() {
          return vec![ step.unwrap() ]
        }
      }
    }

    vec![]
  }
}

impl CommonPeerElimination {
  fn find_common_peer_elimination_cells_with_value(&self, solver: &LogicalSolver, area: &Area, cells: Vec<CellPosition>, value: u32) -> Option<SolutionStep> {
    let values = CandidateSet::from([value]);
    let affected_cells: Vec<CellPosition> = self.find_common_peers_for_cells_with_values(solver, &cells, &values);

    if affected_cells.is_empty() {
      return None
    }

    Some(
      self.build_solution_step(
        cells,
        vec![ value ],
        vec![ area.clone() ],
        affected_cells,
      )
    )
  }

  fn find_common_peers_for_cells_with_values(&self, solver: &LogicalSolver, cells: &Vec<CellPosition>, values: &CandidateSet) -> Vec<CellPosition> {
    let common_peers = Self::find_common_peers_for_cells(solver, cells);
    let common_peers_with_values: Vec<CellPosition> = solver.state.filter_cells_with_any_candidates(
      &common_peers, &values
    );
    common_peers_with_values
  }

  pub fn find_common_peers_for_cells(solver: &LogicalSolver, cells: &Vec<CellPosition>) -> Vec<CellPosition> {
    let mut peer_counts = vec![ vec![ 0; solver.state.puzzle.grid_size ]; solver.state.puzzle.grid_size ];
    for cell in cells {
      for CellPosition { row, col } in solver.state.puzzle.all_cell_peers_unconstrained(cell) {
        peer_counts[row][col] += 1;
      }
    }
    for &CellPosition { row, col } in cells {
      peer_counts[row][col] = 0;
    }
    let common_peers: Vec<CellPosition> = solver.state.all_empty_cells()
      .into_iter()
      .filter(|&cell| peer_counts[cell.row][cell.col] == cells.len())
      .collect();

    common_peers
  }

  /// Returns a list of cell candidates that cannot exist because all `cells` `combinations` remove them
  pub fn find_cell_eliminations(cells: Vec<CellPosition>, combinations: Vec<Vec<u32>>, solver: &LogicalSolver) -> Vec<(CellPosition, u32)> {
    let cell_peers: Vec<Vec<CellPosition>> = cells.iter().map(|cell| {
      solver.state.puzzle.all_cell_peers_unconstrained(cell)
    }).collect();

    let mut cell_eliminations: HashSet<(CellPosition, u32)> = HashSet::new();

    for (idx, combination) in combinations.iter().enumerate() {
      // TODO: instead of HashSet use CandidateSet
      let mut changed_cells: HashMap<CellPosition, HashSet<u32>> = HashMap::new();
      for cell_index in 0..cells.len() {
        let cell_value = combination[cell_index];
        for peer in &cell_peers[cell_index] {
          if solver.state.candidates[peer.row][peer.col].contains(&cell_value) {
            let entry = changed_cells.entry(*peer).or_insert(HashSet::new());
            entry.insert(cell_value);
          }
        }
      }

      let updates: HashSet<(CellPosition, u32)> = changed_cells.into_iter().flat_map(|(cell, candidates)| {
        candidates.into_iter().map(move |c| (cell, c))
      }).collect();

      if idx == 0 {
        cell_eliminations = updates;
      } else {
        cell_eliminations = cell_eliminations.intersection(&updates).copied().collect();
      }
    }

    cell_eliminations.into_iter().sorted().collect_vec()
  }
}

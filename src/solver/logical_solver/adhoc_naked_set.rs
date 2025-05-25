use std::collections::HashMap;
use std::ops::{BitAnd, BitOrAssign};
use itertools::Itertools;
use crate::solver::logical_solver::naked_set::NakedSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area, CellPosition};
use super::technique::Technique;

pub struct AdhocNakedSet;

impl Technique for AdhocNakedSet {
  fn get_rule(&self) -> Rule { Rule::AdhocNakedSet }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let cells = solver.get_all_empty_cells();

    for starting_cell in cells {
      let special_peers: Vec<CellPosition> = solver.get_cell_only_special_peers(&starting_cell, true)
        .into_iter()
        .filter(|cell| solver.grid[cell.row][cell.col] == 0)
        .collect();

      if special_peers.is_empty() {
        continue
      }

      let peers: Vec<CellPosition> = solver.get_cell_peers(&starting_cell, true)
        .into_iter()
        .filter(|cell| solver.grid[cell.row][cell.col] == 0)
        .collect();
      
      // Do not even bother when there's too many peers because it hurts performance
      // and we want to use 32-bit masks
      if peers.len() >= 31 {
        continue
      }

      let nodes = vec![vec![starting_cell], peers].concat();

      let mut node_indices: HashMap<CellPosition, usize> = HashMap::new();
      for (index, node) in nodes.iter().enumerate() {
        node_indices.entry(*node).or_insert(index);
      }

      // Precalculate neighbour masks
      let neighbour_masks: Vec<u32> = nodes.iter().map(|node| {
        let node_neighbours = solver.get_cell_peers(node, true);
        let mut neighbour_mask: u32 = 0;

        for neighbour in node_neighbours {
          if let Some(index) = node_indices.get(&neighbour) {
            neighbour_mask.bitor_assign(1 << index);
          }
        }

        neighbour_mask
      }).collect();

      let mut cliques: Vec<u32> = vec![];
      // Start with a special peer so we know that this clique will not be within a classic area
      for special_peer in &special_peers {
        // Start the algorithm with R = {cell, special_peer}, P = peers \ {cell, special_peer}, X = {}
        let peer_index = *node_indices.get(&special_peer).unwrap();
        let clique: u32 = 1u32 | (1u32 << peer_index);
        let peers: u32 = ((1 << nodes.len()) - 1) & neighbour_masks[peer_index];
        self.bron_kerbosch(&neighbour_masks, clique, peers, 0, &mut cliques);
      }

      for clique_mask in cliques {
        let clique: Vec<CellPosition> = (0..nodes.len())
          .filter(|b| clique_mask.bitand(1 << b) != 0)
          .map(|index| nodes[index])
          .sorted()
          .collect();

        if clique.len() < 3 {
          // Not enough cells, we won't find anything
          continue
        }

        // If the whole clique is within the same area, skip
        // because the same deduction can be made using simpler techniques.
        // We shouldn't reach this point because those techiques will run first, but still...
        let common_areas = solver.find_common_areas(&clique);
        if !common_areas.is_empty() {
          continue
        }

        let max_set_size = 4.min(clique.len());
        for set_size in 2..=max_set_size {
          let ns = NakedSet::new(set_size);
          let steps = ns.run_in_area(Area::Adhoc(clique.clone()), solver);
          if steps.is_empty() {
            continue
          }
          return steps.into_iter().map(|step|
            self.build_solution_step(
              step.cells.into_iter().sorted().collect(),
              step.values,
              step.areas,
              step.affected_cells,
            )
          ).collect()
        }
      }
    }

    vec![]
  }
}

impl AdhocNakedSet {
  // https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm
  fn bron_kerbosch(&self, neighbour_masks: &Vec<u32>, clique: u32, mut peers: u32, mut banned: u32, cliques: &mut Vec<u32>) {
    if peers == 0 {
      if banned == 0 {
        cliques.push(clique);
      }
      return
    }

    for v in 0..neighbour_masks.len() {
      if (peers & (1 << v)) == 0 {
        continue
      }
      self.bron_kerbosch(
        neighbour_masks,
        clique | (1 << v),
        peers & neighbour_masks[v],
        banned & neighbour_masks[v],
        cliques
      );
      peers = peers ^ (1 << v);
      banned = banned | (1 << v);
    }
  }
}

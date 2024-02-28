use std::collections::HashSet;
use itertools::Itertools;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, CellPosition};
use super::technique::Technique;

// Phistomefel ring in its weakest form. Doesn't take many things into account.

pub struct PhistomefelRing;

impl Technique for PhistomefelRing {
  fn get_rule(&self) -> Rule { Rule::PhistomefelRing }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active || solver.constraints.grid_size != 9 {
      return vec![]
    }

    let set1 = vec![
      CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(1, 0), CellPosition::new(1, 1),
      CellPosition::new(0, 7), CellPosition::new(0, 8), CellPosition::new(1, 7), CellPosition::new(1, 8),
      CellPosition::new(7, 0), CellPosition::new(7, 1), CellPosition::new(8, 0), CellPosition::new(8, 1),
      CellPosition::new(7, 7), CellPosition::new(7, 8), CellPosition::new(8, 7), CellPosition::new(8, 8),
    ];
    let set2 = vec![
      CellPosition::new(2, 2), CellPosition::new(2, 3), CellPosition::new(2, 4), CellPosition::new(2, 5),
      CellPosition::new(2, 6),
      CellPosition::new(3, 2), CellPosition::new(3, 6),
      CellPosition::new(4, 2), CellPosition::new(4, 6),
      CellPosition::new(5, 2), CellPosition::new(5, 6),
      CellPosition::new(6, 2), CellPosition::new(6, 3), CellPosition::new(6, 4), CellPosition::new(6, 5),
      CellPosition::new(6, 6),
    ];
    let steps = self.match_sets(solver, set1, set2);

    steps
  }
}

impl PhistomefelRing {
  fn get_cell_values(&self, solver: &Solver, cell: &CellPosition) -> HashSet<u32> {
    let &CellPosition { row, col } = cell;
    if solver.grid[row][col] != 0 {
      HashSet::from([ solver.grid[row][col] ])
    } else {
      solver.candidates[row][col].to_owned()
    }
  }

  fn match_sets(&self, solver: &Solver, set1: Vec<CellPosition>, set2: Vec<CellPosition>) -> Vec<SolutionStep> {
    let res1 = self.match_sets_ordered(solver, &set1, &set2);
    let res2 = self.match_sets_ordered(solver, &set2, &set1);
    [res1, res2].concat()
  }

  fn match_sets_ordered(&self, solver: &Solver, set1: &Vec<CellPosition>, set2: &Vec<CellPosition>) -> Vec<SolutionStep> {
    set1.iter().flat_map(|cell1| {
      let values1 = self.get_cell_values(solver, cell1);
      let mut matching_values: HashSet<u32> = HashSet::new();
      let mut matching_cell_count = 0;
      let mut matching_cell: Option<&CellPosition> = None;
      for cell2 in set2 {
        let values2 = self.get_cell_values(solver, cell2);
        if !values1.is_disjoint(&values2) {
          matching_values.extend(values2);
          matching_cell_count += 1;
          matching_cell = Some(cell2);
        }
      }

      let mut steps = vec![];

      let extra_values: Vec<u32> = values1.difference(&matching_values).copied().sorted().collect();
      if !extra_values.is_empty() {
        let step = self.build_simple_solution_step(
          extra_values,
          vec![],
          vec![ *cell1 ],
        );
        steps.push(step);
      }

      if matching_cell_count == 1 {
        let extra_values: Vec<u32> = matching_values.difference(&values1).copied().sorted().collect();
        if !extra_values.is_empty() {
          let step = self.build_simple_solution_step(
            extra_values,
            vec![],
            vec![ *matching_cell.unwrap() ],
          );
          steps.push(step);
        }
      }

      steps
    }).collect()
  }
}

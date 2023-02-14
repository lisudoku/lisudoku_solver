use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, KropkiDot, Rule, Area, KropkiDotType};
use super::technique::Technique;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque, HashMap};
use std::ops::{BitAnd, BitXorAssign};

// X can't be a candidate in this cell because it violates the dot chain
pub struct KropkiChainCandidates {
  chain_limit: bool,
}

impl Technique for KropkiChainCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { if self.chain_limit { Rule::Kropki } else { Rule::KropkiChainCandidates } }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    let mut steps = self.find_kropki_candidate_updates(solver);

    if self.chain_limit {
      for step in steps.iter_mut() {
        step.rule = Rule::Kropki;
        step.areas.remove(0);
      }
    }

    steps
  }
}

impl KropkiChainCandidates {
  pub fn new(chain_limit: bool) -> KropkiChainCandidates {
    KropkiChainCandidates { chain_limit }
  }

  pub fn find_kropki_candidate_updates(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }
    if solver.constraints.kropki_dots.is_empty() {
      return vec![]
    }

    let mut steps_by_cell: HashMap<CellPosition, SolutionStep> = HashMap::new();

    for area in &solver.get_all_areas(false, false, false) {
      let mut dot_types = vec![ KropkiDotType::Consecutive, KropkiDotType::Double ];
      if self.chain_limit {
        // Kropki pairs handle negative dots, the rest handled by chains
        dot_types = vec![ KropkiDotType::Negative ];
      }
      for dot_type in dot_types {
        let area_steps = self.find_kropki_area_candidate_updates(solver, area, dot_type);

        for step in area_steps {
          let cell = step.affected_cells[0];
          if steps_by_cell.contains_key(&cell) {
            steps_by_cell.entry(cell)
                         .and_modify(|entry| {
                           if step.values.len() > entry.values.len() {
                             *entry = step;
                           }
                         });
          } else {
            steps_by_cell.insert(cell, step);
          }
        }
      }
    }

    let steps: Vec<SolutionStep> = steps_by_cell.into_values()
                                                .sorted_by_key(|step| step.affected_cells[0])
                                                .collect();

    steps
  }

  fn find_kropki_area_candidate_updates(&self, solver: &Solver, area: &Area, dot_type: KropkiDotType) -> Vec<SolutionStep> {
    let kropki_ccs = Self::compute_area_kropki_ccs(solver, area, dot_type, self.chain_limit);

    kropki_ccs.into_iter().flat_map(|(cells, indices)| {
      let invalid_candidates = self.find_kropki_ccs_invalid_candidate(solver, cells);

      let current_steps: Vec<SolutionStep> = invalid_candidates.into_iter().map(|(cell, invalid_values)| {
        let mut areas = vec![ *area ];
        areas.extend(&indices.iter().map(|&idx| Area::KropkiDot(idx)).collect::<Vec<Area>>());

        SolutionStep {
          rule: self.get_rule(),
          cells: vec![],
          values: invalid_values,
          areas,
          affected_cells: vec![ cell ],
          candidates: None,
        }
      }).collect();

      current_steps
    }).collect()
  }

  pub fn compute_area_kropki_ccs(solver: &Solver, area: &Area, dot_type: KropkiDotType, chain_limit: bool) -> Vec<(Vec<CellPosition>,Vec<usize>)> {
    let kc = KropkiComponents { solver };
    kc.run_area(area, dot_type, chain_limit)
  }

  fn find_kropki_ccs_invalid_candidate(&self, solver: &Solver, cells: Vec<CellPosition>) -> Vec<(CellPosition, Vec<u32>)> {
    let (valid_candidates, _) = Self::mark_kropki_valid_candidates(solver, &cells);
    let invalid_candidates = solver.cell_candidates_diff(&cells, valid_candidates);

    invalid_candidates
  }

  pub fn find_kropki_ccs_combinations(solver: &Solver, cells: &Vec<CellPosition>) -> Vec<Vec<u32>> {
    let (_, combinations_list) = Self::mark_kropki_valid_candidates(solver, cells);

    combinations_list
  }

  pub fn mark_kropki_valid_candidates(solver: &Solver, cells: &Vec<CellPosition>) -> (Vec<HashSet<u32>>, Vec<Vec<u32>>) {
    let mut combinations_runner = KropkiCandidateCombinations {
      used_candidates: vec![ 0; cells.len() ],
      used_candidates_set: 0,
      temp_grid: solver.grid.to_vec(),
      cells,
      valid_candidates: vec![ HashSet::new(); cells.len() ],
      combinations_list: vec![],
      solver,
    };

    combinations_runner.run(0);

    (combinations_runner.valid_candidates, combinations_runner.combinations_list)
  }
}

struct KropkiComponents<'a> {
  solver: &'a Solver,
}

impl KropkiComponents<'_> {
  fn run_area(&self, area: &Area, dot_type: KropkiDotType, chain_limit: bool) -> Vec<(Vec<CellPosition>,Vec<usize>)> {
    let area_cells = self.solver.get_area_cells(area);

    // Separate case when we are only interested in pairs
    if chain_limit {
      return self.compute_area_kropki_pairs_ccs(self.solver, area_cells, dot_type)
    }

    let mut ccs = vec![];
    let area_cells_set: HashSet<CellPosition> = area_cells.iter().copied().collect();
    let mut covered_cells = vec![ vec![ false; self.solver.constraints.grid_size ]; self.solver.constraints.grid_size ];
    for cell in &area_cells {
      if covered_cells[cell.row][cell.col] {
        continue
      }

      covered_cells[cell.row][cell.col] = true;

      let mut queue: VecDeque<CellPosition> = VecDeque::new();
      queue.push_back(*cell);
      let mut cc_cells = vec![ *cell ];
      let mut cc_indices: Vec<usize> = vec![];

      while !queue.is_empty() {
        let cell = queue.pop_front().unwrap();
        let dots: Vec<(usize, &KropkiDot)> = self.solver.grid_to_kropki_dots[cell.row][cell.col].iter().filter_map(|&dot_index| {
          let kropki_dot = &self.solver.constraints.kropki_dots[dot_index];
          if kropki_dot.dot_type == dot_type {
            Some((dot_index, kropki_dot))
          } else {
            None
          }
        }).collect();

        if dots.is_empty() {
          continue
        }

        for (dot_index, dot) in dots {
          let other_cell = dot.other_cell(&cell);
          if !area_cells_set.contains(&other_cell) {
            continue
          }
          if covered_cells[other_cell.row][other_cell.col] {
            continue
          }

          covered_cells[other_cell.row][other_cell.col] = true;
          queue.push_back(other_cell);
          cc_cells.push(other_cell);
          cc_indices.push(dot_index);
        }
      }

      if cc_cells.len() > 1 {
        ccs.push((cc_cells, cc_indices));
      }
    }

    ccs
  }

  fn compute_area_kropki_pairs_ccs(&self, solver: &Solver, area_cells: Vec<CellPosition>, dot_type: KropkiDotType) -> Vec<(Vec<CellPosition>,Vec<usize>)> {
    area_cells.into_iter().flat_map(|cell| {
      let dots: Vec<(usize, &KropkiDot)> = solver.grid_to_kropki_dots[cell.row][cell.col].iter().filter_map(|&dot_index| {
        let kropki_dot = &solver.constraints.kropki_dots[dot_index];
        if kropki_dot.dot_type == dot_type && kropki_dot.cell_1 < kropki_dot.cell_2 {
          Some((dot_index, kropki_dot))
        } else {
          None
        }
      }).collect();

      dots.into_iter().map(|(dot_index, dot)| {
        let cc_cells = vec![ dot.cell_1, dot.cell_2 ];
        let cc_indices = vec![ dot_index ];
        (cc_cells, cc_indices)
      }).collect::<Vec<(Vec<CellPosition>,Vec<usize>)>>()
    }).collect()
  }
}

struct KropkiCandidateCombinations<'a> {
  used_candidates: Vec<u32>,
  used_candidates_set: u32,
  temp_grid: Vec<Vec<u32>>,
  valid_candidates: Vec<HashSet<u32>>,
  combinations_list: Vec<Vec<u32>>,
  cells: &'a Vec<CellPosition>,
  solver: &'a Solver,
}

impl KropkiCandidateCombinations<'_> {
  fn run(&mut self, index: usize) {
    if index == self.cells.len() {
      for (cell_index, &candidate) in self.used_candidates.iter().enumerate() {
        self.valid_candidates[cell_index].insert(candidate);
      }
      self.combinations_list.push(self.used_candidates.to_vec());

      return
    }

    let cell = self.cells[index];
    if self.solver.grid[cell.row][cell.col] != 0 {
      self.run(index + 1);
      return
    }

    for &value in &self.solver.candidates[cell.row][cell.col] {
      if self.used_candidates_set.bitand(1 << value) > 0 {
        continue
      }
      if !self.cells.contains(&cell) {
        continue
      }
      if !self.check_kropki_value(&cell, value) {
        continue
      }

      self.temp_grid[cell.row][cell.col] = value;
      self.used_candidates[index] = value;
      self.used_candidates_set.bitxor_assign(1 << value);

      self.run(index + 1);

      self.temp_grid[cell.row][cell.col] = 0;
      self.used_candidates_set.bitxor_assign(1 << value);
    }
  }

  fn check_kropki_value(&self, cell: &CellPosition, value: u32) -> bool {
    for &kropki_dot_index in &self.solver.grid_to_kropki_dots[cell.row][cell.col] {
      let kropki_dot = &self.solver.constraints.kropki_dots[kropki_dot_index];
      let other_cell = kropki_dot.other_cell(cell);

      let other_value = self.temp_grid[other_cell.row][other_cell.col];
      if !kropki_dot.check_values(value, other_value) {
        return false
      }
    }
    true
  }
}

use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, KropkiDot, Rule, Area, KropkiDotType};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque, HashMap};
use std::ops::{BitAnd, BitXorAssign};

// X can't be a candidate in this cell because it violates the dot chain
impl Solver {
  pub fn find_kropki_chain_candidate_updates(&self) -> Vec<SolutionStep> {
    self.find_kropki_candidate_updates(false)
  }

  pub fn find_kropki_pair_candidate_updates(&self) -> Vec<SolutionStep> {
    let mut steps = self.find_kropki_candidate_updates(true);

    for step in steps.iter_mut() {
      step.rule = Rule::Kropki;
      step.areas.remove(0);
    }

    steps
  }

  pub fn find_kropki_candidate_updates(&self, chain_limit: bool) -> Vec<SolutionStep> {
    if !self.candidates_active {
      return vec![]
    }
    if self.constraints.kropki_dots.is_empty() {
      return vec![]
    }

    let mut steps_by_cell: HashMap<CellPosition, SolutionStep> = HashMap::new();

    for area in &self.get_all_areas(false, false, false) {
      let mut dot_types = vec![ KropkiDotType::Consecutive, KropkiDotType::Double ];
      if chain_limit {
        // Kropki pairs handle negative dots, the rest handled by chains
        dot_types = vec![ KropkiDotType::Negative ];
      }
      for dot_type in dot_types {
        let area_steps = self.find_kropki_area_candidate_updates(area, dot_type, chain_limit);

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

  fn find_kropki_area_candidate_updates(&self, area: &Area, dot_type: KropkiDotType, chain_limit: bool) -> Vec<SolutionStep> {
    let kropki_ccs = self.compute_area_kropki_ccs(area, dot_type, chain_limit);

    kropki_ccs.into_iter().flat_map(|(cells, indices)| {
      let invalid_candidates = self.find_kropki_ccs_invalid_candidate(cells);

      let current_steps: Vec<SolutionStep> = invalid_candidates.into_iter().map(|(cell, invalid_values)| {
        let mut areas = vec![ *area ];
        areas.extend(&indices.iter().map(|&idx| Area::KropkiDot(idx)).collect::<Vec<Area>>());

        SolutionStep {
          rule: Rule::KropkiChainCandidates,
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

  fn find_kropki_ccs_invalid_candidate(&self, cells: Vec<CellPosition>) -> Vec<(CellPosition, Vec<u32>)> {
    let (valid_candidates, _) = self.mark_kropki_valid_candidates(&cells);
    let invalid_candidates = self.cell_candidates_diff(&cells, valid_candidates);

    invalid_candidates
  }

  pub fn find_kropki_ccs_combinations(&self, cells: &Vec<CellPosition>) -> Vec<Vec<u32>> {
    let (_, combinations_list) = self.mark_kropki_valid_candidates(cells);

    combinations_list
  }

  pub fn compute_area_kropki_ccs(&self, area: &Area, dot_type: KropkiDotType, chain_limit: bool) -> Vec<(Vec<CellPosition>,Vec<usize>)> {
    let area_cells = self.get_area_cells(area);

    // Separate case when we are only interested in pairs
    if chain_limit {
      return self.compute_area_kropki_pairs_ccs(area_cells, dot_type)
    }

    let mut ccs = vec![];
    let area_cells_set: HashSet<CellPosition> = area_cells.iter().copied().collect();
    let mut covered_cells = vec![ vec![ false; self.constraints.grid_size ]; self.constraints.grid_size ];
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
        let dots: Vec<(usize, &KropkiDot)> = self.grid_to_kropki_dots[cell.row][cell.col].iter().filter_map(|&dot_index| {
          let kropki_dot = &self.constraints.kropki_dots[dot_index];
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

  fn compute_area_kropki_pairs_ccs(&self, area_cells: Vec<CellPosition>, dot_type: KropkiDotType) -> Vec<(Vec<CellPosition>,Vec<usize>)> {
    area_cells.into_iter().flat_map(|cell| {
      let dots: Vec<(usize, &KropkiDot)> = self.grid_to_kropki_dots[cell.row][cell.col].iter().filter_map(|&dot_index| {
        let kropki_dot = &self.constraints.kropki_dots[dot_index];
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

  fn mark_kropki_valid_candidates(&self, cells: &Vec<CellPosition>) -> (Vec<HashSet<u32>>, Vec<Vec<u32>>) {
    let mut temp_grid = self.grid.to_vec();
    let mut valid_candidates: Vec<HashSet<u32>> = vec![ HashSet::new(); cells.len() ];
    let mut used_candidates: Vec<u32> = vec![ 0; cells.len() ];
    let mut used_candidates_set: u32 = 0;
    let mut combinations_list: Vec<Vec<u32>> = vec![];

    self.generate_kropki_candidate_combinations(
      0, &mut used_candidates, &mut used_candidates_set, &mut temp_grid, cells,
      &mut valid_candidates, &mut combinations_list
    );

    (valid_candidates, combinations_list)
  }

  fn generate_kropki_candidate_combinations(
    &self, index: usize, used_candidates: &mut Vec<u32>, used_candidates_set: &mut u32,
    temp_grid: &mut Vec<Vec<u32>>, cells: &Vec<CellPosition>, valid_candidates: &mut Vec<HashSet<u32>>,
    combinations_list: &mut Vec<Vec<u32>>
  ) {
    if index == cells.len() {
      for (cell_index, &candidate) in used_candidates.iter().enumerate() {
        valid_candidates[cell_index].insert(candidate);
      }
      combinations_list.push(used_candidates.to_vec());

      return
    }

    let cell = cells[index];
    if self.grid[cell.row][cell.col] != 0 {
      self.generate_kropki_candidate_combinations(
        index + 1,
        used_candidates,
        used_candidates_set,
        temp_grid,
        cells,
        valid_candidates,
        combinations_list
      );
      return
    }

    for &value in &self.candidates[cell.row][cell.col] {
      if used_candidates_set.bitand(1 << value) > 0 {
        continue
      }
      if !cells.contains(&cell) {
        continue
      }
      if !self.check_kropki_value(&cell, value, temp_grid) {
        continue
      }

      temp_grid[cell.row][cell.col] = value;
      used_candidates[index] = value;
      used_candidates_set.bitxor_assign(1 << value);

      self.generate_kropki_candidate_combinations(
        index + 1,
        used_candidates,
        used_candidates_set,
        temp_grid,
        cells,
        valid_candidates,
        combinations_list
      );

      temp_grid[cell.row][cell.col] = 0;
      used_candidates_set.bitxor_assign(1 << value);
    }
  }

  fn check_kropki_value(&self, cell: &CellPosition, value: u32, temp_grid: &mut Vec<Vec<u32>>) -> bool {
    for &kropki_dot_index in &self.grid_to_kropki_dots[cell.row][cell.col] {
      let kropki_dot = &self.constraints.kropki_dots[kropki_dot_index];
      let other_cell = kropki_dot.other_cell(cell);

      let other_value = temp_grid[other_cell.row][other_cell.col];
      if !kropki_dot.check_values(value, other_value) {
        return false
      }
    }
    true
  }
}

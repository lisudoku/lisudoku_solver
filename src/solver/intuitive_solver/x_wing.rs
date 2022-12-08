use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area, CellPosition};

impl Solver {
  pub fn find_x_wing(&self) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None;
    }

    let rows = self.get_row_areas();
    let step = self.find_x_wing_in_areas(&rows);
    if step.is_some() {
      return step;
    }

    let cols = self.get_col_areas();
    let step = self.find_x_wing_in_areas(&cols);
    if step.is_some() {
      return step;
    }

    None
  }

  fn find_x_wing_in_areas(&self, areas: &Vec<Area>) -> Option<SolutionStep> {
    for area1 in areas {
      for (value, area1_cells) in self.compute_cells_by_value_in_area(area1, &self.candidates) {
        if area1_cells.len() != 2 {
          continue
        }

        let cell1 = area1_cells[0];
        let cell2 = area1_cells[1];

        for area2 in areas {
          if area2 == area1 {
            continue
          }
          let area2_cells: Vec<CellPosition> = self.get_area_cells_with_candidate(area2, value);

          let translated_cells: Vec<CellPosition> = area2_cells.iter()
            .copied()
            .map(|cell| self.cell_to_area(&cell, area1))
            .collect();
          
          if translated_cells != area1_cells {
            continue
          }

          let cell3 = area2_cells[0];
          let cell4 = area2_cells[1];

          assert!(self.candidates[cell3.row][cell3.col].contains(&value));
          assert!(self.candidates[cell4.row][cell4.col].contains(&value));

          let area3 = *self.find_common_areas(&vec![ cell1, cell3 ]).first().unwrap();
          let area4 = *self.find_common_areas(&vec![ cell2, cell4 ]).first().unwrap();
          let affected_cells: Vec<CellPosition> = vec![
            self.get_area_cells_with_candidate(&area3, value),
            self.get_area_cells_with_candidate(&area4, value),
          ].concat().into_iter().filter(|&cell| {
            cell != cell1 && cell != cell2 && cell != cell3 && cell != cell4
          }).collect();

          if affected_cells.is_empty() {
            continue
          }

          return Some(
            SolutionStep {
              rule: Rule::XWing,
              cells: vec![ cell1, cell2, cell3, cell4 ],
              values: vec![ value ],
              areas: vec![ *area1, *area2, area3, area4 ],
              affected_cells,
              candidates: None,
            }
          )
        }
      }
    }

    None
  }
}

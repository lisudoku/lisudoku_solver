use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area, CellPosition};
use super::technique::Technique;

pub struct XWing;

impl Technique for XWing {
  fn get_rule(&self) -> Rule { Rule::XWing }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let rows = solver.get_row_areas();
    let step = self.find_x_wing_in_areas(solver, &rows);
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let cols = solver.get_col_areas();
    let step = self.find_x_wing_in_areas(solver, &cols);
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    vec![]
  }
}

impl XWing {
  fn find_x_wing_in_areas(&self, solver: &Solver, areas: &Vec<Area>) -> Option<SolutionStep> {
    for area1 in areas {
      for (value, area1_cells) in solver.compute_cells_by_value_in_area(area1, &solver.candidates) {
        if area1_cells.len() != 2 {
          continue
        }

        let cell1 = area1_cells[0];
        let cell2 = area1_cells[1];

        for area2 in areas {
          if area2 == area1 {
            continue
          }
          let area2_cells: Vec<CellPosition> = solver.get_area_cells_with_candidate(area2, value);

          let translated_cells: Vec<CellPosition> = area2_cells.iter()
            .copied()
            .map(|cell| solver.cell_to_area(&cell, area1))
            .collect();
          
          if translated_cells != area1_cells {
            continue
          }

          let cell3 = area2_cells[0];
          let cell4 = area2_cells[1];

          assert!(solver.candidates[cell3.row][cell3.col].contains(&value));
          assert!(solver.candidates[cell4.row][cell4.col].contains(&value));

          let area3 = *solver.find_common_areas(&vec![ cell1, cell3 ]).first().unwrap();
          let area4 = *solver.find_common_areas(&vec![ cell2, cell4 ]).first().unwrap();
          let affected_cells: Vec<CellPosition> = vec![
            solver.get_area_cells_with_candidate(&area3, value),
            solver.get_area_cells_with_candidate(&area4, value),
          ].concat().into_iter().filter(|&cell| {
            cell != cell1 && cell != cell2 && cell != cell3 && cell != cell4
          }).collect();

          if affected_cells.is_empty() {
            continue
          }

          return Some(
            self.build_solution_step(
              vec![ cell1, cell2, cell3, cell4 ],
              vec![ value ],
              vec![ *area1, *area2, area3, area4 ],
              affected_cells,
            )
          )
        }
      }
    }

    None
  }
}

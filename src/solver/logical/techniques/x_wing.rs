use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, TechniquePhase}}, types::{Area, CellPosition, SolutionStep}};

#[derive(Clone, Debug)]
pub(crate) struct XWing;

impl TechniqueDefinition for XWing {
  fn kind(&self) -> TechniqueKind { TechniqueKind::XWing }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let Some(constraint) = solver.state.puzzle.region() else { return vec![] };

    let rows = constraint.row_areas();
    let step = self.find_x_wing_in_areas(solver, &rows);
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    let cols = constraint.col_areas();
    let step = self.find_x_wing_in_areas(solver, &cols);
    if step.is_some() {
      return vec![ step.unwrap() ]
    }

    vec![]
  }
}

impl XWing {
  fn find_x_wing_in_areas(&self, solver: &LogicalSolver, areas: &Vec<Area>) -> Option<SolutionStep> {
    for area1 in areas {
      let value_cells = solver.state.compute_cells_by_value_in_area(area1, &solver.state.candidates);
      for (value, area1_cells) in value_cells {
        if area1_cells.len() != 2 {
          continue
        }

        let cell1 = area1_cells[0];
        let cell2 = area1_cells[1];

        // TODO: continue from area1's index
        for area2 in areas {
          if area2 == area1 {
            continue
          }
          let area2_cells: Vec<CellPosition> = solver.state.area_cells_with_candidate(area2, value);

          let translated_cells: Vec<CellPosition> = area2_cells.iter()
            .copied()
            .map(|cell| area1.translate_cell_to(&cell))
            .collect();

          if translated_cells != area1_cells {
            continue
          }

          let cell3 = area2_cells[0];
          let cell4 = area2_cells[1];

          assert!(solver.state.candidates[cell3.row][cell3.col].contains(&value));
          assert!(solver.state.candidates[cell4.row][cell4.col].contains(&value));

          // TODO: only need to check rows&cols
          let area3 = solver.state.puzzle.find_common_region_areas(&vec![ cell1, cell3 ]).first().unwrap().clone();
          let area4 = solver.state.puzzle.find_common_region_areas(&vec![ cell2, cell4 ]).first().unwrap().clone();
          let affected_cells: Vec<CellPosition> = vec![
            solver.state.area_cells_with_candidate(&area3, value),
            solver.state.area_cells_with_candidate(&area4, value),
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
              vec![ area1.clone(), area2.clone(), area3, area4 ],
              affected_cells,
            )
          )
        }
      }
    }

    None
  }
}

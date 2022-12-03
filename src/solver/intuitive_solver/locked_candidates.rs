use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};

impl Solver {
  pub fn find_locked_candidates(&self) -> Option<SolutionStep> {    
    if !self.candidates_active {
      return None
    }

    let areas = self.get_all_areas();
    for area in areas {
      let value_cells = self.compute_cells_by_value_in_area(&area, &self.candidates);

      for (value, cells) in value_cells {
        // TODO: also check for triples
        if cells.len() != 2 {
          continue
        }

        let other_area = self.find_common_area_except(cells[0], cells[1], area);
        if other_area.is_none() {
          continue
        }
        let other_area = other_area.unwrap();

        let affected_cells: Vec<CellPosition> = self.get_area_cells_with_candidate(&other_area, value)
                                                    .into_iter()
                                                    .filter(|cell| !cells.contains(cell))
                                                    .collect();
        if !affected_cells.is_empty() {
          return Some(
            SolutionStep {
              rule: Rule::LockedCandidates,
              cells,
              values: vec![ value ],
              areas: vec![ other_area ],
              affected_cells,
              candidates: None,
            }
          )
        }
      }
    }

    return None
  }

  fn find_common_area_except(&self, cell1: CellPosition, cell2: CellPosition, area_exception: Area) -> Option<Area> {
    let areas = self.find_common_areas(cell1, cell2);
    let other_areas: Vec<Area> = areas.into_iter().filter(|area| *area != area_exception).collect();
    assert!(other_areas.len() <= 1);
    other_areas.get(0).copied()
  }

  fn find_common_areas(&self, cell1: CellPosition, cell2: CellPosition) -> Vec<Area> {
    let mut areas = vec![];
    if cell1.row == cell2.row {
      areas.push(Area::Row(cell1.row));
    }
    if cell1.col == cell2.col {
      areas.push(Area::Column(cell1.col));
    }
    let region1 = self.grid_to_region[cell1.row][cell1.col];
    let region2 = self.grid_to_region[cell2.row][cell2.col];
    if region1 == region2 {
      areas.push(Area::Region(region1));
    }
    areas
  }
}

use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{StepApplication, TechniqueDefinition, TechniqueKind, TechniquePhase}}, types::{Area, SolutionStep}};
use super::thermo_candidate_update::ThermoCandidateUpdate;

// This is a weaker version of ThermoCandidateUpdate to apply it before running CandidateInitialization
// The idea is to have more granularity to separate easy vs medium, not sure how useful it is.

#[derive(Clone, Debug)]
pub(crate) struct ThermoNakedSingle;

impl TechniqueDefinition for ThermoNakedSingle {
  fn kind(&self) -> TechniqueKind { TechniqueKind::ThermoNakedSingle }

  fn phase(&self) -> TechniquePhase { TechniquePhase::Grid }

  fn step_application(&self) -> StepApplication { StepApplication::SetCellValue }

  fn complexity(&self, _step: &SolutionStep, _solver: &LogicalSolver) -> u32 { 2 }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    let Some(constraint) = solver.state.puzzle.thermo() else { return vec![] };

    for (thermo_index, thermo) in constraint.thermos.iter().enumerate() {
      // TODO: duplicate logic in constraint definition
      let lower_bounds = ThermoCandidateUpdate::find_thermo_lower_bounds(solver, &thermo);
      let upper_bounds = ThermoCandidateUpdate::find_thermo_upper_bounds(solver, &thermo);

      for (cell_index, cell) in thermo.iter().enumerate() {
        let valid_values: Vec<u32> = solver.state.compute_cell_candidates(cell)
          .into_iter()
          .filter(|&value| lower_bounds[cell_index] <= value && value <= upper_bounds[cell_index])
          .collect();

        if valid_values.len() == 1 {
          return vec![
            self.build_grid_solution_step(
              vec![ *cell ],
              valid_values,
              vec![ Area::Thermo(thermo_index) ],
              &solver,
            )
          ]
        }
      }
    }

    vec![]
  }
}

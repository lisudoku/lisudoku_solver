use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{StepApplication, TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::logical::techniques::combinations::cell_combinations_runner::CellCombinationsRunner;
use crate::solver::types::CandidateSet;
use crate::types::{SolutionStep, Area};
use super::common_peer_elimination::CommonPeerElimination;

// Eliminate 1 candidate from a cell because all arrow combinations would eliminate it
// Normal locked candidates don't pick this up because they think ALL candidate combinations are possible

#[derive(Clone, Debug)]
pub struct ArrowCommonPeerElimination;

impl TechniqueDefinition for ArrowCommonPeerElimination {
  fn kind(&self) -> TechniqueKind { TechniqueKind::ArrowCommonPeerElimination }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn step_application(&self) -> StepApplication {
    StepApplication::RemoveCorrespondingCandidates
  }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let Some(constraint) = solver.state.puzzle.arrow() else { return vec![] };
    if constraint.arrows.is_empty() {
      return vec![]
    }

    let mut steps: Vec<SolutionStep> = vec![];
    let grid_size = solver.state.puzzle.grid_size;
    let mut cell_previous_invalid_values: Vec<CandidateSet> = vec![CandidateSet::new(); grid_size * grid_size];

    for (arrow_index, arrow) in constraint.arrows.iter().enumerate() {
      let cells = arrow.all_cells();

      let mut arrow_combinatons_logic_factory = solver.caches.arrow_combinatons_logic_factory.borrow_mut();
      let combination_logic = arrow_combinatons_logic_factory.create(arrow, solver);
      let mut runner = CellCombinationsRunner::new(solver, Box::new(combination_logic));
      let (_, combinations) = runner.run();

      let cell_eliminations = CommonPeerElimination::find_cell_eliminations(cells, combinations, solver);

      if cell_eliminations.is_empty() {
        continue
      }

      let (cells, values): (Vec<_>, Vec<_>) = solver
        .deduplicate_flat_eliminations(cell_eliminations, &mut cell_previous_invalid_values)
        .unzip();

      steps.push(
        self.build_simple_solution_step(
          values,
          vec![Area::Arrow(arrow_index)],
          cells,
        )
      );
    }

    steps
  }
}

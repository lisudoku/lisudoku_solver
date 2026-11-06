use crate::{solver::{SolveState, logical::{logical_solver::LogicalSolver, techniques::TechniqueKind}, types::CandidateSet}, types::{CellPosition, Grid, SolutionType, SudokuBruteSolveResult, SudokuConstraints}};
use std::mem;
use rand::seq::SliceRandom;

pub struct BruteSolverSettings {
  pub use_logic: bool,
  pub randomize: bool,
  pub solution_count_limit: u32,
}

impl Default for BruteSolverSettings {
  fn default() -> Self {
    Self {
      use_logic: false,
      randomize: false,
      solution_count_limit: 2,
    }
  }
}

pub struct BruteSolver {
  settings: BruteSolverSettings,
  state: SolveState,
  solution: Option<Grid>,
  solution_count: u32,
  logical_solver: LogicalSolver,
}

impl BruteSolver {
  pub fn new(input_constraints: SudokuConstraints) -> BruteSolver {
    let mut state = SolveState::new(input_constraints);
    state.performance_mode = true;

    let mut technique_kinds: Vec<TechniqueKind> = vec![TechniqueKind::NakedSingle, TechniqueKind::HiddenSingle];
    // TODO: automatically detect these
    let mut extra_technique_kinds: Vec<TechniqueKind> = vec![];
    if state.puzzle.renban().is_some() {
      extra_technique_kinds.push(TechniqueKind::RenbanCandidateUpdate);
    }
    if state.puzzle.top_bottom().is_some() {
      extra_technique_kinds.push(TechniqueKind::TopBottomCandidateUpdate);
    }
    if state.puzzle.palindrome().is_some() {
      extra_technique_kinds.push(TechniqueKind::PalindromeValue);
      extra_technique_kinds.push(TechniqueKind::PalindromeCandidateUpdate);
    }
    if state.puzzle.kropki().is_some() {
      extra_technique_kinds.push(TechniqueKind::KropkiChainCandidateUpdate);
      // TODO: KropkiAdvancedCandidateUpdate very slow, not enabling for now
    }
    if state.puzzle.killer().is_some() {
      extra_technique_kinds.push(TechniqueKind::KillerCandidateUpdate);
      extra_technique_kinds.push(TechniqueKind::Killer45);
    }
    if state.puzzle.arrow().is_some() {
      extra_technique_kinds.push(TechniqueKind::ArrowCandidateUpdate);
      // TODO: ArrowAdvancedCandidateUpdate very slow, not enabling for now
    }
    if state.puzzle.thermo().is_some() {
      extra_technique_kinds.push(TechniqueKind::ThermoCandidateUpdate);
    }

    if !extra_technique_kinds.is_empty() {
      technique_kinds.push(TechniqueKind::CandidateInitialization);
      technique_kinds.extend(extra_technique_kinds);
    }

    let logical_solver = LogicalSolver::from_solve_state(state.clone())
      .with_techniques_order(technique_kinds)
      .with_performance_mode();

    BruteSolver{
      state: state.clone(),
      settings: BruteSolverSettings::default(),
      solution: None,
      solution_count: 0,
      logical_solver,
    }
  }

  pub fn with_logic(mut self) -> BruteSolver {
    self.settings.use_logic = true;
    self
  }

  pub fn with_solution_limit(mut self, limit: u32) -> BruteSolver {
    self.settings.solution_count_limit = limit;
    self
  }

  pub fn with_randomize(mut self) -> BruteSolver {
    self.settings.randomize = true;
    self
  }

  pub fn solve(&mut self) -> SudokuBruteSolveResult {
    self.recursive_check(1);

    SudokuBruteSolveResult {
      solution_count: self.solution_count,
      solution: self.solution.clone(),
    }
  }

  fn recursive_check(&mut self, depth: u32) {
    let mut best_cell: Option<CellPosition> = None;
    let mut best_candidates = CandidateSet::new();

    if !self.state.any_empty_cells() {
      if !self.settings.use_logic {
        // This is needed for constraints that do not have restrict_cell_special_candidates()
        // and only enforce all restrictions through techniques
        let check = self.state.check_all_grid_special_validity();
        if !check.solved {
          return
        }
      }
      if self.solution.is_none() {
        self.solution = Some(self.state.grid().clone());
      }
      self.solution_count += 1;
      return
    }

    let mut logic_cells: Option<Vec<CellPosition>> = None;
    if self.settings.use_logic {
      self.logical_solver.state = mem::take(&mut self.state);
      self.logical_solver.state.candidates_active = false;

      let result = self.logical_solver.solve();

      self.state = mem::take(&mut self.logical_solver.state);

      logic_cells = Some(
        result.steps
          .into_iter()
          .filter(|step| step.is_grid_step() && step.invalid_state_reason.is_none())
          .map(|step| step.cells[0])
          .collect()
      );

      if result.solution_type == SolutionType::None {
        for cell in logic_cells.unwrap() {
          self.state.clear_value(&cell);
        }
        return
      }
      if result.solution_type == SolutionType::Full {
        if self.solution.is_none() {
          self.solution = Some(self.state.grid().clone());
        }
        self.solution_count += 1;

        for cell in logic_cells.unwrap() {
          self.state.clear_value(&cell);
        }

        return
      }
    }

    for cell in self.state.all_empty_cells() {
      // Note: don't use recompute_cell_candidates here because we are actually
      // reusing the freshly computed candidates from the logical solver (if enabled)
      let cell_candidates = self.state.compute_cell_candidates(&cell);
      if best_cell.is_none() || cell_candidates.len() < best_candidates.len() {
        best_cell = Some(cell);
        best_candidates = cell_candidates;
        if best_candidates.len() <= 1 {
          break
        }
      }
    }

    if !best_candidates.is_empty() {
      let cell = best_cell.unwrap();
      let mut values: Vec<u32> = best_candidates.into_iter().collect();

      if self.settings.randomize && values.len() > 1 {
        values.shuffle(&mut rand::rng());
      }
      for value in values {
        self.state.set_value(&cell, value);

        self.recursive_check(depth + 1);

        self.state.clear_value(&cell);

        if self.solution_count >= self.settings.solution_count_limit {
          break
        }
      }
    }

    if self.settings.use_logic && logic_cells.is_some() {
      // Sync back grid after logical solver run
      for cell in logic_cells.unwrap() {
        self.state.clear_value(&cell);
      }
    }
  }
}

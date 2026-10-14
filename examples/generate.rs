#![allow(unused)]
use std::collections::HashSet;
use std::mem::swap;
// use std::rc::Rc;
use itertools::Itertools;
use lisudoku_solver::solver::{Solver, BruteSolveSettings, /* Technique, ArrowCandidates, Candidates, EmptyRectangles, HiddenSet, HiddenSingles, Killer45, KillerCandidates, KropkiChainCandidates, LockedCandidates, NakedSet, NakedSingle, NishioForcingChains, PalindromeCandidates, PalindromeValues, RenbanCandidates, Thermo, ThermoCandidates, TopBottomCandidates, TurbotFish */};
use lisudoku_solver::types::{Arrow, CellDirection, CellPosition, Grid, KillerCage, KropkiDot, Palindrome, Region, Renban, SolutionType, SudokuConstraints};
use rand::RngExt;
use rand::seq::{IndexedRandom, SliceRandom};
use url::Url;

fn main() {
    // generate_classic(&SudokuConstraints::new(9));
    // generate_kropki(&SudokuConstraints::new(9));
    // generate_kropki(&SudokuConstraints::new(9).with_kropki_negative());
    // generate_kropki(&SudokuConstraints::new(6));
    // generate_palindrome(&SudokuConstraints::new(9));
    // generate_palindrome(&SudokuConstraints::new(6));
    // generate_palindrome(&SudokuConstraints::new(4));
    // generate_thermo(&SudokuConstraints::new(9));
    // generate_thermo(&SudokuConstraints::new(6));
    // generate_irregular(&SudokuConstraints::new(9));
    // generate_odd_even(&SudokuConstraints::new(9));
    // generate_renban(&SudokuConstraints::new(9));
    // generate_renban(&SudokuConstraints::new(6));
    // generate_renban(&SudokuConstraints::new(4));
    // generate_killer(&SudokuConstraints::new(9));
    // generate_killer(&SudokuConstraints::new(6));
    // generate_killer(&SudokuConstraints::new(4));
    // generate_extra_regions(&SudokuConstraints::new(9));
    // generate_extra_regions(&SudokuConstraints::new(6));
    // generate_anti_knight(&SudokuConstraints::new(9));
    // generate_anti_king(&SudokuConstraints::new(6));
    // generate_diagonal(&SudokuConstraints::new(9));
    // generate_top_bottom(&SudokuConstraints::new(9));
    // generate_top_bottom(&SudokuConstraints::new(6));
    // generate_arrow(&SudokuConstraints::new(9));
    generate_arrow(&SudokuConstraints::new(6));
}

fn random_grid(constraints: &SudokuConstraints) -> Grid {
  let mut solver = Solver::new(constraints.clone())
  // TODO: on slow cases profile and try with even less techniques?
    // .without_techniques(vec![Rc::new(NishioForcingChains), Rc::new(TurbotFish), Rc::new(EmptyRectangles)])
  ;
  // TODO: why use-Logic true, where is it false? I think I tried it and it didn't make a difference?
  let result = solver.brute_solve(BruteSolveSettings{ use_logic: true, randomize: true, solution_count_limit: Some(1) });
  assert!(result.solution.is_some());
  let Some(grid) = result.solution else { panic!("no solution?!") };
  grid
}

// fn easy_techniques() -> Vec<Rc<dyn Technique>> {
//   vec![Rc::new(NakedSingle), Rc::new(HiddenSingles), Rc::new(Thermo), Rc::new(PalindromeValues)]
// }

// fn easy_medium_techniques() -> Vec<Rc<dyn Technique>> {
//   vec![
//     Rc::new(NakedSingle), Rc::new(HiddenSingles), Rc::new(Thermo), Rc::new(PalindromeValues),
//     Rc::new(ThermoCandidates),
//     Rc::new(KillerCandidates),
//     Rc::new(KropkiChainCandidates::new(false)),
//     Rc::new(KropkiChainCandidates::new(true)),
//     Rc::new(TopBottomCandidates::new(false)),
//     Rc::new(ArrowCandidates),
//     Rc::new(RenbanCandidates),
//     Rc::new(PalindromeCandidates),
//     Rc::new(Candidates),
//     Rc::new(Killer45),
//     Rc::new(LockedCandidates::new(2)),
//     Rc::new(NakedSet::new(2)),
//     Rc::new(HiddenSet::new(2)),
//     // ???
//   ]
// }

fn is_constraints_solvable(constraints: &SudokuConstraints) -> bool {
  let mut logical_solver = Solver::new(constraints.clone())
  // .without_techniques(vec![Rc::new(NishioForcingChains), Rc::new(TurbotFish), /*Rc::new(EmptyRectangles)*/])
  // temp for EASY
  // .with_techniques(easy_techniques())
  // .with_techniques(easy_medium_techniques())
  ;
  let logical_solution = logical_solver.logical_solve();
  logical_solution.solution_type == SolutionType::Full
}

fn is_grid_solvable(grid: &Grid, constraints: &SudokuConstraints) -> bool {
  is_constraints_solvable(
    &constraints.clone().with_fixed_numbers_grid(grid)
  )
}

fn is_odd_even_solvable(restricted_cells: &Vec<CellPosition>, constraints: &SudokuConstraints, full_grid: &Grid) -> bool {
  is_constraints_solvable(
    &constraint_with_odd_even_cells(constraints.clone(), &restricted_cells, &full_grid)
  )
}

fn is_extra_regions_solvable(extra_regions: &Vec<Region>, constraints: &SudokuConstraints) -> bool {
  let mut logical_solver = Solver::new(
    constraints.clone().with_extra_regions(extra_regions.clone())
  )
  // .without_techniques(vec![Rc::new(NishioForcingChains), Rc::new(TurbotFish), Rc::new(EmptyRectangles)])
  ;
  let logical_solution = logical_solver.logical_solve();
  logical_solution.solution_type == SolutionType::Full
}

fn is_thermo_solvable(thermos: &Vec<lisudoku_solver::types::Thermo>, constraints: &SudokuConstraints) -> bool {
  let mut logical_solver = Solver::new(
    constraints.clone().with_thermos(thermos.clone())
  )
  // .without_techniques(vec![Rc::new(NishioForcingChains), Rc::new(TurbotFish), Rc::new(EmptyRectangles)])
  ;
  let logical_solution = logical_solver.logical_solve();
  logical_solution.solution_type == SolutionType::Full
}

fn is_renban_solvable(renbans: &Vec<Renban>, constraints: &SudokuConstraints) -> bool {
  is_constraints_solvable(
    &constraints.clone().with_renbans(renbans.clone())
  )
}

fn is_arrow_solvable(arrows: &Vec<Arrow>, constraints: &SudokuConstraints) -> bool {
  is_constraints_solvable(
    &constraints.clone().with_arrows(arrows.clone())
  )
}

fn generate_classic_constraints(existing_constraints: &SudokuConstraints) -> SudokuConstraints {
  let mut grid = random_grid(existing_constraints);
  println!("{}\n", grid.to_string(Some("\n")));

  simplify_grid_with_constraints(&mut grid, existing_constraints);

  assert!(is_grid_solvable(&grid, existing_constraints));

  existing_constraints.clone().with_fixed_numbers_grid(&grid)
}

fn generate_classic(existing_constraints: &SudokuConstraints) {
  let constraints = generate_classic_constraints(existing_constraints);
  println!("{}", constraints_to_url(&constraints));
}

fn is_kropki_solvable(kropki_dots: &Vec<KropkiDot>, constraints: &SudokuConstraints) -> bool {
  let mut logical_solver = Solver::new(
    constraints.clone().with_kropki_dots(kropki_dots.clone())
  )
  // .without_techniques(vec![Rc::new(NishioForcingChains), Rc::new(TurbotFish), Rc::new(EmptyRectangles)])
  ;
  let logical_solution = logical_solver.logical_solve();
  logical_solution.solution_type == SolutionType::Full
}

fn generate_kropki(existing_constraints: &SudokuConstraints) {
  let mut grid = random_grid(&existing_constraints.clone().unset_kropki_negative());
  println!("{}\n", grid.to_string(Some("\n")));

  let mut full_kropki_dots: Vec<KropkiDot> = vec![];
  // CellPosition::all_cells_in_grid_size
  for row in 0..existing_constraints.grid_size {
    for col in 0..existing_constraints.grid_size {
      if col + 1 < existing_constraints.grid_size {
        let mut val1 = grid[row][col];
        let mut val2 = grid[row][col + 1];
        if val1 < val2 {
          swap(&mut val1, &mut val2);
        }
        if val1 - val2 == 1 {
          full_kropki_dots.push(
            KropkiDot::consecutive(
              CellPosition::new(row, col),
              CellPosition::new(row, col + 1),
            )
          );
        } else if val2 * 2 == val1 {
          full_kropki_dots.push(
            KropkiDot::double(
              CellPosition::new(row, col),
              CellPosition::new(row, col + 1),
            )
          );
        }
      }
      if row + 1 < existing_constraints.grid_size {
        let mut val1 = grid[row][col];
        let mut val2 = grid[row + 1][col];
        if val1 < val2 {
          swap(&mut val1, &mut val2);
        }
        if val1 - val2 == 1 {
          full_kropki_dots.push(
            KropkiDot::consecutive(
              CellPosition::new(row, col),
              CellPosition::new(row + 1, col),
            )
          );
        } else if val2 * 2 == val1 {
          full_kropki_dots.push(
            KropkiDot::double(
              CellPosition::new(row, col),
              CellPosition::new(row + 1, col),
            )
          );
        }
      }
    }
  }

  println!("{}\n", grid.to_string(Some("\n")));
  // for kropki_dot in &full_kropki_dots {
  //   for cell in [kropki_dot.cell_1, kropki_dot.cell_2] {
  //     grid[cell.row][cell.col] = 0;
  //   }
  // }
  // println!("{}\n", grid.to_string(Some("\n")));

  let constraints = existing_constraints.clone()
    .with_kropki_dots(full_kropki_dots.clone())
    .with_fixed_numbers_grid(&grid)
    ;
  println!("{}", constraints_to_url(&constraints));

  assert!(is_kropki_solvable(&full_kropki_dots, &constraints));

  simplify_grid_with_constraints(&mut grid, &constraints);

  let constraints = constraints.with_fixed_numbers_grid(&grid);

  if constraints.kropki_negative.unwrap_or_default() {
    assert!(!is_kropki_solvable(&full_kropki_dots, &constraints.clone().unset_kropki_negative()), "negative flag should matter");

    println!("{}", constraints_to_url(&constraints));

    return
  }

  println!("kropki count: {}", full_kropki_dots.len());

  let mut minimal_kropki_dots: Vec<KropkiDot> = full_kropki_dots.clone();
  minimal_kropki_dots.shuffle(&mut rand::rng());
  for without_index in (0..minimal_kropki_dots.len()).rev() {
    let kropki_dot = minimal_kropki_dots.remove(without_index);
    if is_kropki_solvable(&minimal_kropki_dots, &constraints) {
      println!("SUCCESS!");
    } else {
      minimal_kropki_dots.insert(without_index, kropki_dot);
    }
  }

  println!("final kropki count: {}", minimal_kropki_dots.len());

  let constraints = constraints.with_kropki_dots(minimal_kropki_dots);
  println!("{}", constraints_to_url(&constraints));
}

// Copied? similar to otrhogonal_neighbours
const ADJACENT_MOVES: [CellDirection; 8] = [
  CellDirection { row: -1, col: -1 },
  CellDirection { row: -1, col: 0 },
  CellDirection { row: -1, col: 1 },
  CellDirection { row: 0, col: -1 },
  CellDirection { row: 0, col: 1 },
  CellDirection { row: 1, col: -1 },
  CellDirection { row: 1, col: 0 },
  CellDirection { row: 1, col: 1 },
];

pub fn neighbours(cell: &CellPosition, grid_size: usize) -> Vec<CellPosition> {
  ADJACENT_MOVES.iter().filter_map(|direction| {
    let prow = cell.row as isize + direction.row;
    let pcol = cell.col as isize + direction.col;
    if prow < 0 || prow >= grid_size as isize ||
        pcol < 0 || pcol >= grid_size as isize {
      return None
    }
    let peer = CellPosition {
      row: prow as usize,
      col: pcol as usize,
    };
    Some(peer)
  }).collect()
}

fn search_palindrome(cell_1: &CellPosition, cell_2: &CellPosition, grid: &Grid) -> Vec<CellPosition> {
  let mut visited_cells: HashSet<CellPosition> = HashSet::new();
  visited_cells.insert(*cell_1);
  visited_cells.insert(*cell_2);

  let mut palindrome: Vec<CellPosition> = Vec::from_iter(visited_cells.iter().copied());
  let mut found = true;
  while found {
    let a = palindrome.first().unwrap();
    let b = palindrome.last().unwrap();

    found = false;
    for (pa, pb) in neighbours(a, grid.len()).into_iter().cartesian_product(neighbours(b, grid.len()).into_iter()) {
      if pa != pb &&
        !visited_cells.contains(&pa) &&
        !visited_cells.contains(&pb) &&
        grid[pa.row][pa.col] == grid[pb.row][pb.col] &&
        // (pa.row == pb.row || pa.col == pb.col || !visited_cells.contains(&CellPosition::new(pa.row, pb.col)) || !visited_cells.contains(&CellPosition::new(pb.row, pa.col)))
        (pa.row == a.row || pa.col == a.col || !visited_cells.contains(&CellPosition::new(a.row, pa.col)) || !visited_cells.contains(&CellPosition::new(pa.row, a.col))) &&
        (pb.row == b.row || pb.col == b.col || !visited_cells.contains(&CellPosition::new(b.row, pb.col)) || !visited_cells.contains(&CellPosition::new(pb.row, b.col)))
      {
        palindrome.insert(0, pa);
        palindrome.push(pb);
        visited_cells.insert(pa);
        visited_cells.insert(pb);
        found = true;
        break
      }
    }
  }

  palindrome
}

fn palindrome_lone_cell_count(pal: &Vec<CellPosition>) -> usize {
  pal
    .into_iter()
    .enumerate()
    .filter(|(index, cell)| {
      let bad = cell.otrhogonal_neighbours(9);
      pal.iter().enumerate().all(|(i, c)| index.abs_diff(i) <= 1 || !bad.contains(c))
    })
    .count()
}

fn palindrome_is_better(pal_a: &Vec<CellPosition>, pal_b: &Vec<CellPosition>) -> bool {
  pal_a.len() + palindrome_lone_cell_count(pal_a) > pal_b.len() + palindrome_lone_cell_count(pal_b)
}

fn generate_palindrome(existing_constraints: &SudokuConstraints) {
  let mut grid = random_grid(existing_constraints);
  println!("{}\n", grid.to_string(Some("\n")));

  let mut longest_palindrome: Vec<CellPosition> = vec![];

  let upper_bound = if existing_constraints.grid_size == 9 { 20 }
    else if existing_constraints.grid_size == 6 { 14 }
    else if existing_constraints.grid_size == 4 { 10 }
    else { panic!() };

  // CellPosition::all_cells_in_grid_size
  for row in 0..existing_constraints.grid_size {
    for col in 0..existing_constraints.grid_size {
      let cell = CellPosition::new(row, col);
      let palindrome = search_palindrome(&cell, &cell, &grid);
      println!("odd palindrome length {}", palindrome.len());
      if palindrome.len() < upper_bound && palindrome_is_better(&palindrome, &longest_palindrome) {
        longest_palindrome = palindrome;
      }

      for peer in neighbours(&cell, existing_constraints.grid_size) {
        if grid[cell.row][cell.col] == grid[peer.row][peer.col] {
          let palindrome = search_palindrome(&cell, &peer, &grid);
          println!("even palindrome length {}", palindrome.len());
          if palindrome.len() < upper_bound && palindrome_is_better(&palindrome, &longest_palindrome) {
            longest_palindrome = palindrome;
          }
        }
      }
    }
  }

  println!("longest palindrome length {}", longest_palindrome.len());

  if existing_constraints.grid_size > 4 {
    for cell in &longest_palindrome {
      grid[cell.row][cell.col] = 0;
    }
  }

  let mut constraints = existing_constraints
    .clone()
    .with_palindromes(vec![Palindrome(longest_palindrome.clone())]);

  println!("{}", constraints_to_url(&constraints));

  simplify_grid_with_constraints(&mut grid, &constraints);

  constraints = constraints.with_fixed_numbers_grid(&grid);

  println!("{}", constraints_to_url(&constraints));
}

fn constraints_to_url(constraints: &SudokuConstraints) -> Url {
  let constraints_json = serde_json::to_string(constraints).unwrap();
  let mut url = Url::parse("https://lisudoku.github.io/sudoku_formats/redirect_encode").expect("invalid url");
  url.query_pairs_mut()
    .append_pair("constraints", &constraints_json)
    .append_pair("redirect_url", &format!("https://lisudoku.xyz/admin/build/{}?import=", constraints.grid_size));
  url
}

fn simplify_grid_with_constraints(grid: &mut Grid, constraints: &SudokuConstraints) {
  println!("Simplifying grid...");
  assert!(is_grid_solvable(&grid, constraints));

  let mut cells: Vec<CellPosition> = vec![];
  // CellPosition::all_cells_in_grid_size
  for row in 0..constraints.grid_size {
    for col in 0..constraints.grid_size {
      if grid[row][col] != 0 {
        cells.push(CellPosition::new(row, col));
      }
    }
  }
  cells.shuffle(&mut rand::rng());

  for CellPosition { row, col } in cells {
    let value = grid[row][col];
    assert_ne!(value, 0);
    grid[row][col] = 0;
    if is_grid_solvable(&grid, constraints) {
      println!("SUCCESS!");
      println!("{}\n", grid.to_string(Some("\n")));
      println!("{}", constraints_to_url(&constraints.clone().with_fixed_numbers_grid(grid)));
    } else {
      println!("FAIL!");
      grid[row][col] = value;
    }
  }

  assert!(is_grid_solvable(&grid, constraints));
}

fn search_thermo(start_cell: &CellPosition, grid: &Grid, visited_cells: &HashSet<CellPosition>) -> Vec<CellPosition> {
  let mut cell: CellPosition = *start_cell;
  let mut cells: Vec<CellPosition> = vec![cell];
  let mut found = true;
  while found {
    found = false;
    for peer in neighbours(&cell, grid.len()) {
      if grid[cell.row][cell.col] > grid[peer.row][peer.col] && !visited_cells.contains(&peer) &&
        (
          cell.row == peer.row ||
          cell.col == peer.col ||
          !(visited_cells.contains(&CellPosition { row: cell.row, col: peer.col }) || cells.contains(&CellPosition { row: cell.row, col: peer.col })) ||
          !(visited_cells.contains(&CellPosition { row: peer.row, col: cell.col }) || cells.contains(&CellPosition { row: peer.row, col: cell.col }))
        ) {
        cell = peer;
        cells.push(cell);
        found = true;
        break
      }
    }
  }
  cells.reverse();

  cell = *start_cell;
  found = true;
  while found {
    found = false;
    for peer in neighbours(&cell, grid.len()) {
      if grid[cell.row][cell.col] < grid[peer.row][peer.col] && !visited_cells.contains(&peer) &&
        (
          cell.row == peer.row ||
          cell.col == peer.col ||
          !(visited_cells.contains(&CellPosition { row: cell.row, col: peer.col }) || cells.contains(&CellPosition { row: cell.row, col: peer.col })) ||
          !(visited_cells.contains(&CellPosition { row: peer.row, col: cell.col }) || cells.contains(&CellPosition { row: peer.row, col: cell.col }))
        ) {
        cell = peer;
        cells.push(cell);
        found = true;
        break
      }
    }
  }

  cells
}

fn generate_thermo(existing_constraints: &SudokuConstraints) {
  let mut grid = random_grid(existing_constraints);
  println!("{}\n", grid.to_string(Some("\n")));

  let mut thermos: Vec<lisudoku_solver::types::Thermo> = vec![];
  let mut visited_cells: HashSet<CellPosition> = HashSet::new();

  for start_cell in CellPosition::all_cells_in_grid_size(existing_constraints.grid_size) {
    if visited_cells.contains(&start_cell) {
      continue
    }

    let thermo = search_thermo(&start_cell, &grid, &visited_cells);
    if thermo.len() <= 3 {
      continue
    }

    visited_cells.extend(thermo.clone());
    thermos.push(lisudoku_solver::types::Thermo(thermo));
  }

  let mut constraints = existing_constraints
    .clone()
    .with_thermos(thermos.clone());

  simplify_grid_with_constraints(&mut grid, &constraints);

  constraints = constraints.with_fixed_numbers_grid(&grid);

  println!("thermo count: {}", thermos.len());

  // TODO: rename below from renban to thermo!
  let mut minimal_renbans: Vec<lisudoku_solver::types::Thermo> = thermos.clone();
  minimal_renbans.shuffle(&mut rand::rng());
  for without_index in (0..minimal_renbans.len()).rev() {
    let thermo = minimal_renbans.remove(without_index);
    if is_thermo_solvable(&minimal_renbans, &constraints) {
      println!("SUCCESS!");
    } else {
      minimal_renbans.insert(without_index, thermo);
    }
  }

  println!("final thermo count: {}", minimal_renbans.len());

  let constraints = constraints.with_thermos(minimal_renbans);
  println!("{}", constraints_to_url(&constraints));
}

fn generate_irregular(existing_constraints: &SudokuConstraints) {
  let constraints = generate_irregular_constraints(existing_constraints);

  generate_classic(&constraints);
}

fn generate_irregular_constraints(existing_constraints: &SudokuConstraints) -> SudokuConstraints {
  let mut regions: Vec<Region> = vec![];
  while regions.is_empty() {
    regions = generate_irregular_regions(existing_constraints);
  }

  let constraints = existing_constraints.clone().with_regions(regions);

  constraints
}

fn generate_irregular_regions(existing_constraints: &SudokuConstraints) -> Vec<Region> {
  println!("==!! GENERATING IRREGULAR REGIONS !!==");

  let mut region_grid = Grid::new(existing_constraints.grid_size);
  let mut regions: Vec<Region> = vec![];

  for region_index in 0..existing_constraints.grid_size {
    // Find starting cell (first one in row major order so it's at the "edge")
    let start_cell = CellPosition::all_cells_in_grid_size(existing_constraints.grid_size).into_iter().find(|cell| {
      if region_grid[cell.row][cell.col] != 0 {
        return false
      }
      let neighbours = cell.otrhogonal_neighbours(existing_constraints.grid_size);
      let out_of_bounds_count = 4 - neighbours.len();
      let bad_neighbours_count = neighbours.into_iter().filter(|peer| region_grid[peer.row][peer.col] != 0).count();
      out_of_bounds_count + bad_neighbours_count >= 2
    }).unwrap();
    dbg!(&start_cell);

    region_grid[start_cell.row][start_cell.col] = region_index as u32 + 1;
    let mut component: HashSet<CellPosition> = HashSet::from([start_cell]);
    for it in 0..existing_constraints.grid_size-1 {
      println!("EXPANDING COMPONENT {}", region_index + 1);
      println!("{}", region_grid.to_string(Some("\n")));
      let peer_candidates: Vec<CellPosition> = component.iter().flat_map(|cell| {
        cell.otrhogonal_neighbours(existing_constraints.grid_size)
      }).unique().filter(|peer| {
        region_grid[peer.row][peer.col] == 0 &&
          !cell_disconnects_grid(&region_grid, peer, existing_constraints.grid_size - 2 - it)
      }).collect();
      if peer_candidates.is_empty() {
        println!("==!! RETRYING IRREGULAR REGIONS !!==");
        return vec![]
      }
      let peer = peer_candidates.choose(&mut rand::rng()).unwrap();
      // let peer = peer_candidates.first().unwrap();
      component.insert(*peer);
      region_grid[peer.row][peer.col] = region_index as u32 + 1;
    }

    // for cell in &component {
    //   region_grid[cell.row][cell.col] = region_index as u32 + 1;
    // }
    dbg!(component.iter().sorted());
    println!("{}", region_grid.to_string(Some("\n")));

    regions.push(Region(component.into_iter().sorted().collect()));
  }

  regions
}

fn cell_disconnects_grid(grid: &Grid, next_cell: &CellPosition, cell_count_left: usize) -> bool {
  let mut visited = Grid::new(grid.len());
  let mut components: Vec<usize> = vec![];

  loop {
    let Some(start_cell) = CellPosition::all_cells_in_grid_size(grid.len()).into_iter().find(|cell| {
      grid[cell.row][cell.col] == 0 && visited[cell.row][cell.col] == 0 && cell != next_cell
    }) else { break };

    // TODO: queue
    let mut component_size: usize = 1;
    visited[start_cell.row][start_cell.col] = 1;
    let mut found = true;
    while found {
      found = false;

      for cell in CellPosition::all_cells_in_grid_size(grid.len()) {
        if visited[cell.row][cell.col] != 0 {
          for peer in cell.otrhogonal_neighbours(grid.len()) {
            if grid[peer.row][peer.col] == 0 && peer != *next_cell && visited[peer.row][peer.col] == 0 {
              visited[peer.row][peer.col] = 1;
              component_size += 1;
              found = true;
            }
          }
        }
      }
    }

    components.push(component_size);
  }

  if components.is_empty() {
    return false
  }

  // println!("===GRID====");
  // println!("{}", grid.to_string(Some("\n")));
  // println!("===VISITED====");
  // println!("{}", visited.to_string(Some("\n")));

  // let res = CellPosition::all_cells_in_grid_size(grid.len()).into_iter().any(|cell| {
  //   grid[cell.row][cell.col] == 0 && visited[cell.row][cell.col] == 0
  // });
  // dbg!(&res);

  // CellPosition::all_cells_in_grid_size(grid.len()).into_iter().any(|cell| {
  //   grid[cell.row][cell.col] == 0 && cell != *next_cell && visited[cell.row][cell.col] == 0
  // })

  components.sort();
  dbg!(&components);
  components.pop();
  if components.is_empty() {
    return false
  }
  let bad_components_total_size: usize = components.iter().sum();
  bad_components_total_size > cell_count_left
}

fn constraint_with_odd_even_cells(constraints: SudokuConstraints, restricted_cells: &Vec<CellPosition>, full_grid: &Grid) -> SudokuConstraints {
  let odd_cells: Vec<CellPosition> = restricted_cells.iter().filter(|cell| full_grid[cell.row][cell.col] != 0 && full_grid[cell.row][cell.col] % 2 == 1).copied().collect();
  let even_cells: Vec<CellPosition> = restricted_cells.iter().filter(|cell| full_grid[cell.row][cell.col] != 0 && full_grid[cell.row][cell.col] % 2 == 0).copied().collect();
  constraints.with_odd_cells(odd_cells).with_even_cells(even_cells)
}

fn generate_odd_even(existing_constraints: &SudokuConstraints) {
  let full_grid = random_grid(existing_constraints);
  let mut grid = full_grid.clone();
  println!("{}\n", grid.to_string(Some("\n")));

  let constraints = constraint_with_odd_even_cells(existing_constraints.clone(), &CellPosition::all_cells_in_grid_size(existing_constraints.grid_size), &full_grid).with_fixed_numbers_grid(&grid);

  let mut restricted_cells = CellPosition::all_cells_in_grid_size(constraints.grid_size);
  assert!(is_odd_even_solvable(&restricted_cells, &constraints, &full_grid));

  simplify_grid_with_constraints(&mut grid, &constraints);

  let constraints = constraints.with_fixed_numbers_grid(&grid);

  restricted_cells.shuffle(&mut rand::rng());
  for without_index in (0..restricted_cells.len()).rev() {
    let cell = restricted_cells.remove(without_index);
    if is_odd_even_solvable(&restricted_cells, &constraints, &full_grid) {
      println!("odd/even SUCCESS!");
      println!("{}", constraints_to_url(&constraint_with_odd_even_cells(constraints.clone(), &restricted_cells, &full_grid)));
    } else {
      restricted_cells.insert(without_index, cell);
      println!("odd/even FAIL!");
    }
  }

  println!("final odd+even count: {}", restricted_cells.len());

  let constraints = constraint_with_odd_even_cells(constraints.clone(), &restricted_cells, &full_grid);
  println!("{}", constraints_to_url(&constraints));
}

fn search_renban_path(path: &mut Vec<CellPosition>, min_value: u32, max_value: u32, used_values: &mut HashSet<u32>, grid: &Grid, visited_cells: &mut HashSet<CellPosition>, best_path: &mut Vec<CellPosition>) {
  if path.len() >= 7.min(grid.len()) {
    return
  }
  if path.len() as u32 == max_value - min_value + 1 {
    if path.len() > best_path.len() && (grid.len() > 6 || (path.iter().any(|cell| cell.row != path[0].row) && path.iter().any(|cell| cell.col != path[0].col))) {
      *best_path = path.clone();
    }
  }

  let cell = *path.last().unwrap();
  for peer in neighbours(&cell, grid.len()) {
    let value = grid[peer.row][peer.col];
    if value > 0 && !used_values.contains(&value) && !visited_cells.contains(&peer) &&
      (
        cell.row == peer.row ||
        cell.col == peer.col ||
        (!visited_cells.contains(&CellPosition { row: cell.row, col: peer.col }) || !visited_cells.contains(&CellPosition { row: peer.row, col: cell.col }))
      )
    {
      path.push(peer);
      used_values.insert(value);
      search_renban_path(path, min_value.min(value), max_value.max(value), used_values, grid, visited_cells, best_path);
      path.pop();
      used_values.remove(&value);
    }
  }
}

fn search_renban(start_cell: &CellPosition, grid: &Grid, visited_cells: &HashSet<CellPosition>) -> Vec<CellPosition> {
  let value = grid[start_cell.row][start_cell.col];
  assert!(value > 0);
  let mut path: Vec<CellPosition> = vec![*start_cell];
  let mut best_path: Vec<CellPosition> = vec![];
  let mut vis = visited_cells.clone();
  let mut used_values: HashSet<u32> = HashSet::from([value]);
  search_renban_path(&mut path, value, value, &mut used_values, grid, &mut vis, &mut best_path);

  best_path
}

fn generate_renban(existing_constraints: &SudokuConstraints) {
  let mut grid = random_grid(existing_constraints);
  println!("{}\n", grid.to_string(Some("\n")));

  let mut renbans: Vec<Renban> = vec![];
  let mut visited_cells: HashSet<CellPosition> = HashSet::new();

  for start_cell in CellPosition::all_cells_in_grid_size(existing_constraints.grid_size) {
    if visited_cells.contains(&start_cell) {
      continue
    }

    let renban = search_renban(&start_cell, &grid, &visited_cells);
    if renban.len() < 3 {
      continue
    }

    visited_cells.extend(renban.clone());
    renbans.push(Renban(renban));
  }

  println!("renban count: {}", renbans.len());

  let mut constraints = existing_constraints
    .clone()
    .with_renbans(renbans.clone());

  simplify_grid_with_constraints(&mut grid, &constraints);

  constraints = constraints.with_fixed_numbers_grid(&grid);

  println!("renban count: {}", renbans.len());

  let mut minimal_renbans: Vec<Renban> = renbans.clone();
  minimal_renbans.shuffle(&mut rand::rng());
  for without_index in (0..minimal_renbans.len()).rev() {
    let renban = minimal_renbans.remove(without_index);
    if is_renban_solvable(&minimal_renbans, &constraints) {
      println!("SUCCESS!");
    } else {
      minimal_renbans.insert(without_index, renban);
    }
  }

  println!("final renban count: {}", minimal_renbans.len());

  let constraints = constraints.with_renbans(minimal_renbans);
  println!("{}", constraints_to_url(&constraints));
}

fn generate_killer_regions(existing_constraints: &SudokuConstraints) -> Vec<Region> {
  println!("==!! GENERATING KILLER REGIONS !!==");

  let mut killer_grid = Grid::new(existing_constraints.grid_size);
  let mut killer_regions: Vec<Region> = vec![];
  let total_cell_count = killer_grid.len() * killer_grid.len();
  let mut covered_cell_count: usize = 0;

  while covered_cell_count < total_cell_count {
    // Find starting cell (first one in row major order so it's at the "edge")
    let start_cell = CellPosition::all_cells_in_grid_size(existing_constraints.grid_size).into_iter().find(|cell| {
      if killer_grid[cell.row][cell.col] != 0 {
        return false
      }
      let neighbours = cell.otrhogonal_neighbours(existing_constraints.grid_size);
      let out_of_bounds_count = 4 - neighbours.len();
      let bad_neighbours_count = neighbours.into_iter().filter(|peer| killer_grid[peer.row][peer.col] != 0).count();
      out_of_bounds_count + bad_neighbours_count >= 2
    }).unwrap();
    dbg!(&start_cell);

    killer_grid[start_cell.row][start_cell.col] = killer_regions.len() as u32 + 1;
    let mut component: HashSet<CellPosition> = HashSet::from([start_cell]);
    let upper_bound = if existing_constraints.grid_size == 9 { 6 }
      else if existing_constraints.grid_size == 6 { 4 }
      else if existing_constraints.grid_size == 4 { 3 }
      else { panic!() };
    let killer_cage_size = rand::rng().random_range(2..=upper_bound);
    for _ in 0..killer_cage_size-1 {
      println!("EXPANDING COMPONENT {}", killer_regions.len() as u32 + 1);
      println!("{}", killer_grid.to_string(Some("\n")));
      let peer_candidates: Vec<CellPosition> = component.iter().flat_map(|cell| {
        cell.otrhogonal_neighbours(existing_constraints.grid_size)
      }).unique().filter(|peer| {
        killer_grid[peer.row][peer.col] == 0 // &&
          // !cell_disconnects_grid(&region_grid, peer, existing_constraints.grid_size - 2 - it)
      }).collect();
      if peer_candidates.is_empty() {
        // println!("==!! RETRYING KILLER REGIONS !!==");
        // return vec![]
        // overestimated available space
        break
      }
      let peer = peer_candidates.choose(&mut rand::rng()).unwrap();
      // let peer = peer_candidates.first().unwrap();
      component.insert(*peer);
      killer_grid[peer.row][peer.col] = killer_regions.len() as u32 + 1;
    }

    // for cell in &component {
    //   region_grid[cell.row][cell.col] = region_index as u32 + 1;
    // }
    dbg!(component.iter().sorted());
    for row in 0..killer_grid.len() {
      for col in 0..killer_grid[0].len() {
        print!("{}", if killer_grid[row][col] < 10 { char::from_u32(killer_grid[row][col] + u32::from('0')).unwrap() } else { char::from_u32(killer_grid[row][col] - 10 + u32::from('A')).unwrap() });
        // print!("{:x}", killer_grid[row][col]);
      }
      println!();
    }
    // println!("{}", killer_grid.to_string(Some("\n")));

    covered_cell_count += component.len();
    killer_regions.push(Region(component.into_iter().sorted().collect()));
  }

  killer_regions
}

fn generate_killer(existing_constraints: &SudokuConstraints) {
  let killer_regions = generate_killer_regions(existing_constraints);

  let mut grid = random_grid(&existing_constraints.clone().with_extra_regions(killer_regions.clone()));

  let killer_cages: Vec<KillerCage> = killer_regions.into_iter().map(|region| {
    let sum: u32 = region.iter().map(|cell| grid[cell.row][cell.col]).sum();
    KillerCage { sum: Some(sum), region }
  }).collect();

  let mut constraints = existing_constraints
    .clone()
    .with_killer_cages(killer_cages);

  println!("\n\n{}\n\n", grid.to_string(Some("\n")));

  simplify_grid_with_constraints(&mut grid, &constraints);

  constraints = constraints.with_fixed_numbers_grid(&grid);

  println!("{}", constraints_to_url(&constraints));
}

fn generate_extra_regions(existing_constraints: &SudokuConstraints) {
  // let mut extra_regions = generate_irregular_regions(existing_constraints);
  let mut extra_regions: Vec<Region> = vec![];
  while extra_regions.is_empty() {
    extra_regions = generate_irregular_regions(existing_constraints);
  }

  let mut constraints = existing_constraints
    .clone()
    .with_extra_regions(extra_regions.clone());

  println!("Generating random grid");
  let mut grid = random_grid(&constraints);

  // Remove some given digits and some regions

  assert!(is_grid_solvable(&grid, &constraints));

  let mut cells: Vec<CellPosition> = CellPosition::all_cells_in_grid_size(constraints.grid_size)
    .into_iter()
    .filter(|cell| grid[cell.row][cell.col] != 0)
    .collect();
  cells.shuffle(&mut rand::rng());

  extra_regions.shuffle(&mut rand::rng());

  println!("extra regions count: {}", extra_regions.len());

  let mut cells_index = 0;
  let mut extra_regions_index: i32 = extra_regions.len() as i32 - 1;

  while cells_index < cells.len() || extra_regions_index >= 0 {
    // add cut-off when to start deleting extra regions
    // experiment with classic windowku layout
    // maybe generate half and mirror it

    if cells_index < cells.len() && (extra_regions_index < 0 || cells_index < cells.len() / 2 || rand::rng().random_bool(3.0 / 4.0)) {
      let CellPosition { row, col } = cells[cells_index];
      let value = grid[row][col];
      assert_ne!(value, 0);
      grid[row][col] = 0;

      if is_grid_solvable(&grid, &constraints) {
        println!("[cells] SUCCESS!");
        println!("{}\n", grid.to_string(Some("\n")));

        constraints = constraints.with_fixed_numbers_grid(&grid);
      } else {
        println!("[cells] FAIL!");
        grid[row][col] = value;
      }

      cells_index += 1;
    } else {
      let extra_region = extra_regions.remove(extra_regions_index as usize);
      if is_extra_regions_solvable(&extra_regions, &constraints) {
        println!("[extra region] SUCCESS!");
        constraints = constraints.with_extra_regions(extra_regions.clone());
      } else {
        println!("[extra region] FAIL!");
        extra_regions.insert(extra_regions_index as usize, extra_region);
      }

      extra_regions_index -= 1;
    }
  }

  println!("final extra regions count: {}", extra_regions.len());

  assert!(is_constraints_solvable(&constraints));

  println!("{}", constraints_to_url(&constraints));
}

fn generate_anti_knight(existing_constraints: &SudokuConstraints) {
  let constraints = generate_classic_constraints(&existing_constraints.clone().with_anti_knight());

  assert!(is_constraints_solvable(&constraints));
  assert!(!is_constraints_solvable(&constraints.clone().unset_anti_knight()));

  // TODO: check that any of the grid steps (and not only?) uses antiknight constraint

  println!("{}", constraints_to_url(&constraints));
}

fn generate_anti_king(existing_constraints: &SudokuConstraints) {
  let constraints = generate_classic_constraints(&existing_constraints.clone().with_anti_king());

  assert!(is_constraints_solvable(&constraints));
  assert!(!is_constraints_solvable(&constraints.clone().unset_anti_king()));

  // TODO: check that any of the grid steps (and not only?) uses antiknight constraint

  println!("{}", constraints_to_url(&constraints));
}

fn generate_diagonal(existing_constraints: &SudokuConstraints) {
  let state = rand::rng().random_range(1..=3);

  let mut constrains_with_diagonals = existing_constraints.clone();
  if (state & 1) > 0 {
    constrains_with_diagonals = constrains_with_diagonals.with_primary_diagonal();
  }
  if (state & 2) > 0 {
    constrains_with_diagonals = constrains_with_diagonals.with_secondary_diagonal();
  }

  let mut constraints = generate_classic_constraints(&constrains_with_diagonals.with_diagonals());

  assert!(is_constraints_solvable(&constraints));

  if constraints.primary_diagonal.unwrap_or_default() &&
     is_constraints_solvable(&constraints.clone().unset_primary_diagonal()) {
    constraints = constraints.unset_primary_diagonal();
  }

  if constraints.secondary_diagonal.unwrap_or_default() &&
     is_constraints_solvable(&constraints.clone().unset_secondary_diagonal()) {
    constraints = constraints.unset_secondary_diagonal();
  }

  assert!(constraints.primary_diagonal.unwrap_or_default() || constraints.secondary_diagonal.unwrap_or_default());

  // TODO: check that any of the grid steps (and not only?) uses diagonal constraint
  // or the fact that they are necessary is enough

  println!("{}", constraints_to_url(&constraints));
}

fn generate_top_bottom(existing_constraints: &SudokuConstraints) {
  let constraints = generate_classic_constraints(&existing_constraints.clone().with_top_bottom());

  assert!(is_constraints_solvable(&constraints));
  assert!(!is_constraints_solvable(&constraints.clone().unset_top_bottom()));

  // TODO: check that any of the grid steps (and not only?) uses antiknight constraint

  println!("{}", constraints_to_url(&constraints));
}

fn generate_arrow(existing_constraints: &SudokuConstraints) {
  let mut grid = random_grid(existing_constraints);
  println!("{}\n", grid.to_string(Some("\n")));

  let mut arrows: Vec<Arrow> = vec![];
  let mut visited_cells: HashSet<CellPosition> = HashSet::new();

  for start_cell in CellPosition::all_cells_in_grid_size(existing_constraints.grid_size) {
    if visited_cells.contains(&start_cell) {
      continue
    }

    let arrow = search_arrow(&start_cell, &grid, &visited_cells);
    if arrow.arrow_cells.len() == 0 {
      continue
    }

    visited_cells.extend(arrow.arrow_cells.clone());
    visited_cells.extend(arrow.circle_cells.clone());
    arrows.push(arrow);
  }

  println!("arrow count: {}", arrows.len());

  let mut constraints = existing_constraints
    .clone()
    .with_arrows(arrows.clone());

  assert!(is_grid_solvable(&grid, &constraints));

  let mut cells: Vec<CellPosition> = CellPosition::all_cells_in_grid_size(constraints.grid_size)
    .into_iter()
    .filter(|cell| grid[cell.row][cell.col] != 0)
    .collect();
  cells.shuffle(&mut rand::rng());

  arrows.shuffle(&mut rand::rng());

  println!("arrow count: {}", arrows.len());

  // let mut minimal_arrows: Vec<Arrow> = arrows.clone();
  // minimal_arrows.shuffle(&mut rand::rng());
  // for without_index in (0..minimal_arrows.len()).rev() {
  //   let arrow = minimal_arrows.remove(without_index);
  //   if is_arrow_solvable(&minimal_arrows, &constraints) {
  //     println!("SUCCESS!");
  //   } else {
  //     minimal_arrows.insert(without_index, arrow);
  //   }
  // }

  let mut cells_index = 0;
  let mut arrows_index: i32 = arrows.len() as i32 - 1;

  while cells_index < cells.len() || arrows_index >= 0 {
    if cells_index < cells.len() && (arrows_index < 0 || cells_index < cells.len() / 2 || rand::rng().random_bool(3.0 / 4.0)) {
      let CellPosition { row, col } = cells[cells_index];
      let value = grid[row][col];
      assert_ne!(value, 0);
      grid[row][col] = 0;

      if is_grid_solvable(&grid, &constraints) {
        println!("[cells] SUCCESS!");
        println!("{}\n", grid.to_string(Some("\n")));

        constraints = constraints.with_fixed_numbers_grid(&grid);
      } else {
        println!("[cells] FAIL!");
        grid[row][col] = value;
      }

      cells_index += 1;
    } else {
      let arrow = arrows.remove(arrows_index as usize);
      if is_arrow_solvable(&arrows, &constraints) {
        println!("[arrow] SUCCESS!");
        constraints = constraints.with_arrows(arrows.clone());
      } else {
        println!("[arrow] FAIL!");
        arrows.insert(arrows_index as usize, arrow);
      }

      arrows_index -= 1;
    }
  }

  println!("final arrow count: {}", arrows.len());

  let constraints = constraints.with_arrows(arrows);
  println!("{}", constraints_to_url(&constraints));
}

fn search_arrow_path(path: &mut Vec<CellPosition>, sum_left: u32, grid: &Grid, visited_cells: &mut HashSet<CellPosition>, best_path: &mut Vec<CellPosition>) {
  if sum_left == 0 {
    if path.len() > best_path.len() {
      *best_path = path.clone();
    }
    return
  }

  let cell = *path.last().unwrap();
  for peer in neighbours(&cell, grid.len()) {
    let value = grid[peer.row][peer.col];
    if value > 0 && value <= sum_left && path.iter().all(|c| *c != peer) && !visited_cells.contains(&peer) &&
      (
        cell.row == peer.row ||
        cell.col == peer.col ||
        (!visited_cells.contains(&CellPosition { row: cell.row, col: peer.col }) || !visited_cells.contains(&CellPosition { row: peer.row, col: cell.col }))
      )
    {
      path.push(peer);
      search_arrow_path(path, sum_left - value, grid, visited_cells, best_path);
      path.pop();
    }
  }
}

fn search_arrow(start_cell: &CellPosition, grid: &Grid, visited_cells: &HashSet<CellPosition>) -> Arrow {
  let value = grid[start_cell.row][start_cell.col];
  assert!(value > 0);
  let mut path: Vec<CellPosition> = vec![*start_cell];
  let mut best_path: Vec<CellPosition> = vec![];
  let mut vis = visited_cells.clone();
  search_arrow_path(&mut path, value, grid, &mut vis, &mut best_path);

  Arrow {
    circle_cells: vec![*start_cell],
    arrow_cells: if best_path.is_empty() { vec![] } else { best_path[1..].to_vec() },
  }
}

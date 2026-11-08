use crate::{solver::brute::BruteSolver, types::{CellPosition, FixedNumber, Grid, KropkiDot, SudokuConstraints}};
use ntest::timeout;

// N4IghgdgLglg0jCBzEAuKAnArgUwDTjTwQxIAWUamuBAZjAB44AmAclgLYBGOGAzmgDaoAA4B7PjFhiIaUAGMxAGzQBGAhjEB3NACYAvgQBuYJbjQBmQ6IlSYMuSEUrUAVg3a0AFkMgTZnDQATmsQcUlpWVQFZTQADg8dVB9jU3NUKzwbCPsomJddRLRXX390zOy7B2inWNQAdiK3UrTA1AA2UPCqvNqXAAYm%2BpaAyy7bSMdnNSa4kfSfAF0CJAwYZgBlGAAvNqCCAGtNEQOYABExKAFUYSccJSVVKbrBkE0kzLuH3WeXdTfPBlfMxLgAVACeIjaIAAwjI%2BDh5FhYEZAqF5PdHr80K93npfBjvtjUP88agDAQQVAIVC0LD4YjkTBUSB0ZinjVpm4mqoCZifpy6u0ecCwZDoRcsFwlGisl8sYKXF4miUCISlAL8t4mp1KWLaagQJLpbKFOzicKAUkUvLNX00I0rd5RdTxXTjTLWXL1RytQ0mp91XauQknUC9a6DfSIAikSjTfLffaSU0bUHiYUwzaqTToXCY4z416zQ8k1zM2TeWr%2BcSLCKI7m6fnY0yWWzSxnZnyiYrLF2G27Dc3C8yEz7ie4w3Nqz2%2FZaydOQDnB9GW0X2wq%2Fcqw4Ga73uTuXY2hwy46PWYt9EA
fn mixed_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers([
      FixedNumber::new(2, 1, 3), FixedNumber::new(4, 5, 9), FixedNumber::new(4, 8, 3),
      FixedNumber::new(5, 2, 3), FixedNumber::new(5, 7, 6), FixedNumber::new(7, 0, 3),
      FixedNumber::new(8, 1, 4),
    ])
    .with_kropki_dots(
      [
        KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(3, 1)),
        KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(2, 1)),
        KropkiDot::double(CellPosition::new(1, 5), CellPosition::new(1, 6)),
        KropkiDot::double(CellPosition::new(5, 4), CellPosition::new(6, 4)),
        KropkiDot::double(CellPosition::new(4, 6), CellPosition::new(4, 7)),
        KropkiDot::consecutive(CellPosition::new(3, 7), CellPosition::new(3, 8)),
        KropkiDot::consecutive(CellPosition::new(4, 1), CellPosition::new(4, 2)),
        KropkiDot::consecutive(CellPosition::new(1, 2), CellPosition::new(1, 3)),
        KropkiDot::consecutive(CellPosition::new(8, 2), CellPosition::new(8, 3)),
        KropkiDot::consecutive(CellPosition::new(8, 5), CellPosition::new(8, 6)),
        KropkiDot::consecutive(CellPosition::new(3, 4), CellPosition::new(3, 5)),
      ]
    )
    .with_anti_king()
    .with_anti_knight();
  let solution = Grid::from_string("
    694231875
    875694231
    231875694
    569423187
    187569423
    423187569
    756942318
    318756942
    942318756
  ");
  (constraints, solution)
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_solves_mixed_9x9() {
  let (constraints, solution) = mixed_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(2_000)]
fn brute_solver_with_logic_solves_mixed_9x9() {
  let (constraints, solution) = mixed_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

// https://lisudoku.xyz/p/PrZ4jUcJYyza3vGi713O
// N4IghgdgLglg0hGBzAFlEAuKAnArgUwBoQlsYATAZRgC99MA2Yga2wHsAHZmAETagDOmANqgAxvgA2kgIyZxbSZgAMxdgHcVAX2ITpAJnkgxizDLVtNGZTpDl%2BAFQCeHehhABhNhAH4xuWAA3eh1xKVkjEyUMcxANbV1wwwwFaP0LKxtieyhnV0xPb19%2FIJDCMOk5FONTawyzWz1JZNSzepjbHLy3ED5cACNJMoqI6qjMdLjLBsSDSNqAZnaZTscXHr7B4eNwqtaMABZ2rJ25sdqAVmPV3PWCrx8%2FAJhgkFDT0f2jqasV2eb5tFvvEMPobt17kUnqU3uUPnsakD2mD%2Fi1EZgrj8JuC7u4HsVnq93k0EeNQe0Fo0koDMEssRhKdk1vl3JshrCRqTarEQSiPmiyTzpgycSzemwBuzibsaTF2gcqWd9pMQQqmbcxfjoS9tiTZap6RdFQDztEhVYjeqIXioSUdRz4fqKcaBdznVbcYVHnaiQBdLRAA%3D%3D
fn mixed_6x6_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(6)
    .with_kropki_dots(
      [
        KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
        KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
        KropkiDot::consecutive(CellPosition::new(0, 4), CellPosition::new(0, 5)),
        KropkiDot::double(CellPosition::new(1, 0), CellPosition::new(1, 1)),
        KropkiDot::double(CellPosition::new(1, 2), CellPosition::new(1, 3)),
        KropkiDot::consecutive(CellPosition::new(1, 4), CellPosition::new(2, 4)),
        KropkiDot::double(CellPosition::new(2, 1), CellPosition::new(3, 1)),
        KropkiDot::consecutive(CellPosition::new(2, 4), CellPosition::new(2, 5)),
        KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(3, 1)),
        KropkiDot::double(CellPosition::new(3, 2), CellPosition::new(3, 3)),
        KropkiDot::consecutive(CellPosition::new(4, 1), CellPosition::new(4, 2)),
        KropkiDot::consecutive(CellPosition::new(5, 0), CellPosition::new(5, 1)),
      ]
    )
    .with_anti_knight();
  let solution = Grid::from_string("
    456312
    213654
    134265
    562431
    621543
    345126
  ");

  (constraints, solution)
}

#[test]
#[timeout(2_000)]
fn brute_solver_without_logic_solves_mixed_6x6() {
  let (constraints, solution) = mixed_6x6_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(1_000)]
fn brute_solver_with_logic_solves_mixed_6x6() {
  let (constraints, solution) = mixed_6x6_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

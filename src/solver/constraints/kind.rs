use crate::solver::constraints::{ConstraintBuilder, anti_king::ANTI_KING_CONSTRAINT_BUILDER, anti_knight::ANTI_KNIGHT_CONSTRAINT_BUILDER, arrow::ARROW_CONSTRAINT_BUILDER, diagonals::{PRIMARY_DIAGONAL_CONSTRAINT_BUILDER, SECONDARY_DIAGONAL_CONSTRAINT_BUILDER}, fixed_numbers::FIXED_NUMBERS_CONSTRAINT_BUILDER, killer::KILLER_CONSTRAINT_BUILDER, kropki::KROPKI_CONSTRAINT_BUILDER, odd_even::{EVEN_CONSTRAINT_BUILDER, ODD_CONSTRAINT_BUILDER}, palindrome::PALINDROME_CONSTRAINT_BUILDER, regions::REGIONS_CONSTRAINT_BUILDER, renban::RENBAN_CONSTRAINT_BUILDER, thermo::THERMO_CONSTRAINT_BUILDER, top_bottom::TOP_BOTTOM_CONSTRAINT_BUILDER};
use strum::{EnumCount, EnumIter};

#[repr(usize)]
#[derive(EnumIter, Debug, EnumCount)]
pub enum ConstraintKind {
  Regions,
  FixedNumber,
  Thermo,
  Arrow,
  Killer,
  AntiKnight,
  AntiKing,
  PrimaryDiagonal,
  SecondaryDiagonal,
  Kropki,
  Odd,
  Even,
  TopBottom,
  Renban,
  Palindrome,
}

impl ConstraintKind {
   pub(crate) fn builder(&self) -> &'static dyn ConstraintBuilder {
      match self {
        ConstraintKind::Regions => &REGIONS_CONSTRAINT_BUILDER,
        ConstraintKind::FixedNumber => &FIXED_NUMBERS_CONSTRAINT_BUILDER,
        ConstraintKind::Thermo => &THERMO_CONSTRAINT_BUILDER,
        ConstraintKind::Arrow => &ARROW_CONSTRAINT_BUILDER,
        ConstraintKind::Killer => &KILLER_CONSTRAINT_BUILDER,
        ConstraintKind::AntiKnight => &ANTI_KNIGHT_CONSTRAINT_BUILDER,
        ConstraintKind::AntiKing => &ANTI_KING_CONSTRAINT_BUILDER,
        ConstraintKind::PrimaryDiagonal => &PRIMARY_DIAGONAL_CONSTRAINT_BUILDER,
        ConstraintKind::SecondaryDiagonal => &SECONDARY_DIAGONAL_CONSTRAINT_BUILDER,
        ConstraintKind::Kropki => &KROPKI_CONSTRAINT_BUILDER,
        ConstraintKind::Odd => &ODD_CONSTRAINT_BUILDER,
        ConstraintKind::Even => &EVEN_CONSTRAINT_BUILDER,
        ConstraintKind::TopBottom => &TOP_BOTTOM_CONSTRAINT_BUILDER,
        ConstraintKind::Renban => &RENBAN_CONSTRAINT_BUILDER,
        ConstraintKind::Palindrome => &PALINDROME_CONSTRAINT_BUILDER,
    }
  }
}

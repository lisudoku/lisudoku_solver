use crate::types::Area;

#[derive(Default, Clone, Debug)]
pub(crate) struct AreaIdMapper {
  next_id: usize,
  regions: Vec<Option<usize>>,
  rows: Vec<Option<usize>>,
  columns: Vec<Option<usize>>,
  thermos: Vec<Option<usize>>,
  arrows: Vec<Option<usize>>,
  killer_cages: Vec<Option<usize>>,
  kropki_dots: Vec<Option<usize>>,
  renbans: Vec<Option<usize>>,
  palindromes: Vec<Option<usize>>,
  primary_diagonal: Option<usize>,
  secondary_diagonal: Option<usize>,
  grid: Option<usize>,
}

impl AreaIdMapper {
  pub(crate) fn area_to_id(&mut self, area: &Area) -> Option<usize> {
    let area_id = match area {
      &Area::Region(region_index) => Self::get_or_insert(&mut self.regions, region_index, &mut self.next_id),
      &Area::Row(row_index) => Self::get_or_insert(&mut self.rows, row_index, &mut self.next_id),
      &Area::Column(col_index) => Self::get_or_insert(&mut self.columns, col_index, &mut self.next_id),
      &Area::Thermo(thermo_index) => Self::get_or_insert(&mut self.thermos, thermo_index, &mut self.next_id),
      &Area::Arrow(arrow_index) => Self::get_or_insert(&mut self.arrows, arrow_index, &mut self.next_id),
      &Area::KillerCage(killer_cage_index) => Self::get_or_insert(&mut self.killer_cages, killer_cage_index, &mut self.next_id),
      &Area::KropkiDot(kropki_dot_index) => Self::get_or_insert(&mut self.kropki_dots, kropki_dot_index, &mut self.next_id),
      &Area::Renban(renban_index) => Self::get_or_insert(&mut self.renbans, renban_index, &mut self.next_id),
      &Area::Palindrome(palindrome_index) => Self::get_or_insert(&mut self.palindromes, palindrome_index, &mut self.next_id),
      &Area::PrimaryDiagonal => Self::get_or_insert_opt(&mut self.primary_diagonal, &mut self.next_id),
      &Area::SecondaryDiagonal => Self::get_or_insert_opt(&mut self.secondary_diagonal, &mut self.next_id),
      &Area::Grid => Self::get_or_insert_opt(&mut self.grid, &mut self.next_id),
      &Area::Cell(_, _) => return None,
      &Area::Adhoc(_) => return None,
    };
    Some(area_id)
  }

  pub(crate) fn get_or_insert(table: &mut Vec<Option<usize>>, table_index: usize, next_id: &mut usize) -> usize {
    if table_index >= table.len() {
      table.resize(table_index + 1, None);
    }
    Self::get_or_insert_opt(&mut table[table_index], next_id)
  }

  pub(crate) fn get_or_insert_opt(slot: &mut Option<usize>, next_id: &mut usize) -> usize {
    if let Some(area_id) = slot {
      return *area_id
    }
    let area_id = *next_id;
    *slot = Some(area_id);
    *next_id += 1;
    area_id
  }
}

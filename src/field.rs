use crate::cell::Cell;

pub struct Field {
    cells: Vec<Cell>,
}

impl Field {
    pub fn new(cells: Vec<Cell>) -> Field {
        Field {
            cells,
        }
    }
}

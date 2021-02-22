use crate::cell::Cell;
use crate::cell::cell_methods::CellMethods;
use ggez::Context;

pub struct Field {
    cells: Vec<Cell>,
}

impl Field {
    pub fn new(cells: Vec<Cell>) -> Field {
        Field {
            cells,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for cell in &mut self.cells {
            cell.draw(ctx);
        }
    }

    pub fn execute_code(&mut self) {
        for cell in &mut self.cells {
            cell.execute_gene();
        }
    }
}

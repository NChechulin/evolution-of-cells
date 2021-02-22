use crate::cell::Cell;
use crate::cell::cell_methods::CellMethods;
use ggez::Context;

#[derive(Clone)]
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
        for i in 0..self.cells.len() {
            self.cells[i] = self.move_cell(self.cells[i].clone());
        }
    }

    fn cell_is_empty(&self, x_pos: i32, y_pos: i32) -> bool {
        for cell in &self.cells {
            if cell.x_pos == x_pos && cell.y_pos == y_pos {
                return false;
            }
        }
        true
    }

    pub fn move_cell(&self, mut cell: Cell) -> Cell {
        let new_pos = cell.get_new_pos();

        if self.cell_is_empty(new_pos.0, new_pos.1) {
            cell.x_pos = new_pos.0;
            cell.y_pos = new_pos.1;
        }
        cell
    }
}

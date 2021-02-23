use crate::cell::Cell;
use crate::cell::gene::Gene;
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
            self.cells[i] = self.execute_next_gene(self.cells[i].clone());
        }
    }

    fn position_is_empty(&self, x_pos: i32, y_pos: i32) -> bool {
        for cell in &self.cells {
            if cell.x_pos == x_pos && cell.y_pos == y_pos {
                return false;
            }
        }
        true
    }

    fn move_cell(&self, mut cell: Cell) -> Cell {
        let new_pos = cell.get_new_pos();

        if self.position_is_empty(new_pos.0, new_pos.1) {
            cell.x_pos = new_pos.0;
            cell.y_pos = new_pos.1;
        }
        cell
    }

    fn cell_photosynthesize(&self, mut cell: Cell) -> Cell {
        cell.energy += cell.energy_constants.photosynthesize;
        cell
    }

    fn execute_next_gene(&mut self, mut cell: Cell) -> Cell {
        match cell.get_next_gene() {
            Gene::MoveForward => self.move_cell(cell),
            // Gene::Eat => {}
            Gene::Photosynthesize => self.cell_photosynthesize(cell),
            // Gene::ChangeLineOfSight(_) => {}
            // Gene::AttachToCell => {}
            // Gene::DetachFromAllCells => {}
            // Gene::ShareEnergy => {}
            // Gene::SkipMove => {}
            // Gene::Split => {}
            // Gene::GoTo(_) => {}
            _ => cell,
        }
    }
}

pub(crate) mod gene;
pub mod energy_constants;
pub mod cell_methods;

use ggez::graphics::Color;
use ggez::mint::Vector2;
use crate::GRID_SIZE;
use crate::GRID_CELL_SIZE;
use crate::CELL_PADDING;
use crate::field::Field;
use ggez::graphics;
use gene::Gene;
use energy_constants::EnergyConstants;
use cell_methods::CellMethods;

#[derive(Copy, Clone)]
pub struct Cell {
    pub x_pos: i32,
    pub y_pos: i32,
    pub color: Color,
    pub energy: f32,
    max_energy: f32,
    pub lifetime: u32,
    line_of_sight: Vector2<i32>,
    genome: [u8; 64],
    genome_index: usize,
    pub energy_constants: EnergyConstants,
}

impl Cell {
    pub fn new(x_pos: i32, y_pos: i32, color: Color, direction: u8, energy_constants: EnergyConstants) -> Cell {
        Cell {
            x_pos,
            y_pos,
            color,
            energy: 128f32,
            max_energy: 256f32,
            lifetime: 0,
            line_of_sight: Cell::direction_by_number(direction),
            genome: [1, 4, 1, 10, 1, 4, 1, 8, 1, 3, 1, 1, 7, 1, 11, 1, 3, 1, 8, 1, 10, 1, 11, 1, 7, 1, 11, 1, 9, 1, 8, 1, 9, 1, 5, 1, 6, 3, 7, 1, 10, 1, 11, 1, 9, 1, 7, 1, 8, 1, 9, 1, 11, 1, 4, 1, 5, 1, 4, 1, 6, 1, 6, 1],
            genome_index: 0,
            energy_constants,
        }
    }

    fn direction_by_number(direction: u8) -> Vector2<i32> {
        match direction {
            0 => Vector2 { x: 0, y: 1 },
            1 => Vector2 { x: 1, y: 1 },
            2 => Vector2 { x: 1, y: 0 },
            3 => Vector2 { x: 1, y: -1 },
            4 => Vector2 { x: 0, y: -1 },
            5 => Vector2 { x: -1, y: -1 },
            6 => Vector2 { x: -1, y: 0 },
            7 => Vector2 { x: -1, y: 1 },
            _ => Vector2 { x: 0, y: 0 },
        }
    }

    /// Returns position at which cell is pointing (forward)
    pub fn get_new_pos(&self) -> (i32, i32) {
        let x_pos = (self.x_pos + self.line_of_sight.x).rem_euclid(GRID_SIZE.0);
        let y_pos = (self.y_pos + self.line_of_sight.y).rem_euclid(GRID_SIZE.1);
        (x_pos, y_pos)
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let x_pos = self.x_pos * GRID_CELL_SIZE.0 + self.x_pos * CELL_PADDING;
        let y_pos = self.y_pos * GRID_CELL_SIZE.1 + self.y_pos * CELL_PADDING;

        let bounds = graphics::Rect::new(
            x_pos as f32,
            y_pos as f32,
            GRID_CELL_SIZE.0 as f32,
            GRID_CELL_SIZE.1 as f32,
        );

        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            bounds,
            self.color,
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 }, ))?;
        Ok(())
    }

    pub fn move_forward(&mut self) {
        let new_pos = self.get_new_pos();
        self.x_pos = new_pos.0;
        self.y_pos = new_pos.1;
    }

    pub fn photosynthesize(&mut self) {
        self.energy += self.energy_constants.photosynthesize;
    }

    pub fn change_line_of_sight(&mut self, direction: u8) {
        self.line_of_sight = Cell::direction_by_number(direction);
    }

    pub fn get_next_gene(&mut self) -> Gene {
        self.lifetime += 1;
        self.genome_index += 1;
        self.genome_index %= self.genome.len();

        Gene::from(self.genome[self.genome_index])
    }
}

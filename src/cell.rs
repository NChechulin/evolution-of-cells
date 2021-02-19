mod gene;
pub mod cell_methods;

use ggez::graphics::Color;
use ggez::mint::Vector2;
use crate::GRID_SIZE;
use crate::GRID_CELL_SIZE;
use crate::CELL_PADDING;
use ggez::graphics;
use gene::Gene;
use cell_methods::CellMethods;


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
}

impl Cell {
    pub fn new(x_pos: i32, y_pos: i32, color: Color, direction: u8) -> Cell {
        Cell {
            x_pos,
            y_pos,
            color,
            energy: 128f32,
            max_energy: 256f32,
            lifetime: 0,
            line_of_sight: Cell::direction_by_number(direction),
            genome: [1, 4, 1, 10, 1, 4, 1, 8, 1, 11, 1, 1, 7, 1, 11, 1, 11, 1, 8, 1, 10, 1, 11, 1, 7, 1, 11, 1, 9, 1, 8, 1, 9, 1, 5, 1, 6, 1, 7, 1, 10, 1, 11, 1, 9, 1, 7, 1, 8, 1, 9, 1, 11, 1, 4, 1, 5, 1, 4, 1, 6, 1, 6, 1],
            genome_index: 0,
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
    fn get_new_pos(&self) -> (i32, i32) {
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
}


impl CellMethods for Cell {
    fn die(&mut self) {
        unimplemented!()
    }

    fn split(&mut self) {
        unimplemented!()
    }

    fn decrease_energy_by(&mut self, by: f32) {
        self.energy -= by;

        if self.energy < 0f32 {
            self.die();
        }
    }

    fn increase_energy_by(&mut self, by: f32) {
        self.energy += by;
        if self.energy > self.max_energy {
            self.energy = self.max_energy;
        }
    }

    fn move_forward(&mut self) {
        let new_pos = self.get_new_pos();
        self.x_pos = new_pos.0;
        self.y_pos = new_pos.1;
    }

    fn eat(&mut self) {
        unimplemented!()
    }

    fn photosynthesize(&mut self) {
        unimplemented!()
    }

    fn change_line_of_sight(&mut self, new_direction: u8) {
        self.line_of_sight = Cell::direction_by_number(new_direction);
    }

    fn attach_to_cell(&mut self) {
        // TODO: implement colonies
        unimplemented!()
    }

    fn detach_from_all_cells(&mut self) {
        // TODO: implement colonies
        unimplemented!()
    }

    fn share_energy(&mut self) {
        // TODO: implement colonies
        unimplemented!()
    }

    fn skip_move(&mut self) {
        // TODO: implement a blocking mechanism in order to stop/continue executing next command
        unimplemented!()
    }

    fn go_to(&mut self, new_index: usize) {
        self.genome_index = new_index;
        self.execute_gene();
    }

    fn execute_gene(&mut self) {
        self.lifetime += 1;
        self.genome_index += 1;
        self.genome_index %= self.genome.len();

        match Gene::from(self.genome[self.genome_index]) {
            Gene::MoveForward => self.move_forward(),
            Gene::Eat => self.eat(),
            Gene::Photosynthesize => self.photosynthesize(),
            Gene::ChangeLineOfSight(val) => self.change_line_of_sight(val % 8),
            Gene::AttachToCell => self.attach_to_cell(),
            Gene::DetachFromAllCells => self.detach_from_all_cells(),
            Gene::ShareEnergy => self.share_energy(),
            Gene::SkipMove => self.skip_move(),
            Gene::Split => self.split(),
            Gene::GoTo(gene_index) => self.go_to(gene_index),
        }
    }
}

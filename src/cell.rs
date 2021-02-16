use ggez::graphics::Color;
use ggez::mint::Vector2;
use crate::GRID_SIZE;
use crate::GRID_CELL_SIZE;
use crate::CELL_PADDING;
use ggez::graphics;

pub struct Cell {
    pub x_pos: i32,
    pub y_pos: i32,
    pub color: Color,
    pub energy: i32,
    pub lifetime: u32,
    line_of_sight: Vector2<i32>,
}

impl Cell {
    pub fn new(x_pos: i32, y_pos: i32, color: Color, direction: u8) -> Cell {
        Cell {
            x_pos,
            y_pos,
            color,
            energy: 64,
            lifetime: 0,
            line_of_sight: Cell::direction_by_number(direction),
        }
    }

    pub fn move_forward(&mut self) {
        self.x_pos = (self.x_pos + self.line_of_sight.x).rem_euclid(GRID_SIZE.0);
        self.y_pos = (self.y_pos + self.line_of_sight.y).rem_euclid(GRID_SIZE.1);
    }

    fn reduce_energy_by(&mut self, by: u8) {
        self.energy -= by as i32;

        if self.energy < 0 {
            self.die();
        }
    }

    fn die(&mut self) {
        // do something when cell is dead.
    }

    pub fn change_line_of_sight(&mut self, new_direction: u8) {
        self.line_of_sight = Cell::direction_by_number(new_direction);
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

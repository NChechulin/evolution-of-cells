use ggez::graphics::Color;
use ggez::mint::Vector2;

pub struct Cell {
    pub x_pos: i32,
    pub y_pos: i32,
    pub color: Color,
    line_of_sight: Vector2<i8>,
}

impl Cell {
    pub fn new(x_pos: i32, y_pos: i32, color: Color, direction: u8) -> Cell {
        Cell {
            x_pos,
            y_pos,
            color,
            line_of_sight: Cell::direction_by_number(direction),
        }
    }

    pub fn change_line_of_sight(&mut self, new_direction: u8) {
        self.line_of_sight = Cell::direction_by_number(new_direction);
    }

    fn direction_by_number(direction: u8) -> Vector2<i8> {
        match direction {
            0 => Vector2 { x: 0, y: 1 },
            1 => Vector2 { x: 1, y: 1 },
            2 => Vector2 { x: 1, y: 0 },
            3 => Vector2 { x: 1, y: -1 },
            4 => Vector2 { x: 0, y: -1 },
            5 => Vector2 { x: -1, y: -1 },
            6 => Vector2 { x: -1, y: 0 },
            7 => Vector2 { x: -1, y: 1 },
            _ => Vector2 {x: 0, y: 0},
        }
    }
}

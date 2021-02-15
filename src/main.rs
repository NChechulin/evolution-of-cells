mod cell;

use ggez::*;
use std::time::Duration;
use ggez::graphics::{DrawMode, Rect, Mesh};
use cell::Cell;

// width and height
const GRID_SIZE: (i32, i32) = (48, 27);
const GRID_CELL_SIZE: (i32, i32) = (20, 20);
const CELL_PADDING: i32 = 1;

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32 + (GRID_SIZE.0 * CELL_PADDING) as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32 + (GRID_SIZE.1 * CELL_PADDING) as f32,
);

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

struct GameState {
    cells: Vec<Cell>,
    meshes: Vec<graphics::Mesh>,
}

impl GameState {
    fn add_cell_to_mesh_builder(cell: &Cell, mb: &mut graphics::MeshBuilder) {
        let x_pos = cell.x_pos * GRID_CELL_SIZE.0 + cell.x_pos * CELL_PADDING;
        let y_pos = cell.y_pos * GRID_CELL_SIZE.1 + cell.y_pos * CELL_PADDING;

        let rect = graphics::Rect::new(x_pos as f32, y_pos as f32, GRID_CELL_SIZE.0 as f32, GRID_CELL_SIZE.1 as f32);
        mb.rectangle(
            DrawMode::fill(),
            rect,
            cell.color,
        );
    }

    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mb = &mut graphics::MeshBuilder::new();
        //
        // for row_num in 0..(GRID_SIZE.1) {
        //     for col_num in 0..(GRID_SIZE.0) {
        //         let x_pos = col_num * GRID_CELL_SIZE.0 + col_num * CELL_PADDING;
        //         let y_pos = row_num * GRID_CELL_SIZE.1 + row_num * CELL_PADDING;
        //
        //         let rect = graphics::Rect::new(x_pos as f32, y_pos as f32, GRID_CELL_SIZE.0 as f32, GRID_CELL_SIZE.1 as f32);
        //
        //         mb.rectangle(
        //             DrawMode::fill(),
        //             rect,
        //             graphics::Color::WHITE,
        //         );
        //     }
        // }

        let cells: Vec<Cell> = vec![
            Cell {
                x_pos: 0,
                y_pos: 0,
                color: graphics::Color::WHITE,
            },
            Cell {
                x_pos: 3,
                y_pos: 2,
                color: graphics::Color::from_rgb(60, 70, 90),
            },
            Cell {
                x_pos: 4,
                y_pos: 7,
                color: graphics::Color::from_rgb(255, 0, 0),
            }
        ];

        for cell in &cells {
            GameState::add_cell_to_mesh_builder(&cell, mb);
        }

        Ok(GameState {
            cells,
            meshes: vec![mb.build(ctx)?],
        })
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for cell_mesh in &self.meshes {
            graphics::draw(ctx, cell_mesh, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let (mut ctx, events_loop) = ggez::ContextBuilder::new("evolution-of-cells", "Nikolay Chechulin")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("Evolution of cells"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build().unwrap();


    // Next we create a new instance of our GameState struct, which implements EventHandler
    let mut state = GameState::new(&mut ctx).unwrap();

    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}

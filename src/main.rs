mod cell;

use cell::Cell;
use ggez::graphics::{DrawMode, Mesh, Rect};
use ggez::*;
use std::time::Duration;

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

        let rect = graphics::Rect::new(
            x_pos as f32,
            y_pos as f32,
            GRID_CELL_SIZE.0 as f32,
            GRID_CELL_SIZE.1 as f32,
        );
        mb.rectangle(DrawMode::fill(), rect, cell.color);
    }

    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mb = &mut graphics::MeshBuilder::new();
        let cells: Vec<Cell> = vec![
            Cell::new(0, 0, graphics::Color::WHITE, 0),
            Cell::new(3, 2, graphics::Color::from_rgb(60, 70, 90), 0),
            Cell::new(4, 7, graphics::Color::from_rgb(255, 0, 0), 0),
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
        self.cells[0].move_forward();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        for cell in &self.cells {
            cell.draw(ctx);
        }

        // for cell_mesh in &self.meshes {
        //     graphics::draw(ctx, cell_mesh, graphics::DrawParam::default())?;
        // }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let (mut ctx, events_loop) =
        ggez::ContextBuilder::new("evolution-of-cells", "Nikolay Chechulin")
            // Next we set up the window. This title will be displayed in the title bar of the window.
            .window_setup(ggez::conf::WindowSetup::default().title("Evolution of cells"))
            // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
            .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
            // And finally we attempt to build the context and create the window. If it fails, we panic with the message
            // "Failed to build ggez context"
            .build()
            .unwrap();

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let mut state = GameState::new(&mut ctx).unwrap();

    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}

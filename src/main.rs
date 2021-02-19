mod cell;

use cell::Cell;
use crate::cell::energy_constants::EnergyConstants;
use ggez::*;
use std::time::Duration;
use std::time::Instant;
use crate::cell::cell_methods::CellMethods;

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
    last_update: Instant,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let cells: Vec<Cell> = vec![
            Cell::new(0, 0, graphics::Color::WHITE, 0, EnergyConstants::new()),
            Cell::new(3, 2, graphics::Color::from_rgb(60, 70, 90), 2, EnergyConstants::new()),
            Cell::new(4, 7, graphics::Color::from_rgb(255, 0, 0), 3, EnergyConstants::new()),
        ];

        Ok(GameState {
            cells,
            last_update: Instant::now(),
        })
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // limit the FPS
        if Instant::now() - self.last_update < Duration::from_millis(MILLIS_PER_UPDATE) {
            return Ok(());
        }
        for cell in &mut self.cells {
            cell.execute_gene();
        }
        self.last_update = Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        for cell in &self.cells {
            cell.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let (ctx, events_loop) =
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
    let state = GameState::new().unwrap();

    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}

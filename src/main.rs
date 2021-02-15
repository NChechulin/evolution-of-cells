use ggez::*;
use std::time::Duration;
use ggez::graphics::{DrawMode, Rect};

// width and height
const GRID_SIZE: (i16, i16) = (80, 60);
const GRID_CELL_SIZE: (i16, i16) = (10, 10);
const CELL_PADDING: i16 = 1;

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32 + (GRID_SIZE.0 * CELL_PADDING) as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32 + (GRID_SIZE.1 * CELL_PADDING) as f32,
);

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

struct GameState {
    meshes: Vec<graphics::Mesh>,
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        let r1 =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::WHITE)?;
        graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
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
    let mut state = GameState { meshes: vec![] };

    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}

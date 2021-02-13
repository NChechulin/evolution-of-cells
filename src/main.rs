use ggez::*;
use std::time::Duration;

// width and height
const GRID_SIZE: (i16, i16) = (80, 60);
const GRID_CELL_SIZE: (i16, i16) = (10, 10);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() {
    let (ctx, events_loop) = ggez::ContextBuilder::new("evolution-of-cells", "Nikolay Chechulin")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("Evolution of cells"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build().unwrap();        println!("Hello ggez! dt = {}ns", self.dt.as_nanos());


    // Next we create a new instance of our GameState struct, which implements EventHandler
    let mut state = State { dt: std::time::Duration::new(0, 0) };
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}

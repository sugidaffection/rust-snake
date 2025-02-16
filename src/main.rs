use piston_window::*;

mod app;
mod food;
mod snake;
mod vec2;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;
const GRID_SIZE: f64 = 20.0;
const UPS: u64 = 60;
const FPS: u64 = 60;

fn main() -> Result<(), String> {
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32])
            .exit_on_esc(true)
            .build()
            .map_err(|e| format!("Failed to build PistonWindow: {}", e))?;

    window.set_ups(UPS);
    window.set_max_fps(FPS);
    let mut app = app::App::new(
        GRID_SIZE,
        Size {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        },
    );

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            app.render(c.transform, g);
        });

        if let Some(r) = e.render_args() {
            app.render_update(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_input(key);
        }
    }

    Ok(())
}

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::*;
use opengl_graphics::glyph_cache::GlyphCache;
use sdl2_window::Sdl2Window;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window =
        WindowSettings::new(
            "opengl_graphics: hello_world",
            [200, 200]
        )
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf").unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                let transform = c.transform.trans(10.0, 100.0);

                clear([0.0, 0.0, 0.0, 1.0], g);
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    "Hello world!",
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );
            });
        }
    }
}

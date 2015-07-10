extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use std::path::Path;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    dim: (u32, u32)
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        let (width, height) = self.dim;
        let ground = rectangle::centered([0.0, height as f64, (width as f64), 100.0]);
        //let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let image   = Image::new().rect(rectangle::centered([0.0, 0.0, 100.0, 200.0]));
        let texture = Texture::from_path(Path::new("sprites/person.png")).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLUE, gl);

            let transform: [[f64; 3]; 2] = c.transform.trans(0.0, 0.0);


            // Draw a box rotating around the middle of the screen.
            rectangle(GREEN, ground, transform, gl);
            image.draw(&texture, default_draw_state(), c.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        
    }
}

fn main() {
    let opengl = OpenGL::_3_2;
    let width = 600 as u32;
    let height = 400 as u32;

    // Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "rario",
            [width, height]
        )
        .exit_on_esc(true)
    );

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        dim: (width, height)
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
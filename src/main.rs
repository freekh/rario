extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use std::path::Path;
use piston::event::Event::Input;
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    dim: (u32, u32),
    x: f64, 
    y: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.4, 0.4, 1.0, 1.0];

        let (width, height) = self.dim;
        let groundHeight = 100.0;
        let ground = rectangle::centered([0.0, height as f64, width as f64, groundHeight]);
        //let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let scale = 0.40;

        let personHeight = 161.0*scale;
        let personWidth = 88.0*scale;

        let (personX, personY) = (self.x, self.y);
        let personY =  groundHeight + personHeight;

        let image   = Image::new().rect(rectangle::centered([personWidth/2.0, (height as f64) - personY, personWidth, personHeight]));
        let texture = Texture::from_path(Path::new("sprites/person.png")).unwrap();
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLUE, gl);

            let transform: [[f64; 3]; 2] = c.transform.trans(0.0, 0.0);
            let personTransform = c.transform.trans(personX, 0.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(GREEN, ground, transform, gl);
            image.draw(&texture, default_draw_state(), personTransform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        //let (personX, personY) = self.translation;        
        //self.translation = (personX + 1.0, 0.0);
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
        dim: (width, height),
        x: 0.0,
        y: 0.0
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        match e {
            piston::event::Event::Input(piston::input::Input::Press(b)) => {
                match b {
                    Keyboard(Up) => {
                        println!("JUMP");
                    }
                    Keyboard(Right) => {
                        app.x += 10.0;
                        println!("Move right");
                    }
                    Keyboard(Left) => {
                        app.x -= 10.0;
                        println!("Move left");
                    }
                    Keyboard(Down) => {
                        println!("crouch baby");
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }
}
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
    y: f64,
    deltaY: f64,
    deltaX: f64
}


impl App {
    
    
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.4, 0.4, 1.0, 1.0];

        let (width, height) = self.dim;
        let groundHeight = 100.0;
        let ground = rectangle::centered([0.0, height as f64, width as f64, groundHeight]);
        let boxX = width as f64 * 0.75;
        let boxWidth = 10.0;
        let boxHeight = 10.0;
        let boxY = height as f64 - groundHeight - boxHeight;
        let boxCrate = rectangle::centered([boxX, boxY, boxWidth, boxHeight]);
        //let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let scale = 0.40;

        let personHeight = 161.0*scale;
        let personWidth = 88.0*scale;

        let personX = self.x;
        let personY = self.y;

        let image   = Image::new().rect(rectangle::centered([personWidth/2.0, (height as f64) - groundHeight - personHeight, personWidth, personHeight]));
        let texture = Texture::from_path(Path::new("sprites/person.png")).unwrap();
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLUE, gl);

            let transform: [[f64; 3]; 2] = c.transform.trans(0.0, 0.0);
            let personTransform = c.transform.trans(personX, personY);

            // Draw a box rotating around the middle of the screen.
            rectangle(GREEN, ground, transform, gl);
            rectangle(RED, boxCrate, transform, gl);
            image.draw(&texture, default_draw_state(), personTransform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        fn intersect(person: [f64;4], boxCrate: [f64;4]) -> (f64, f64) {
            let (personX, personY, personWidth, personHeight) = (person[0], person[1], person[2], person[3]);
            fn corners(person: [f64;4]) -> (f64, f64, f64, f64) {
              let (personX, personY, personWidth, personHeight) = (person[0], person[1], person[2], person[3]);
              let personCornerLeft = personX;
              let personCornerRight = personX + personWidth;
              let personCornerTop = personY;
              let personCornerBottom = personY + personHeight;
              (personCornerLeft, personCornerRight, personCornerTop, personCornerBottom)
            }
            let (personCornerLeft, personCornerRight, personCornerTop, personCornerBottom) = corners(person);
            let (boxCornerLeft,  boxCornerRight, boxCornerTop, boxCornerBottom) = corners(boxCrate);
            let pointInBox = |x: f64, y: f64| -> bool {
                println!("{}, {}", x, boxCornerLeft);
                (x >= boxCornerLeft && x <= boxCornerRight) && (y <= boxCornerBottom && y >= boxCornerTop)
            };
            if (pointInBox(personCornerLeft, personCornerTop)) {
                (boxCornerLeft - personWidth, personY)
            } else if (pointInBox(personCornerRight, personCornerTop)) {
                (boxCornerRight + personWidth, personY)
            } else {
                (personX, personY)
            }
        }
        //println!("{}", self.y);
        
        if (self.y < 0.0) {
            self.y += self.deltaY;
            self.deltaY += 0.02;
        }
        self.x += self.deltaX;
        let scale = 0.40;
        let groundHeight = 100.0;

        let personHeight = 161.0*scale;
        let personWidth = 88.0*scale;
        let (width, height) = self.dim;
        let boxX = width as f64 * 0.75;
        let boxWidth = 10.0;
        let boxHeight = 10.0;
        let boxY = height as f64 - groundHeight - boxHeight;

        let (newX, newY) = intersect([self.x, self.y, personWidth, personHeight], [boxX, boxY, boxWidth, boxHeight]);
        self.x = newX;
        self.y = newY;
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
        y: 0.0,
        deltaY: 0.0,
        deltaX: 0.0
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
                        if (app.y >= 0.0) {
                            println!("JUMP");
                            app.deltaY = 0.0;
                            app.y = -50.0;
                        }
                    }
                    Keyboard(Right) => {
                        if (app.deltaX <= 1.5) {
                            app.deltaX += 0.5;
                        }
                        println!("Move right");
                    }
                    Keyboard(Left) => {
                        if (app.deltaX >= -1.5) {
                            app.deltaX -= 0.5;
                        }
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
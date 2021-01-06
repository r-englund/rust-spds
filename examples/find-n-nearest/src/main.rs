use glutin_window::GlutinWindow as Window;
//use graphics::{circle_arc, math::Scalar, types::Radius, CircleArc};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::Button::*;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{
    event_loop::{EventSettings, Events},
    ButtonEvent, ButtonState, Key, MouseCursorEvent,
};

use rand::{distributions::Uniform, prelude::*};

use spds::spatialdatastructure::SpatialDataStructure;

const W: u32 = 1200;
const H: u32 = 800;

pub struct App<SD>
where
    SD: SpatialDataStructure,
{
    gl: GlGraphics, // OpenGL drawing backend.
    points: SD,
    rand: ThreadRng,
    focus: SD::Point,
    find_n_points: usize,
}

impl<SD> App<SD>
where
    SD: SpatialDataStructure + Clone + IntoIterator,
{
    fn render(&mut self, args: &RenderArgs) {
        if self.points.length() < 2 * self.find_n_points {
            self.gen_points(self.find_n_points * 2 - self.points.length());
        }

        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 2.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        
        let dots = self.points.iter().map(|x| (x[0], x[1])).collect::<Vec<_>>();

        let highlighted_dots = self
            .points
            .find_n_nearest(&self.focus, self.find_n_points)
            .unwrap_or_default();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y);

            // for (px, py) in dots {
            //     let t = c.transform.trans(x + px as f64, y + py as f64);
            //     circle_arc(RED, 1.0, 0.0, 6.28, square, t, gl)
            // }

            for pos in highlighted_dots {
                // let px = pos[0];
                // let py = pos[1];
                // let t = c.transform.trans(x + px as f64, y + py as f64);
                // circle_arc(BLUE, 1.5, 0.0, 6.28, square, t, gl);
            }

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn gen_points(&mut self, n: usize) {
        let w2 = W as i32 / 2;
        let h2 = H as i32 / 2;
        let randx = Uniform::new(-w2, w2);
        let randy = Uniform::new(-h2, h2);

        let mut r = self.rand.clone();

        //for p in self.points.into_iter() {}

        // self.points.add_points((0..n).map(|_| {
        //     let x = r.sample(randx);
        //     let y = r.sample(randy);
        //     [x, y]
        // }));
    }

    fn update(&mut self, args: &UpdateArgs, gen_points: bool) {
        // Rotate 2 radians per second.

        const POINTS_PER_SECOND: usize = 1000;
        if gen_points {
            let n = (POINTS_PER_SECOND as f64 * args.dt) as usize;

            self.gen_points(n);
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [W, H])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        points: Vec::new(),
        rand: rand::thread_rng(),
        focus: [0, 0],
        find_n_points: 100,
    };

    let mut space_pressed = false;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            if let Keyboard(key) = args.button {
                if key == Key::Space {
                    space_pressed = args.state == ButtonState::Press;
                }
            }
        }

        if let Some(args) = e.mouse_cursor_args() {
            let x = args[0] as i32 - (W as i32 / 2);
            let y = args[1] as i32 - (H as i32 / 2);
            app.focus = [x, y];
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, space_pressed);
        }
    }
}

// use bufro::Color;
// use cgmath::VectorSpace;
// use winit::{
//     event::*,
//     event_loop::{ControlFlow, EventLoop},
//     window::WindowBuilder,
// };
// use std::time::Instant;
// use rand::RngCore;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

static opengl : OpenGL = OpenGL::V4_0;

pub struct Window {
    width : usize,
    height : usize,
    gl : GlGraphics,
    window : GlutinWindow,
    events : Events,
    tick : u64,
}

impl Window {
    pub fn new(width : usize, height : usize) -> Window {
        let mut events = Events::new(EventSettings::new());
        //events.set_max_fps(165);
        events.set_lazy(true);
        let mut window: GlutinWindow = WindowSettings::new(
            "Test Window!",
            [width as u32, height as u32]
        ).opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        Window {
            width,
            height,
            gl : GlGraphics::new(opengl),
            window,
            events,
            tick : 0,
        }
    }
    pub fn maintain(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }
        }
    }
    pub fn set_ups(&mut self, new_ups : u64) {
        self.events.set_ups(new_ups);
    }
    pub fn render(&mut self, args : &RenderArgs) {
        use graphics;
        let COOL_COLOR : [f32; 4] = [0.65, 0.85, 0.13, 1.0];
        let COOL_COLOR2 : [f32; 4] = [0.35, 0.15, 0.87, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            // if self.tick != 0 {
            graphics::clear(COOL_COLOR, gl);
            let mut rect = graphics::rectangle::square(0.0, 0.0, (self.tick % 1024) as f64);
            graphics::rectangle(COOL_COLOR2, rect, c.transform, gl)
            // }
        });
        self.tick += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut my_window = Window::new(800,600);
        my_window.set_ups(100);
        loop {
            my_window.maintain();
        }
    }
}
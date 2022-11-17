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
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

static opengl : OpenGL = OpenGL::V4_0;

pub struct Window {
    width : usize,
    height : usize,
    gl : GlGraphics,
    window : GlutinWindow,
    events : Events,
}

impl Window {
    pub fn new(width : usize, height : usize) -> Window {
        let mut events = Events::new(EventSettings::new());
        let mut window: GlutinWindow = WindowSettings::new(
            "Test Window!",
            [width, height]
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
        }
    }
    pub fn maintain(&mut self) {
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                myWindow.render(&r);
            }
        }
    }
    pub fn render(&mut self, args : &RenderArgs) {
        use graphics;
        let COOL_COLOR : [f32; 4] = [0.65, 0.85, 0.13, 1.0];
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(COOL_COLOR, gl);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut myWindow = Window::new(800,600);
        loop {
            myWindow
        }
    }
}
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

type RenderFunction = Box<dyn FnMut(u64, u64, graphics::Context, &mut GlGraphics)>;
type UpdateFunction = Box<dyn FnMut(u64)>;
static OPENGL_VER : OpenGL = OpenGL::V4_0;

pub struct Window {
    pub width : usize,
    pub height : usize,
    gl : Option<GlGraphics>,
    window : Option<GlutinWindow>,
    events : Events,
    rtick : u64,
    utick : u64,
    renderf : Option<Box<dyn FnMut(u64, u64, graphics::Context, &mut GlGraphics)>>,
    updatef : Option<Box<dyn FnMut(u64)>>,
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
impl Window {
    
    const FLAGLESS : u32 = 0;
    const EXIT_ON_ESC : u32 = 0x1;
    const RESIZABLE : u32 = 0x2;
    const VSYNC : u32 = 0x4;
    const DECORATED : u32 = 0x8;
    const FULLSCREEN : u32 = 0x16;
    const SRGB : u32 = 0x32;
    const CONTROLLERS : u32 = 0x64;
    const LAZY : u32 = 0x128;
    pub const DEFAULT_FLAGS : u32 = Self::EXIT_ON_ESC | Self::RESIZABLE | Self::VSYNC | Self::DECORATED;

    pub fn new(width : usize, height : usize) -> Window {
        //Create a new window
        //events.set_max_fps(165);
        //events.set_lazy(true);
        Self {
            width,
            height,
            gl : None,
            window : None,
            events : Events::new(EventSettings::new()),
            rtick : 0,
            utick : 0,
            renderf : None,
            updatef : None,
        }
    }
    pub fn begin(&mut self, title : String, FLAGS : u32) {
        //Begin the window by moving the cache to a active window on the screen
        //renderf and updatef should be set for the window's behavior
        self.window = Some(WindowSettings::new(
            "",
            [self.width as u32, self.height as u32],
        ).opengl(OPENGL_VER)
            //.resizable(FLAGS & Self::RESIZABLE != 0)
            .exit_on_esc(true)//FLAGS & Self::EXIT_ON_ESC != 0)
            .vsync(true)//FLAGS & Self::VSYNC != 0)
            .title(title)
            //.fullscreen(FLAGS & Self::FULLSCREEN != 0)
            .decorated(true)//FLAGS & Self::DECORATED != 0)
            //.controllers(FLAGS & Self::CONTROLLERS != 0)
            //.srgb(FLAGS & Self::SRGB != 0)
            .build().unwrap());
        self.events = self.events
            .lazy(false);//FLAGS | Self::LAZY != 0);
        self.gl = Some(GlGraphics::new(OPENGL_VER));
    }   
    // pub fn edit(&mut self, title : String, FLAGS : u32) {
    //     //Update the window flags 
    // }
    pub fn init(width : usize, height : usize, renderf : RenderFunction, updatef : UpdateFunction) -> Window {
        let mut ret = Window::new(width, height);
        ret.renderf = Some(renderf);
        ret.updatef = Some(updatef);
        ret
    }
    pub fn maintain(&mut self) {
        if self.window.is_none() {
            return;
        }
        //print_type_of(&mut self.window)
        while let Some(e) = self.events.next(self.window.as_mut().expect("no window")) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }
            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }
    pub fn set_ups(&mut self, new_ups : u64) -> Option<()>{
        self.events.set_ups(new_ups);
        Some(())
    }
    pub fn set_fps(&mut self, new_fps : u64) -> Option<()>{
        self.events.set_max_fps(new_fps);
        Some(())
    }
    pub fn render(&mut self, args : &RenderArgs) {
        use graphics;
        if let Some(f) = &mut self.renderf {
            self.gl.as_mut().expect("Window Not Initialized").draw(args.viewport(), |c, gl| {f(self.rtick, self.utick, c, gl)});
        }
        self.rtick += 1;
    }
    pub fn update(&mut self, args : &UpdateArgs) {
        use graphics;
        if let Some(f) = &mut self.updatef {
            f(self.utick);
        }
        self.utick += 1;
    }
    pub fn set_render_fn(&mut self, new_renderf : Box<dyn FnMut(u64, u64, graphics::Context, &mut GlGraphics)>) {
        self.renderf = Some(new_renderf);
    }
    pub fn set_update_fn(&mut self, new_updatef : Box<dyn FnMut(u64)>) {
        self.updatef = Some(new_updatef);
    }
}

impl Default for Window {
    fn default() -> Self {
        let renderf = |rtick : u64, utick : u64, c : graphics::Context, gl : &mut GlGraphics| {
            const COOL_COLOR : [f32; 4] = [0.65, 0.85, 0.13, 1.0];
            const COOL_COLOR2 : [f32; 4] = [0.35, 0.15, 0.87, 1.0];
            // if self.tick != 0 {
            graphics::clear(COOL_COLOR, gl);
            let rect = graphics::rectangle::square(0.0, 0.0, ((rtick + utick) % 1024) as f64);
            graphics::rectangle(COOL_COLOR2, rect, c.transform, gl)
            // }
        };
        let updatef = |utick : u64|{
            
        };
        Self::init(640, 480, Box::new(renderf), Box::new(updatef))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //let mut my_window = Window::new(3,3);
        let mut my_window = Window::default();
        my_window.begin("Yeet".to_string(), Window::DEFAULT_FLAGS);
        loop {
            my_window.maintain();
        }
    }
}
use super::kinematics::*;
const GRAVITY_MPS2: f32 = -9.81;

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    mass: f32,
    fx: fn(f32) -> f32, //respect to time
    fy: fn(f32) -> f32,
    cached_x_Function : Box<dyn Function>,
    cached_y_Function : Box<dyn Function>,
}

struct Space {
    time_units : Units,
    space_units : Units,
    mass_units : Units,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    floor: f32,
    ax: fn((f32, f32)) -> f32,
    ay: fn((f32, f32)) -> f32,
    pixelx: fn(f32) -> usize,

    elapsed_time: f32,
    pub balls: Vec<Ball>,
}

impl Space {
    fn tick(&mut self, dt: f32) {
        self.elapsed_time += dt;
        for ball in &mut self.balls {
            //keep track of the cached calculus functions
            //check if last acceleration for ball was different and then recompile the cached calculus polynomial if so
            let (x, y) = ball.position(self.elapsed_time);
        }
    }
}
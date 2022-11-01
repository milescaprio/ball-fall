use super::kinematics;
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

struct Angle {
    deg : f32,
}

impl Angle {
    fn new(deg : f32) -> Angle {
        Angle { deg }
    }
    fn xy_h(&self, h : f32) -> (f32, f32) {
        let rad = self.deg.to_radians();
        (rad.cos() * h, rad.sin() * h);
    }
}

enum accel_xy_function {
    ParterFunctionVector(Box<dyn kinematics::Function>, Angle),
    IndependentFunctions(Box<dyn kinematics::Function>, Box<dyn kinematics::Function>),
    CompositeAcceleration(Self::ParterFunctionVector, Self::IndependentFunctions),
}

impl Ball {
    fn soft_update_quick(&mut self) {
        fx = cached_x_Function.quick_compile();
        fy = cached_y_Function.quick_compile();
        self.x = x;
        self.y = y;
    }
    fn soft_update(&mut self) -> Result<(), FunctionInternalError> {
        fx = cached_x_Function.compile()?;
        fy = cached_y_Function.compile()?;
        self.x = x;
        self.y = y;
    }
    fn hard_update_quick(&mut self) {
        if let accel_xy_function::ParterFunctionVector(f, a) = self.fx {
            let cached_d_Function = f .integrate().unwrap().integrate().unwrap();
            cached_x_Function = cached_d_Function.mult_const(a.xy_h(1).0);
            cached_y_Function = cached_d_Function.mult_const(a.xy_h(1).1);
        }
        else if let accel_xy_function::IndependentFunctions(fx, fy) = self.fx {
            cached_x_Function = fx.integrate().unwrap().integrate().unwrap();
            cached_y_Function = fy.integrate().unwrap().integrate().unwrap();
            self.soft_update_quick();
        }
        else if let accel_xy_function::CompositeAcceleration(f, a) = self.fx {
            //idk yet
        }
        self.cached_x_Function = ;
        self.cached_y_Function = Box::new(fy);
        self.x = x;
        self.y = y;
    }
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
    a : accel_xy_function,
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
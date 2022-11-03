use super::kinematics;
use kinematics::Function;
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
    cached_x_Function : Box<dyn kinematics::Function>,
    cached_y_Function : Box<dyn kinematics::Function>,
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
    CompositeAcceleration(accel_xy_function, accel_xy_function),
}

impl Ball {
    use 
    pub fn soft_update_unchecked(&mut self) { 
        //use when data hasnt been recently injected and checking isn't worth
        //not checking won't result in unsafe code but could create odd function behavior
        fx = cached_x_Function.compile_unchecked();
        fy = cached_y_Function.compile_unchecked();
        self.x = x;
        self.y = y;
    }
    pub fn soft_update(&mut self) -> Result<(), FunctionInternalError> {
        fx = cached_x_Function.compile()?;
        fy = cached_y_Function.compile()?;
        self.x = x;
        self.y = y;
    }
    pub fn hard_update_unchecked(&mut self) {
        todo!()
        if let accel_xy_function::ParterFunctionVector(a, d) = self.a {
            let cached_d_Function = a .integrate().unwrap().integrate().unwrap();
            self.cached_x_Function = cached_d_Function.mult_const(d.xy_h(1).0);
            self.cached_y_Function = cached_d_Function.mult_const(d.xy_h(1).1);
            self.soft_update_unchecked();
        }
        else if let accel_xy_function::IndependentFunctions(ax, ay) = self.a {
            self.cached_x_Function = ax.integrate().unwrap().integrate().unwrap();
            self.cached_y_Function = ay.integrate().unwrap().integrate().unwrap();
            self.soft_update_unchecked();
        }
        //else if let accel_xy_function::CompositeAcceleration(a1, a2) = self.a {
            //self.cached_x_Function = kinematics::SumFunction()
        //}
    }
    fn recurhelper_hard_update_unchecked(&accel_xy_function : a_ref) -> (Box<dyn kinematics::Function>, Box<dyn kinematics::Function>) {
        let mut ret : Box<dyn kinematics::, kinematics::Polynomial::new()
        if let accel_xy_function::ParterFunctionVector(a, d) = a_ref {
            let cached_d_Function = a .integrate().unwrap().integrate().unwrap();
            self.cached_x_Function = cached_d_Function.mult_const(d.xy_h(1).0);
            self.cached_y_Function = cached_d_Function.mult_const(d.xy_h(1).1);
            self.soft_update_unchecked();
        }
        else if let accel_xy_function::IndependentFunctions(ax, ay) = self.a {
            self.cached_x_Function = ax.integrate().unwrap().integrate().unwrap();
            self.cached_y_Function = ay.integrate().unwrap().integrate().unwrap();
            self.soft_update_unchecked();
        }
        //else if let accel_xy_function::CompositeAcceleration(a1, a2) = self.a {
            //self.cached_x_Function = kinematics::SumFunction()
        //}
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

    elapsed: f32,
    pub balls: Vec<Ball>,
}

impl Space {
    fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
        for ball in &mut self.balls {
            //keep track of the cached calculus functions
            //check if last acceleration for ball was different and then recompile the cached calculus polynomial if so
            let (x, y) = (ball.fx(self.elapsed), ball.fy(self.elapsed);
        }
    }
}
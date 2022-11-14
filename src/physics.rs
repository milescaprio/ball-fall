use super::kinematics;
use kinematics::Function;
use kinematics::CalcFunction;
use kinematics::SumFunction;
use kinematics::Unit;
use kinematics::Units;
const GRAVITY_MPS2: f32 = -9.81;

struct FunctionCache {
    pub closure: fn(f32) -> f32,
}

impl Default for FunctionCache {
    fn default() -> Self {
        Self {
            closure: |x| x,
        }
    }
}



#[derive(Default)]
struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    mass: f32,
    fx: FunctionCache, //respect to time
    fy: FunctionCache,
    cached_x_Function : Box<dyn Function>,
    cached_y_Function : Box<dyn Function>,
}

struct Angle {
    deg : f32,
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
    //pixelx: fn(f32) -> usize,
    elapsed: f32,
    pub balls: Vec<Ball>,
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
    ParterFunctionVector(Box<dyn Function>, Angle),
    IndependentFunctions(Box<dyn Function>, Box<dyn Function>),
    CompositeAcceleration(accel_xy_function, accel_xy_function),
}

impl Ball {
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
    pub fn hard_update_unchecked(&mut self, a_ref : &accel_xy_function) {
        if let accel_xy_function::ParterFunctionVector(a, d) = a_ref {
            let cached_d_Function = a .integrate().unwrap().integrate().unwrap();
            self.cached_x_Function = cached_d_Function.mult_const(d.xy_h(1).0);
            self.cached_y_Function = cached_d_Function.mult_const(d.xy_h(1).1);
            self.soft_update_unchecked();
        }
        else if let accel_xy_function::IndependentFunctions(ax, ay) = a_ref {
            self.cached_x_Function = ax.integrate().unwrap().integrate().unwrap();
            self.cached_y_Function = ay.integrate().unwrap().integrate().unwrap();
            self.soft_update_unchecked();
        } else if let accel_xy_function::CompositeAcceleration(a1, a2) = a_ref {
            //we have two accelerations and need to integrate each parts individually, both xnet and ynet
            //we calculate x and y from a1 and x and y from a2 recursively then throw them into a self.cached_x_Function and self.cached_y_Function
            xy1 = self.recurhelper_hard_update_unchecked(a1);
            xy2 = self.recurhelper_hard_update_unchecked(a2);
            self.cached_x_Function = kinematics::SumFunction(xy1.0,xy2.0);
            self.cached_y_Function = kinematics::SumFunction(xy1.1,xy2.1);
            self.soft_update_unchecked();
        }
    }
    fn recurhelper_hard_update_unchecked(&self, a_ref : &accel_xy_function) -> (Box<dyn Function>, Box<dyn Function>) {
        if let accel_xy_function::ParterFunctionVector(a, d) = a_ref {
            let cached_d_Function = a .integrate().unwrap().integrate().unwrap();
            return (
                cached_d_Function.mult_const(d.xy_h(1).0),
                cached_d_Function.mult_const(d.xy_h(1).1)
            );
        }
        else if let accel_xy_function::IndependentFunctions(ax, ay) = a_ref {
            return (
                ax.integrate().unwrap().integrate().unwrap(),
                ay.integrate().unwrap().integrate().unwrap()
            );
        }
        else if let accel_xy_function::CompositeAcceleration(a1, a2) = self.a {
            xy1 = self.recurhelper_hard_update_unchecked(a1);
            xy2 = self.recurhelper_hard_update_unchecked(a2);
            (
                kinematics::SumFunction(xy1.0,xy2.0),
                kinematics::SumFunction(xy1.1,xy2.1)
            )
        }
    }
    // fn recurhelper_soft_update_unchecked(&accel_xy_function : a_ref) -> Result<(Box<dyn Function>, Box<dyn Function>), FunctionInternalError> {\
    //     //this never work i was just thinking about it lol
    //     let mut ret_xy : (Box<dyn Function>, Box<dyn Function>); 
    //     if let accel_xy_function::ParterFunctionVector(a, d) = a_ref {
    //         let cached_d_Function = a .integrate().unwrap().integrate().unwrap();
    //         ret_xy.0 = cached_d_Function.mult_const(d.xy_h(1).0);
    //         ret_xy.1 = cached_d_Function.mult_const(d.xy_h(1).1);
    //         self.soft_update_unchecked();
    //     }
    //     else if let accel_xy_function::IndependentFunctions(ax, ay) = a_ref {
    //         ret_xy.0 = ax.integrate().unwrap().integrate().unwrap();
    //         ret_xy.1 = ay.integrate().unwrap().integrate().unwrap();
    //         self.soft_update_unchecked();
    //     }
    //     //else if let accel_xy_function::CompositeAcceleration(a1, a2) = self.a {
    //         //self.cached_x_Function = kinematics::SumFunction()
    //     //}
    // }
}

impl Space {
    fn blank(a : accel_xy_function) -> Space {
        Space {
            x1 : -10,
            x2 : 10,
            y1 : -10,
            y2 : 10,
            time_units : Unit::S.units(),
            space_units : Unit::M.units(),
            mass_units : Unit::KG.units(),
            a,
            //pixelx : fn(m : f32) -> usize { (m * 1000.0) as usize }, //space is a meter by a meter
            elapsed : 0.0,
            balls : Vec::new(),
        }
    }

    fn new_ball_unchecked(&mut self, x : f32, y : f32, r : f32, m : f32){
        let mut ret = Ball::default();
        ret.x = x;
        ret.y = y;
        ret.radius = r;
        ret.mass = m;
        ret.hard_update_unchecked(&a);
        self.balls.push(ret);
    }

    fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
        for ball in &mut self.balls {
            //keep track of the cached calculus functions
            //check if last acceleration for ball was different and then recompile the cached calculus polynomial if so
            let (x, y) = (ball.fx(self.elapsed), ball.fy(self.elapsed));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gravity_space() {
        let mut myspace : Space = Space::blank(accel_xy_function::IndependentFunctions(Box::new(kinematics::ConstantFunction(0.0)), Box::new(kinematics::ConstantFunction(GRAVITY_MPS2))));
        myspace.new_ball_unchecked(0.0, 0.0, 1.0, 1.0);
        myspace.tick(1.0);
    }
}
use super::kinematics;
use kinematics::Function;
use kinematics::SumFunction;
use kinematics::Polynomial;
use kinematics::Unit;
use kinematics::Units;
use kinematics::Var;
use kinematics::EvalFunctionError;
pub const GRAVITY_MPS2: f32 = -9.81;

struct FunctionCache {
    pub closure: Box<dyn Fn(f32) -> Result<f32, EvalFunctionError>>,
}

impl Default for FunctionCache {
    fn default() -> Self {
        Self {
            closure: Box::new(|x| Ok(x)),
        }
    }
}

impl FunctionCache {
    pub fn new(closure: Box<dyn Fn(f32) -> Result<f32, EvalFunctionError>>) -> Self {
        Self {
            closure,
        }
    }
}

#[derive(Default)]
pub struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    mass: f32,
    fx: FunctionCache, //respect to time
    fy: FunctionCache,
    cached_x_Function : Option<Box<dyn Function>>,
    cached_y_Function : Option<Box<dyn Function>>,
}

pub struct Angle {
    deg : f32,
}

pub struct Space {
    time_units : Units,
    space_units : Units,
    mass_units : Units,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    floor: f32,
    a : AccelxyFunction,
    //pixelx: fn(f32) -> usize,
    elapsed: f32,
    pub balls: Vec<Ball>,
    //pub last_ball_poses : Vec<(f32, f32)>,
}

impl Angle {
    fn new(deg : f32) -> Angle {
        Angle { deg }
    }
    fn xy_h(&self, h : f32) -> (f32, f32) {
        let rad = self.deg.to_radians();
        (rad.cos() * h, rad.sin() * h)
    }
}

pub enum AccelxyFunction {
    ParterFunctionVector(Box<dyn Function>, Angle),
    IndependentFunctions(Box<dyn Function>, Box<dyn Function>),
    CompositeAcceleration(Box<AccelxyFunction>, Box<AccelxyFunction>),
}

impl Ball {
    pub fn soft_update_unchecked(&mut self) { 
        //use when data hasnt been recently injected and checking isn't worth
        //not checking won't result in unsafe code but could create odd function behavior
        self.fx = FunctionCache::new(self.cached_x_Function.as_ref().expect("No cache, unable to soft update!").compile_unchecked());
        self.fy = FunctionCache::new(self.cached_y_Function.as_ref().expect("No cache, unable to soft update!").compile_unchecked());
        // self.x = x;
        // self.y = y;
    }
    pub fn soft_update(&mut self) -> Result<(), kinematics::FunctionInternalError> {
        let fcx = self.cached_x_Function.as_ref().expect("No cache, unable to soft update!").compile()?;
        let fcy = self.cached_y_Function.as_ref().expect("No cache, unable to soft update!").compile()?;
        self.fx = FunctionCache::new(fcx);
        self.fy = FunctionCache::new(fcy);
        // if let Ok(fc) = self.cached_x_Function.as_ref().expect("No cache, unable to soft update!").compile() {
        //     self.fx = fc;
        // } else {
        //     return Err(kinematics::FunctionInternalError::new("Unable to compile x function!"));
        // }
        // if let Ok(fc) = self.cached_y_Function.as_ref().expect("No cache, unable to soft update!").compile() {
        //     self.fy = fc;
        // }
        Ok(())
        // self.x = x;
        // self.y = y;
    }
    fn recurhelper_hard_update_unchecked(&self, a_ref : &AccelxyFunction) -> (Box<dyn Function>, Box<dyn Function>) {
        if let AccelxyFunction::ParterFunctionVector(a, d) = a_ref {
            let cached_d_Function = a .integrated(Var::S).unwrap().integrated(Var::S).unwrap();
            return (
                cached_d_Function.stretch_vert(d.xy_h(1.).0),
                cached_d_Function.stretch_vert(d.xy_h(1.).1)
            );
        }
        else if let AccelxyFunction::IndependentFunctions(ax, ay) = a_ref {
            return (
                ax.integrated(Var::X).unwrap().integrated(Var::X).unwrap(),
                ay.integrated(Var::Y).unwrap().integrated(Var::Y).unwrap()
            );
        }
        else if let AccelxyFunction::CompositeAcceleration(a1, a2) = a_ref {
            let xy1 = self.recurhelper_hard_update_unchecked(&a1);
            let xy2 = self.recurhelper_hard_update_unchecked(&a2);
            (
                Box::new(SumFunction::from_compatible(xy1.0,xy2.0).unwrap()),
                Box::new(SumFunction::from_compatible(xy1.1,xy2.1).unwrap()),
            )
        }
        else {
            panic!("Unknown AccelxyFunction variant!");
        }
    }
    pub fn hard_update_unchecked(&mut self, a_ref : &AccelxyFunction) {
        use AccelxyFunction::*;
        match a_ref {
            ParterFunctionVector(a, d) => {
                let cached_d_Function = a .integrated(Var::X).unwrap().integrated(Var::Y).unwrap();
                self.cached_x_Function = Some(cached_d_Function.stretch_vert(d.xy_h(1.0).0));
                self.cached_y_Function = Some(cached_d_Function.stretch_vert(d.xy_h(1.0).1));
                self.soft_update_unchecked();
            }
            IndependentFunctions(ax, ay) => {
                self.cached_x_Function = Some(ax.integrated(Var::T).unwrap().integrated(Var::T).unwrap());
                self.cached_y_Function = Some(ay.integrated(Var::T).unwrap().integrated(Var::T).unwrap());
                self.soft_update_unchecked();
            }
            CompositeAcceleration(a1, a2) => {
                //we have two accelerations and need to integrated each parts individually, both xnet and ynet
                //we calculate x and y from a1 and x and y from a2 recursively then throw them into a self.cached_x_Function and self.cached_y_Function
                let xy1 = self.recurhelper_hard_update_unchecked(a1);
                let xy2 = self.recurhelper_hard_update_unchecked(a2);
                self.cached_x_Function = Some(Box::new(SumFunction::from_compatible(xy1.0,xy2.0).unwrap()));
                self.cached_y_Function = Some(Box::new(SumFunction::from_compatible(xy1.1,xy2.1).unwrap()));
                self.soft_update_unchecked();
            }
        }
    }
    pub fn curr_x(&self) -> f32 {
        self.x
    }
    pub fn curr_y(&self) -> f32 {
        self.y
    }
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

impl Space {
    pub fn blank(a : AccelxyFunction) -> Space {
        Space {
            x1 : -10.0,
            x2 : 10.0,
            y1 : -10.0,
            y2 : 10.0,
            floor : 0.0,
            time_units : Unit::S.units(),
            space_units : Unit::M.units(),
            mass_units : Unit::KG.units(),
            a,
            //pixelx : fn(m : f32) -> usize { (m * 1000.0) as usize }, //space is a meter by a meter
            elapsed : 0.0,
            balls : Vec::new(),
        }
    }

    pub fn new_ball_unchecked(&mut self, x : f32, y : f32, r : f32, m : f32){
        let mut ret = Ball::default();
        ret.x = x;
        ret.y = y;
        ret.radius = r;
        ret.mass = m;
        ret.hard_update_unchecked(&self.a);
        self.balls.push(ret);
    }

    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
        for ball in &mut self.balls {
            //keep track of the cached calculus functions
            //check if last acceleration for ball was different and then recompile the cached calculus polynomial if so
            let (x, y) = ((ball.fx.closure)(self.elapsed), (ball.fy.closure)(self.elapsed));
            ball.x = x.unwrap();
            ball.y = y.unwrap();
            if ball.x < self.x1 {
                ball.cached_x_Function = Some(ball.cached_x_Function.as_ref().expect("No cache, unable to soft update!").flip(self.elapsed));
            }
            if ball.y < self.y1 {
                ball.cached_y_Function = Some(ball.cached_y_Function.as_ref().expect("No cache, unable to soft update!").flip(self.elapsed));
            }
        }
    }

    pub fn get_elapsed(&self) -> f32 {
        self.elapsed
    }
}

#[cfg(test)]
mod tests {
    use crate::kinematics::Monomial;

    use super::*;
    #[test]
    fn gravity_space() {
        let mps2 : Units = Unit::M.units() / Unit::S.units() / Unit::S.units();
        let noaccel = Monomial::init(0.0, mps2, 0);
        let g = Monomial::init(GRAVITY_MPS2, mps2, 0);
        let mut myspace = Space::blank(
            AccelxyFunction::IndependentFunctions(
                Box::new(
                    Polynomial::init(
                        Var::T,
                        Unit::S.units(),
                        mps2, 
                        vec![noaccel]
                    )
                ), 
                Box::new(
                    Polynomial::init(
                        Var::T,
                        Unit::S.units(),
                        mps2, 
                        vec![g]
                    )
                )
            )
        ); 
        myspace.new_ball_unchecked(0.0, 0.0, 1.0, 1.0);
        myspace.tick(1.0);
    }
}
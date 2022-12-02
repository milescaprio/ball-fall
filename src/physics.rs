use super::kinematics;
use kinematics::Function;
use kinematics::SumFunction;
use kinematics::Polynomial;
use kinematics::Unit;
use kinematics::Units;
use kinematics::Var;
use kinematics::EvalFunctionError;
pub const GRAVITY_MPS2: f32 = -100.81;

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

pub enum MaybeNew {
    Update(f32),
    NoUpdate,
}

pub struct Recalculate {
    pub val : (MaybeNew, MaybeNew),
}

impl Recalculate {
    pub fn x(t : f32) -> Self {
        Self {
            val : (MaybeNew::Update(t), MaybeNew::NoUpdate),
        }
    }
    pub fn y(t : f32) -> Self {
        Self {
            val : (MaybeNew::NoUpdate, MaybeNew::Update(t)),
        }
    }
    pub fn xy(t : f32, u : f32) -> Self {
        Self {
            val : (MaybeNew::Update(t), MaybeNew::Update(u)),
        }
    }
    pub fn no_update() -> Self {
        Self {
            val : (MaybeNew::NoUpdate, MaybeNew::NoUpdate),
        }
    }
}

#[derive(Default)]
pub struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    mass: f32,
    bounce: f32,
    fx: FunctionCache, //respect to time
    fy: FunctionCache,
    cached_x_dyn_function : Option<Box<dyn Function>>,
    cached_y_dyn_function : Option<Box<dyn Function>>,
    x_reftime : f32,
    y_reftime : f32,
    color : [f32; 4],
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
    pub floor: f32,
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
        self.fx = FunctionCache::new(self.cached_x_dyn_function.as_ref().expect("No cache, unable to soft update!").compile_unchecked());
        self.fy = FunctionCache::new(self.cached_y_dyn_function.as_ref().expect("No cache, unable to soft update!").compile_unchecked());
        // self.x = x;
        // self.y = y;
    }
    pub fn soft_update(&mut self) -> Result<(), kinematics::FunctionInternalError> {
        let fcx = self.cached_x_dyn_function.as_ref().expect("No cache, unable to soft update!").compile()?;
        let fcy = self.cached_y_dyn_function.as_ref().expect("No cache, unable to soft update!").compile()?;
        self.fx = FunctionCache::new(fcx);
        self.fy = FunctionCache::new(fcy);
        Ok(())
    }
    fn recurhelper_hard_update_unchecked(&self, a_ref : &AccelxyFunction) -> (Box<dyn Function>, Box<dyn Function>) {
        if let AccelxyFunction::ParterFunctionVector(a, d) = a_ref {
            let cached_d_dyn_function = a .integrated(Var::S).unwrap().integrated(Var::S).unwrap();
            return (
                cached_d_dyn_function.stretch_vert(d.xy_h(1.).0),
                cached_d_dyn_function.stretch_vert(d.xy_h(1.).1)
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
    pub fn hard_update_unchecked(&mut self, a_ref : &AccelxyFunction, xi : f32, yi : f32, vxi : f32, vyi : f32, t : Recalculate) {
        use AccelxyFunction::*;
        match a_ref {
            ParterFunctionVector(a, d) => {
                todo!();
                let cached_d_dyn_function = a .integrated(Var::X).unwrap().integrated(Var::Y).unwrap();
                self.cached_x_dyn_function = Some(cached_d_dyn_function.stretch_vert(d.xy_h(1.0).0));
                self.cached_y_dyn_function = Some(cached_d_dyn_function.stretch_vert(d.xy_h(1.0).1));
                self.soft_update_unchecked();
            }
            IndependentFunctions(ax, ay) => {
                if let MaybeNew::Update(x) = t.val.0 {
                    self.x_reftime = x; 
                    self.cached_x_dyn_function = Some(ax.integrated_c(Var::T, vxi).expect("Integration Error").integrated_c(Var::T, xi).expect("Integration Error"));
                }
                if let MaybeNew::Update(y) = t.val.1 {
                    self.y_reftime = y; 
                    self.cached_y_dyn_function = Some(ay.integrated_c(Var::T, vyi).expect("Integration Error").integrated_c(Var::T, yi).expect("Integration Error"));
                }
                self.soft_update_unchecked();
            }
            CompositeAcceleration(a1, a2) => {
                todo!();
                //we have two accelerations and need to integrated each parts individually, both xnet and ynet
                //we calculate x and y from a1 and x and y from a2 recursively then throw them into a self.cached_x_dyn_function and self.cached_y_dyn_function

                //todo: initial velocities work here
                let xy1 = self.recurhelper_hard_update_unchecked(a1);
                let xy2 = self.recurhelper_hard_update_unchecked(a2);
                self.cached_x_dyn_function = Some(Box::new(SumFunction::from_compatible(xy1.0,xy2.0).unwrap()));
                self.cached_y_dyn_function = Some(Box::new(SumFunction::from_compatible(xy1.1,xy2.1).unwrap()));
                self.soft_update_unchecked();
            }
        }
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    pub fn get_mass(&self) -> f32 {
        self.mass
    }
    pub fn get_bounce(&self) -> f32 {
        self.bounce
    }
    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }
    pub fn get_vx(&self, t : f32) -> f32 {
        self.cached_x_dyn_function.as_ref()
            .expect("No cache, unable to get vx!")
            .differentiated(Var::T)
            .expect("Differentiation Error")
            .compile()
            .expect("Compilation Error")
            (t - self.x_reftime)
            .expect("Evaluation Error")
    }
    pub fn get_vy(&self, t : f32) -> f32 {
        self.cached_y_dyn_function.as_ref()
            .expect("No cache, unable to get vy!")
            .differentiated(Var::T)
            .expect("Differentiation Error")
            .compile()
            .expect("Compilation Error")
            (t - self.y_reftime)
            .expect("Evaluation Error")
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

    pub fn new_ball_unchecked(&mut self, x : f32, y : f32, vxi : f32, vyi : f32, r : f32, m : f32, b : f32, color : [f32; 4]) {
        let mut ret = Ball::default();
        ret.x = x;
        ret.y = y;
        ret.radius = r;
        ret.mass = m;
        ret.color = color;
        ret.bounce = b;
        ret.hard_update_unchecked(&self.a, ret.x, ret.y, vxi, vyi, Recalculate::xy(0.0,0.0));
        self.balls.push(ret);
    }

    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
        for ball in &mut self.balls {
            //keep track of the cached calculus functions
            //check if last acceleration for ball was different and then recompile the cached calculus polynomial if so
            let (x, y) = ((ball.fx.closure)(self.elapsed - ball.x_reftime), (ball.fy.closure)(self.elapsed - ball.y_reftime));
            ball.x = x.unwrap();
            ball.y = y.unwrap();
            if ball.x < self.x1 || ball.x > self.x2 {
                // ball.cached_x_dyn_function = Some(ball.cached_x_dyn_function.as_ref().expect("No cache, unable to soft update!").flip(self.elapsed));
                ball.hard_update_unchecked(&self.a, ball.get_x(), ball.get_y(), -ball.get_vx(self.elapsed), ball.get_vy(self.elapsed), Recalculate::x(self.elapsed));
            }
            if ball.y < self.floor {
                // ball.cached_y_dyn_function = Some(ball.cached_y_dyn_function.as_ref().expect("No cache, unable to soft update!").flip(self.elapsed));
                ball.hard_update_unchecked(&self.a, ball.get_x(), ball.get_y(), ball.get_vx(self.elapsed), -ball.get_vy(self.elapsed), Recalculate::y(self.elapsed));
            }
            ball.soft_update_unchecked();
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
        myspace.x1 = -20;
        myspace.x2 = 20;
        myspace.y1 = -10;
        myspace.y2 = 30;
        myspace.new_ball_unchecked(0.0, 25.0, 1.0, 1.0);
        myspace.new_ball_unchecked(2.0, 13.0, 0.5, 0.5);
        myspace.new_ball_unchecked(2.0, 16.0, 3.0, 5.0);

        myspace.tick(1.0);
    }
}
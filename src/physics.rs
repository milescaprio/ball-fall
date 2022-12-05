#![allow(dead_code)]
use super::kinematics;
use kinematics::Function;
use kinematics::SumFunction;
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
    ground_bounce: f32,
    free_bounce: f32,
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
    pub fn hard_update(&mut self, a_ref : &AccelxyFunction, xi : f32, yi : f32, vxi : f32, vyi : f32, t : Recalculate) {
        //todo: make this checked, finish hard_update_unchecked, and start to prefer the checked versions
        self.hard_update_unchecked(a_ref, xi, yi, vxi, vyi, t);
    }




    fn collision_vs(m1 : f32, m2 : f32, v1 : f32, v2 : f32) -> (f32,f32) {
        //calculates exit velocities of two objects colliding in one dimension
        (2.0*m1*v1/(m1+m2) - (m1-m2)/(m1+m2)*v2, 2.0*m2*v2/(m1+m2) + (m1-m2)/(m1+m2)*v1)
    }
    pub fn collide(&mut self, other : &mut Ball, t : f32) {
        //take two balls and bounce them from each other, assuming they are touching
        
        //find angle measures of the two balls
        let (b1vx, b1vy, b2vx, b2vy) = (self.get_vx(t), self.get_vy(t), other.get_vx(t), other.get_vy(t));
        let (b1v, b2v) = (b1vx.hypot(b1vy), b2vx.hypot(b2vy)); //pythagorean theorem, v magnitude
        let ball_centers_angle = (other.y-self.y).atan2(other.x-self.x); //gets direction of centers from self to other (b1 to b2)
        let b1_v_angle = b1vy.atan2(b1vx);
        let b2_v_angle = b2vy.atan2(b2vx);
        let b1_v_angle_relative = b1_v_angle - ball_centers_angle;
        let b2_v_angle_relative = b2_v_angle - ball_centers_angle;
        
        //find final velocities along this axis
        let (b1vr, b2vr) = (b1v * b1_v_angle_relative.cos(), b2v * b2_v_angle_relative.cos()); //velocities on collision axis
        let (b1vr_f, b2vr_f) = Self::collision_vs(self.mass, other.mass, b1vr , b2vr);
        let (b1vr_fb, b2vr_fb) = (b1vr_f * self.free_bounce, b2vr_f * other.free_bounce); //apply bounce coefficients
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        







        b1_v_angle_relative, b2_v_angle_relative
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
    pub fn get_ground_bounce(&self) -> f32 {
        self.ground_bounce
    }
    pub fn get_free_bounce(&self) -> f32 {
        self.free_bounce
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

    pub fn new_ball_unchecked(&mut self, x : f32, y : f32, vxi : f32, vyi : f32, r : f32, m : f32, b_g : f32, b_f : f32, color : [f32; 4]) {
        //creates a new ball in the space with given parameters including starting location, velocity,
        //and radius, mass,
        //ground bounce coefficient (applied to absolute value of velocity when hitting ground or wall), 
        //and free bounce coefficient (applied to change in velocity when hitting another ball)
        let mut ret = Ball::default();
        ret.x = x;
        ret.y = y;
        ret.radius = r;
        ret.mass = m;
        ret.color = color;
        ret.ground_bounce = b_g;
        ret.free_bounce = b_f;
        ret.hard_update_unchecked(&self.a, ret.x, ret.y, vxi, vyi, Recalculate::xy(0.0,0.0));
        self.balls.push(ret);
    }

    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
        let mut i = 0;
        for ball in &mut self.balls {
            //keep track of the cached calculus functions
            //check if last acceleration for ball was different and then recompile the cached calculus polynomial if so
            let (x, y) = ((ball.fx.closure)(self.elapsed - ball.x_reftime), (ball.fy.closure)(self.elapsed - ball.y_reftime));
            ball.x = x.unwrap();
            ball.y = y.unwrap();
            if ball.x - ball.radius < self.x1 {
                let vx = ball.get_vx(self.elapsed); let vy = ball.get_vy(self.elapsed); let b = ball.get_bounce();
                if vx < 0.0 {
                    println!("ball {} had left x collision with x velocity {}, which will be reduced to {}", i, vx, -vx * b);
                    ball.hard_update_unchecked(&self.a, ball.get_x(), ball.get_y(), -vx * b, vy, Recalculate::x(self.elapsed));
                }
            }
            if ball.x + ball.radius > self.x2 {
                let vx = ball.get_vx(self.elapsed); let vy = ball.get_vy(self.elapsed); let b = ball.get_bounce();
                if vx > 0.0 {
                    println!("ball {} had right x collision with x velocity {}, which will be reduced to {}", i, vx, -vx * b);
                    ball.hard_update_unchecked(&self.a, ball.get_x(), ball.get_y(), -vx * b, vy, Recalculate::x(self.elapsed));
                }
            }
            if ball.y - ball.radius < self.floor {
                let vx = ball.get_vx(self.elapsed); let vy = ball.get_vy(self.elapsed); let b = ball.get_bounce();
                if vy < 0.0 {
                    println!("ball {} had y collision with y velocity {}, which will be reduced to {}", i, vy, -vy * b);
                    ball.hard_update_unchecked(&self.a, ball.get_x(), ball.get_y(), vx, -vy * b, Recalculate::y(self.elapsed));
                }
            }
            ball.soft_update_unchecked();
            i += 1;
        }
    }

    pub fn get_elapsed(&self) -> f32 {
        self.elapsed
    }
}

#[cfg(test)]
mod tests {
    use crate::kinematics::Monomial;
    use crate::kinematics::Polynomial;
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
        myspace.x1 = -20.0;
        myspace.x2 = 20.0;
        myspace.y1 = -10.0;
        myspace.y2 = 30.0;
        myspace.new_ball_unchecked(0.0, 25.0, 5.0, 5.0, 1.0, 1.0, 0.8, [1.0,0.0,0.0,1.0]);
        myspace.new_ball_unchecked(2.0, 13.0, -5.0, -5.0, 0.5, 0.5, 0.5, [0.0,1.0,0.0,1.0]);
        myspace.new_ball_unchecked(2.0, 16.0, 0.0, 0.0, 3.0, 5.0, 0.9, [0.0,0.0,1.0,1.0]);

        myspace.tick(1.0);
    }
}
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
    pub fn xy(tx : f32, ty : f32) -> Self {
        Self {
            val : (MaybeNew::Update(tx), MaybeNew::Update(ty)),
        }
    }
    pub fn no_update() -> Self {
        Self {
            val : (MaybeNew::NoUpdate, MaybeNew::NoUpdate),
        }
    }
}
pub struct Angle {
    deg : f32,
}

pub struct Vector<T> {
    pub m : T,
    pub θ : Angle,
}

pub enum Value {
    Constant(f32, Units),
    Function(Box<dyn Function>),
}

impl Value {
    pub fn convert(&self, units : Units) -> Option<Value> {
        Some(match self {
            Value::Constant(c, u) => Value::Constant(Units::convert(*c, *u, units)?, units),
            Value::Function(f) => Value::Function(f.stretch_vert(Units::convert(1.0, f.final_units(), units)?).with_final_units_boxed(units)),
        })
    }
}

pub struct ForceDiagram {
    forces : Vec<Vector<Value>>,
    common_units : Units,
}



impl ForceDiagram {
    pub fn add_component(&mut self, component : Vector<Value>) {

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
    pub fn from_deg(deg : f32) -> Angle {
        Angle { deg }
    }
    pub fn from_rad(rad : f32) -> Angle {
        Angle { deg : rad.to_degrees() }
    }
    pub fn xy_h(&self, h : f32) -> (f32, f32) {
        let rad = self.deg.to_radians();
        (rad.cos() * h, rad.sin() * h)
    }
    pub fn get_rad(&self) -> f32 {
        self.deg.to_radians()
    }
    pub fn get_deg(&self) -> f32 {
        self.deg
    }
    pub fn set_deg(&mut self, deg : f32) {
        self.deg = deg;
    }
    pub fn set_rad(&mut self, rad : f32) {
        self.deg = rad.to_degrees();
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
    // fn recurhelper_hard_update_unchecked(&self, a_ref : &AccelxyFunction) -> (Box<dyn Function>, Box<dyn Function>) {
    //     if let AccelxyFunction::ParterFunctionVector(a, θ) = a_ref {
    //         let cached_d_dyn_function = a .integrated(Var::S).unwrap().integrated(Var::S).unwrap();
    //         return (
    //             cached_d_dyn_function.stretch_vert(θ.xy_h(1.).0),
    //             cached_d_dyn_function.stretch_vert(θ.xy_h(1.).1)
    //         );
    //     }
    //     else if let AccelxyFunction::IndependentFunctions(ax, ay) = a_ref {
    //         return (
    //             ax.integrated(Var::X).unwrap().integrated(Var::X).unwrap(),
    //             ay.integrated(Var::Y).unwrap().integrated(Var::Y).unwrap()
    //         );
    //     }
    //     else if let AccelxyFunction::CompositeAcceleration(a1, a2) = a_ref {
    //         let xy1 = self.recurhelper_hard_update_unchecked(&a1);
    //         let xy2 = self.recurhelper_hard_update_unchecked(&a2);
    //         (
    //             Box::new(SumFunction::from_compatible(xy1.0,xy2.0).unwrap()),
    //             Box::new(SumFunction::from_compatible(xy1.1,xy2.1).unwrap()),
    //         )
    //     }
    //     else {
    //         panic!("Unknown AccelxyFunction variant!");
    //     }
    // }
    pub fn hard_update_unchecked(&mut self, a_ref : &AccelxyFunction, xi : f32, yi : f32, vxi : f32, vyi : f32, t : Recalculate) {
        use AccelxyFunction::*;
        match a_ref {
            ParterFunctionVector(a, d) => {
                // todo!();
                // let cached_d_dyn_function = a .integrated(Var::X).unwrap().integrated(Var::Y).unwrap();
                // self.cached_x_dyn_function = Some(cached_d_dyn_function.stretch_vert(d.xy_h(1.0).0));
                // self.cached_y_dyn_function = Some(cached_d_dyn_function.stretch_vert(d.xy_h(1.0).1));
                // self.soft_update_unchecked();
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
                // todo!();
                // //we have two accelerations and need to integrated each parts individually, both xnet and ynet
                // //we calculate x and y from a1 and x and y from a2 recursively then throw them into a self.cached_x_dyn_function and self.cached_y_dyn_function

                // //todo: initial velocities work here
                // let xy1 = self.recurhelper_hard_update_unchecked(a1);
                // let xy2 = self.recurhelper_hard_update_unchecked(a2);
                // self.cached_x_dyn_function = Some(Box::new(SumFunction::from_compatible(xy1.0,xy2.0).unwrap()));
                // self.cached_y_dyn_function = Some(Box::new(SumFunction::from_compatible(xy1.1,xy2.1).unwrap()));
                // self.soft_update_unchecked();
            }
        }
    }
    pub fn hard_update(&mut self, a_ref : &AccelxyFunction, xi : f32, yi : f32, vxi : f32, vyi : f32, t : Recalculate) {
        //todo: make this checked, finish hard_update_unchecked, and start to prefer the checked versions
        self.hard_update_unchecked(a_ref, xi, yi, vxi, vyi, t);
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

    pub fn debug_velocities(&self) {
        for i in 0..self.balls.len() {
            println!("Ball {} has velocity ({}, {})", i, self.balls[i].get_vx(self.elapsed), self.balls[i].get_vy(self.elapsed));
        }
    }

    pub fn debug_positions(&self) {
        for i in 0..self.balls.len() {
            println!("Ball {} has position ({}, {})", i, self.balls[i].x, self.balls[i].y);
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

    fn two_mut_vals_in_container<T>(container : &mut Vec<T>, mut i1 : usize, mut i2 : usize) -> (&mut T, &mut T) {
        if i1 > i2 {
            (i1,i2) = (i2,i1);
        }
        if i1 == i2 {
            panic!("be the same");
        }
        let first_split = container.split_at_mut(i1 + 1);
        let second_split = first_split.1.split_at_mut(i2 - i1 - 1);
        // first_split.0[0] = v1;
        // second_split.1[0] = v2;
        (&mut first_split.0[0], &mut second_split.1[0])
    }
    fn collision_vs(m1 : f32, m2 : f32, v1 : f32, v2 : f32) -> (f32,f32) {
        //calculates exit velocities of two objects colliding in one dimension
        (2.0*m2*v2/(m1+m2) + (m1-m2)/(m1+m2)*v1, 2.0*m1*v1/(m1+m2) - (m1-m2)/(m1+m2)*v2)
    }
    pub fn exert_collision(&mut self, i : usize, j : usize) {
        //take two round balls and bounce them from each other, assuming they are touching
        //two balls far apart will collide as if one of them is big enough to be touching the other; their size isn't known in this function
        //if the balls aren't headed towards each other, they won't collide
        
        let (mut b1, mut b2) = Self::two_mut_vals_in_container::<Ball>(&mut self.balls, i, j);
        
        //fetch initial velocities
        let (b1vx, b1vy, b2vx, b2vy) = (b1.get_vx(self.elapsed), b1.get_vy(self.elapsed), b2.get_vx(self.elapsed), b2.get_vy(self.elapsed)); 
        let (b1v, b2v) = (b1vx.hypot(b1vy), b2vx.hypot(b2vy)); //pythagorean theorem, v magnitude

        //calculate velocity θs
        let collision_θ = (b2.y-b1.y).atan2(b2.x-b1.x); //gets direction of centers from b1 to b2
        let (b1v_θ   , b2v_θ   ) = (b1vy.atan2(b1vx), b2vy.atan2(b2vx));
        let (b1v_θ_ll, b2v_θ_ll) = (b1v_θ - collision_θ, b2v_θ - collision_θ); //ll represents parellel to collision axis
        
        //find velocities along collision axis
        let (b1vll, b2vll) = (b1v * b1v_θ_ll.cos(), b2v * b2v_θ_ll.cos()); //velocities on collision axis, ll represents parellel
        let (b1vL , b2vL ) = (b1v * b1v_θ_ll.sin(), b2v * b2v_θ_ll.sin()); //velocities off collision axis, L represents perpendicular

        //abort collision if the balls aren't headed towards each other
        let avll = b1vll;
        let bvll = b2vll;
        if !(bvll.abs() > avll.abs() && (bvll < 0.0 || avll > 0.0) || (bvll < 0.0 && avll > 0.0)) {
            return; //no collision, they aren't exerting force;
        }

        //collide balls
        let (b1vll_f , b2vll_f ) = Self::collision_vs(b1.mass, b2.mass, b1vll , b2vll); //f means final
        let (b1vll_fb, b2vll_fb) = (b1vll_f * b1.free_bounce, b2vll_f * b2.free_bounce); //apply bounce coefficients, b means bounce
        
        //calculate new total velocities and their θs
        let (b1v_fb_θ_ll, b2v_fb_θ_ll) = (b1vL.atan2(b1vll_fb), b2vL.atan2(b2vll_fb));
        let (b1v_fb_θ   , b2v_fb_θ   ) = (b1v_fb_θ_ll + collision_θ, b2v_fb_θ_ll + collision_θ);
        let (b1v_fb, b2v_fb) = (b1vll_fb.hypot(b1vL), b2vll_fb.hypot(b2vL));



        //calculate x and y components and put back into ball
        let (b1vx_fb, b1vy_fb, b2vx_fb, b2vy_fb) = (b1v_fb * b1v_fb_θ.cos(), b1v_fb * b1v_fb_θ.sin(), b2v_fb * b2v_fb_θ.cos(), b2v_fb * b2v_fb_θ.sin());
        b1.hard_update(&self.a, b1.x, b1.y, b1vx_fb, b1vy_fb, Recalculate::xy(self.elapsed, self.elapsed));
        b2.hard_update(&self.a, b2.x, b2.y, b2vx_fb, b2vy_fb, Recalculate::xy(self.elapsed, self.elapsed));
    }

    pub fn search_collision_pairs(&self) -> Vec<(usize, usize)> {
        //O(n^2)) function searching for colliding balls with pythagorean theorem and two for loops
        let mut ret = Vec::<(usize, usize)>::new();
        for i in 0..(self.balls.len()-1) {
            for j in (i+1)..self.balls.len() {
                //if (self.balls[i].x - self.balls[j].x) * (self.balls[i].x - self.balls[j].x) + (self.balls[i].y - self.balls[j].y) * (self.balls[i].y - self.balls[j].y) <= self.balls[i].radius + self.balls[j].radius {
                if (self.balls[i].x - self.balls[j].x).hypot(self.balls[i].y - self.balls[j].y) <= self.balls[i].radius + self.balls[j].radius {
                    ret.push((i,j));
                }
            }
        }
        ret
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
                let vx = ball.get_vx(self.elapsed); let vy = ball.get_vy(self.elapsed); let b = ball.get_ground_bounce();
                if vx < 0.0 {
                    println!("ball {} had left x collision with x velocity {}, which will be reduced to {}", i, vx, -vx * b);
                    ball.hard_update_unchecked(&self.a, ball.get_x(), ball.get_y(), -vx * b, vy, Recalculate::x(self.elapsed));
                }
            }
            if ball.x + ball.radius > self.x2 {
                let vx = ball.get_vx(self.elapsed); let vy = ball.get_vy(self.elapsed); let b = ball.get_ground_bounce();
                if vx > 0.0 {
                    println!("ball {} had right x collision with x velocity {}, which will be reduced to {}", i, vx, -vx * b);
                    ball.hard_update_unchecked(&self.a, ball.get_x(), ball.get_y(), -vx * b, vy, Recalculate::x(self.elapsed));
                }
            }
            if ball.y - ball.radius < self.floor {
                let vx = ball.get_vx(self.elapsed); let vy = ball.get_vy(self.elapsed); let b = ball.get_ground_bounce();
                if vy < 0.0 {
                    println!("ball {} had y collision with y velocity {}, which will be reduced to {}", i, vy, -vy * b);
                    ball.hard_update_unchecked(&self.a, ball.get_x(), ball.get_y(), vx, -vy * b, Recalculate::y(self.elapsed));
                }
            }
            ball.soft_update_unchecked();
            i += 1;
        }
        for pair in self.search_collision_pairs() {
            println!("ball {} had collisions with ball {}", pair.0, pair.1);
            self.exert_collision(pair.0, pair.1);
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
    fn gravity_space_collision() {
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
        // myspace.new_ball_unchecked(0.0, 25.0, 5.0, 5.0, 1.0, 1.0, 0.8, 0.99, [1.0,0.0,0.0,1.0]);
        // myspace.new_ball_unchecked(2.0, 13.0, -5.0, -5.0, 0.5, 0.5, 0.5, 0.99,[0.0,1.0,0.0,1.0]);
        // myspace.new_ball_unchecked(2.0, 16.0, 0.0, 0.0, 3.0, 5.0, 0.9, 0.99, [0.0,0.0,1.0,1.0]);

        // myspace.tick(1.0);

        dbg!(Space::collision_vs(1.0, 1.5, -1.0, 1.0));

        myspace.new_ball_unchecked(10.0, 10.0,  1.0,  0.0, 1.0, 1.0, 1.0, 1.0, [1.0,1.0,1.0,1.0]);
        myspace.new_ball_unchecked(12.0, 10.0, -1.0, 0.0, 1.0, 1.0, 1.0, 1.0, [1.0,1.0,1.0,1.0]);
        myspace.debug_velocities();
        myspace.debug_positions();
        myspace.exert_collision(0, 1);
        myspace.debug_velocities();
        myspace.debug_positions();

        // myspace.new_ball_unchecked(7.828, 7.828, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, [1.0,1.0,1.0,1.0]);
        // myspace.new_ball_unchecked(5.0, 5.0, 1.0, 1.0, 1.0, 1.5, 1.0, 1.0, [1.0,1.0,1.0,1.0]);
        // myspace.collide(2, 3);
    }
}
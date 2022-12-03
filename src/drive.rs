use super::kinematics;
use kinematics::Function;
use kinematics::SumFunction;
use kinematics::Polynomial;
use kinematics::Unit;
use kinematics::Units;
use kinematics::Var;
use kinematics::EvalFunctionError;
use kinematics::Monomial;
use super::physics;
use physics::Ball;
use physics::Angle;
use physics::Space;
use physics::AccelxyFunction;
use physics::GRAVITY_MPS2;
use super::gl::Window;

struct World {
    space : Space,
    window : Window,
}

pub fn bind(mut window : Window, mut space : Space, fps : u64, tps : u64) -> Window {
    let xscale : f64 = window.width as f64 / (space.x2 as f64 - space.x1 as f64);
    let xcenter : f64 = -space.x1 as f64;
    let yscale : f64 = window.height as f64 / (space.y1 as f64 - space.y2 as f64);
    let ycenter : f64 = -space.y2 as f64;
    window.set_render_fn(Box::new(move |rtick, utick, c, gl| {
        use graphics;
        let floor_p_y = window.height as f64 + (space.floor as f64 - space.y1 as f64) * yscale;
        space.tick(1.0 / tps as f32);
        graphics::clear([0.5, 0.75, 0.85, 1.0], gl); //sky blue
        graphics::rectangle([0.6,0.4,0.2,1.0],   [0.0, floor_p_y,       window.width as f64, window.height as f64 - floor_p_y], c.transform, gl);
        graphics::rectangle([0.25,0.55,0.2,1.0], [0.0, floor_p_y - 5.0, window.width as f64, 10.0     ], c.transform, gl);
        for ball in &space.balls {
            graphics::ellipse(
                ball.get_color(),
                graphics::ellipse::circle(ball.get_x() as f64 * xscale + xcenter * xscale, ball.get_y() as f64 * yscale + ycenter * yscale, ball.get_radius() as f64 * yscale),
                c.transform,
                gl,
            );
        }
    }));
    // window.set_update_fn(Box::new(|utick| {
    //     ret.space.tick(1.0 / 60.0);
    // }));
    window.set_ups(fps);
    window.set_fps(fps);
    window
}

pub fn world1() {
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
    myspace.y1 = -20.0;
    myspace.y2 = 30.0;
    myspace.new_ball_unchecked(0.0, 25.0, 2.0, 2.0, 1.0, 1.0, 0.8, [1.0, 0.0, 0.0, 1.0]);
    myspace.new_ball_unchecked(2.0, 13.0, 0.0, 0.0, 0.5, 0.5, 0.8, [0.0, 1.0, 0.0, 1.0]);
    myspace.new_ball_unchecked(4.0, 16.0, 10.0, -5.0, 3.0, 5.0, 0.8, [0.0, 0.0, 1.0, 1.0]);
    let mut mywindow = Window::new(640,480);
    let mut boundwindow = bind(mywindow, myspace, 165, 165);
    boundwindow.begin(String::from("Beautiful balls"), Window::DEFAULT_FLAGS);
    loop {
        boundwindow.maintain();
    }
}
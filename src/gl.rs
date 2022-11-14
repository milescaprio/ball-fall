use bufro::Color;
use cgmath::VectorSpace;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::time::Instant;
use rand::RngCore;

pub struct Window {
    width : usize,
    height : usize,

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
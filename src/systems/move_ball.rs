extern crate amethyst;
use crate::pong::Ball;

// use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (ball, local) in (&balls, &mut locals).join() {
            local.translate_x(ball.velocity[0] * time.delta_seconds());
            local.translate_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}

// fn main() {}

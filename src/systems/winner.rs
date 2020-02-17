use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, World, WriteStorage},
};

use crate::pong::{Ball, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, mut locals): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Right player scored on the left side
                println!("Player 2 scores!");
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // left player scored on the right side
                println!("Player 1 scores!");
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0]; // Reverse direction
                transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position
            }
        }
    }
}
use amethyst::core::{Transform, SystemDesc, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use crate::components::Character;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Character>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, characters, time, input): Self::SystemData) {
        for (_character, transform) in (&characters, &mut transforms).join() {
            if let Some(horizontal) = input.axis_value("horizontal") {
                transform.prepend_translation_x(
                    time.delta_seconds() *  horizontal,
                );
            }
            if let Some(vertical) = input.axis_value("vertical") {
                transform.prepend_translation_y(
                    time.delta_seconds() *  vertical,
                );
            }
        }
    }
}

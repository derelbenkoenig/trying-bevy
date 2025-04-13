use crate::prelude::*;
use bevy_utils::*;

const INPUT_UP:    u8 = 1 << 0;
const INPUT_DOWN:  u8 = 1 << 1;
const INPUT_LEFT:  u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

const INPUT_BUTTON1: u8 = 1 << 4;
const INPUT_BUTTON2: u8 = 1 << 5;
const INPUT_BUTTON3: u8 = 1 << 6;
const INPUT_BUTTON4: u8 = 1 << 7;

#[derive(Component)]
pub struct Player {
    pub handle: usize
}

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = 0u8;

        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::Space]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::KeyJ]) {
            input |= INPUT_BUTTON1;
        }
        if keys.any_pressed([KeyCode::KeyI]) {
            input |= INPUT_BUTTON2;
        }
        if keys.any_pressed([KeyCode::KeyO]) {
            input |= INPUT_BUTTON3;
        }
        if keys.any_pressed([KeyCode::Semicolon]) {
            input |= INPUT_BUTTON4;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}

pub fn apply_inputs(
    inputs: Res<PlayerInputs<Config>>,
    mut fighters: Query<(&mut Transform, &Player), With<Fighter>>,
    time: Res<Time>
) {
    for (mut transform, fighter) in &mut fighters {
        let (input, _) = inputs[fighter.handle];
        let mut direction = Vec2::ZERO;

        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }

        let walk_speed = 0.8;
        transform.translation += (walk_speed * direction * time.delta_secs()).extend(0.);
    }
}

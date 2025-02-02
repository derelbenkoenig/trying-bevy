// mod components;

use bevy_matchbox::{
    prelude::*,
    matchbox_socket::{WebRtcSocket, PeerId}
};
use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    tasks::IoTaskPool,
    utils::HashMap
};
use bevy_ggrs::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window{
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<Config>::default()
        ))
        .rollback_component_with_clone::<Transform>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(Startup, (setup, spawn_player, start_matchbox_socket))
        .add_systems(Update, wait_for_players)
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.
            },
            ..OrthographicProjection::default_2d()
        }
    ));
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player,
            Sprite {
                color: Color::srgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            }
        ))
        .add_rollback();
}

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>
) {
    let mut local_inputs = HashMap::new();
    for handle in &local_players.0 {
        let mut input = 0;
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            input |= INPUT_UP;
        }

        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_DOWN;
        }

        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT;
        }

        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }

        if keys.any_pressed([KeyCode::Space, KeyCode::Enter]) {
            input |= INPUT_FIRE;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));

}

fn move_player(
    mut players: Query<&mut Transform, With<Player>>,
    inputs: Res<PlayerInputs<Config>>,
) {
    let mut direction = Vec2::ZERO;

    let (input, _) = inputs[0];

    if input & INPUT_UP != 0 {
        direction.y += 1.;
    }

    if input & INPUT_DOWN != 0 {
        direction.y -= 1.;
    }

    if input & INPUT_LEFT != 0 {
        direction.x -= 1.;
    }

    if input & INPUT_RIGHT != 0 {
        direction.x += 1.;
    }

    let move_speed = 0.13;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in &mut players {
        transform.translation += move_delta;
    }
}

#[derive(Component)]
struct Player{
    handle: usize
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://localhost:3536/bevy_rollback?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket>) {
    if socket.get_channel(0).is_err() {
        return;
    }

    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    info!("all players have joined");

    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let channel = socket.take_channel(0).unwrap();

    let ggrs_sess = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_sess));
}

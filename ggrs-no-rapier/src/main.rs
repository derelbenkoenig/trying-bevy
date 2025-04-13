mod player_movement;
mod fighter;
mod physics;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_matchbox::prelude::*;
    pub use bevy_ggrs::*;

    pub use crate::player_movement::*;
    pub use crate::fighter::*;
    pub use crate::physics::*;

    pub type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;
}

use crate::prelude::*;
use bevy::render::camera::ScalingMode;

pub const MATCHBOX_ADDR: &str = "ws://localhost:3536/trying-bevy-ggrs-no-rapier?next=2";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<Config>::default(),
            PhysicsPlugin{}
        ))
        .rollback_component_with_clone::<Transform>()
        .insert_resource(ClearColor(Color::srgb(0.8, 0.9, 0.9)))
        .add_systems(Startup, (spawn_camera, spawn_players, start_matchbox_socket))
        .add_systems(Update, wait_for_players)
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, apply_inputs)
        .run();
}

fn start_matchbox_socket(mut commands: Commands) {
    commands.insert_resource(MatchboxSocket::new_unreliable(MATCHBOX_ADDR));
}


fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>
) {
    
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
}

fn spawn_players(mut commands: Commands) {
    commands
        .spawn((
            Player{ handle: 0 },
            Fighter,
            PhysicsBundle::default(),
            Transform::from_translation(Vec3::new(-2., 0., 0.)),
            Sprite{
                color: Color::srgb(1., 0., 0.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            }
        ))
        .add_rollback();

    commands
        .spawn((
            Player{ handle: 1 },
            Fighter,
            PhysicsBundle::default(),
            Transform::from_translation(Vec3::new(2., 0., 0.)),
            Sprite{
                color: Color::srgb(0., 1., 0.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            }
        ))
        .add_rollback();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}

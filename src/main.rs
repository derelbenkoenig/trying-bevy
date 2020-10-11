mod components;

use bevy::prelude::*;
use crate::components::Character;

fn character_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Character, &mut Transform)>,
) {
    for (character, mut transform) in &mut query.iter() {
        let translation = transform.translation_mut();
        let move_speed = 5.0;
        if keyboard_input.pressed(KeyCode::D) {
            *translation.x_mut() += time.delta_seconds * move_speed;
        }
        if keyboard_input.pressed(KeyCode::A) {
            *translation.x_mut() -= time.delta_seconds * move_speed;
        }
        if keyboard_input.pressed(KeyCode::W) {
            *translation.y_mut() += time.delta_seconds * move_speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            *translation.y_mut() -= time.delta_seconds * move_speed;
        }
    }
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(character_movement_system.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // The ground
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            // transform: Transform::from_non_uniform_scale(Vec3::new(2.0, 1.0, 1.0 / 10.0)),
            ..Default::default()
        })

        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(-4.0, 8.0, 8.0)),
            ..Default::default()
        })

        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(0.0, 4.0, 12.0),
                Vec3::new(0.0, 3.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });

    let character_model = PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            subdivisions: 4,
            radius: 0.5,
        })),
        material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
        transform: Transform::from_translation(Vec3::new(1.5, 1.5, 1.5)),
        ..Default::default()
    };

    commands.spawn(character_model)
        .with(Character::new());
}
mod systems;
mod components;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst::assets::{PrefabLoader, RonFormat, PrefabLoaderSystemDesc};
use amethyst::utils::scene::BasicScenePrefab;
use amethyst::renderer::rendy::mesh::{Position, Normal, TexCoord};
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use crate::components::Character;
use crate::systems::MovementSystem;


type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Loading...");
        let character = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/square_character.ron", RonFormat, ())
        });
        let stage = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/stage.ron", RonFormat, ())
        });
        data.world.register::<Character>();
        data.world.create_entity().with(character).with(Character::new()).build();
        data.world.create_entity().with(stage).build();
        println!("Ready!");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("End!");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?
        .with(MovementSystem, "movement_system", &["input_system"]);

    let mut game = Application::new(assets_dir, GameState, game_data)?;

    game.run();

    Ok(())
}

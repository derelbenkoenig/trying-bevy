use amethyst::{prelude::*, utils::application_root_dir};

struct GameState;

impl EmptyState for GameState {
    fn on_start(&mut self, _: StateData<'_, ()>) {
        println!("Begin!");
    }

    fn on_stop(&mut self, _: StateData<'_, ()>) {
        println!("End!");
    }

    fn update(&mut self, _: StateData<'_, ()>) -> EmptyTrans {
        println!("Hello from Amethyst!");
        Trans::Quit
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let assets_dir = application_root_dir()?.join("assets");
    let mut game = Application::new(assets_dir, GameState, ())?;
    game.run();

    Ok(())
}
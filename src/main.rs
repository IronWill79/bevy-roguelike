use bevy::{prelude::*, window::PresentMode};

mod components;
mod startup;
use startup::setup;

const SCREEN_WIDTH: f32 = 768.0;
const SCREEN_HEIGHT: f32 = 400.0;
const TITLE: &str = "Bevy Roguelike";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: TITLE.to_string(),
                    width: SCREEN_WIDTH,
                    height: SCREEN_HEIGHT,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                },
                ..default()
            }
        ))
        .add_startup_system(setup)
        // .add_system(animate_sprite)
        // .add_system(animate_direction)
        .run();
}

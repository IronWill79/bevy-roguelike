use bevy::prelude::*;
use crate::components::common::*;

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Position { x: 40, y: 25 },
        Renderable {
            glyph: '@',
            fg: Color::YELLOW,
            bg: Color::BLACK,
        }
    ));
    for i in 0..10 {
        commands.spawn((
            Position { x: i * 7, y: 20 },
            Renderable {
                glyph: '@',
                fg: Color::RED,
                bg: Color::BLACK,
            }
        ));
    }
}

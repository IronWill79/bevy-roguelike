use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: i8,
    pub y: i8
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color
}

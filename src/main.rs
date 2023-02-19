use bevy::{prelude::*, window::PresentMode};

const SCREEN_WIDTH: f32 = 768.0;
const SCREEN_HEIGHT: f32 = 400.0;
const SCREEN_GRID_WIDTH: f32 = 80.0;
const SCREEN_GRID_HEIGHT: f32 = 50.0;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Component, Copy, Clone)]
enum Direction {
    Down = 0,
    Left = 1,
    Right = 2,
    Up = 3
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct FacingAnimationTimer(Timer);

fn animate_direction(
    time: Res<Time>,
    mut query: Query<(
        &mut FacingAnimationTimer,
        &mut Direction
    )>
) {
    for (mut timer,  mut facing) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let direction = facing.as_ref();
            match direction {
                Direction::Down => *facing = Direction::Left,
                Direction::Left => *facing = Direction::Up,
                Direction::Right => *facing = Direction::Down,
                Direction::Up => *facing = Direction::Right,
            }
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Transform,
        &mut Position,
        &Direction
    )>
) {
    for (mut timer, mut sprite, mut transform, mut position, facing) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % 3 + (3 * *facing as usize);
            match facing {
                Direction::Down => {
                    position.as_mut().y -= SCREEN_HEIGHT / SCREEN_GRID_HEIGHT;
                },
                Direction::Left => {
                    position.as_mut().x -= SCREEN_WIDTH / SCREEN_GRID_WIDTH;
                },
                Direction::Right => {
                    position.as_mut().x += SCREEN_WIDTH / SCREEN_GRID_WIDTH;
                },
                Direction::Up => {
                    position.as_mut().y += SCREEN_HEIGHT / SCREEN_GRID_HEIGHT;
                },
            }
            *transform = Transform::from_xyz(position.x, position.y, 0.0);
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(72.0, 72.0), 3, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Direction::Down,
        FacingAnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        Position{ x: 39.0, y: 24.0 }
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Bevy Roguelike".to_string(),
                    width: 768.,
                    height: 480.,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                },
                ..default()
            }
        ))
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .add_system(animate_direction)
        .run();
}

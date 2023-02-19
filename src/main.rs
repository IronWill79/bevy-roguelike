use bevy::{prelude::*, window::PresentMode};

const SCREEN_WIDTH: f32 = 768.0;
const SCREEN_HEIGHT: f32 = 400.0;
const SCREEN_GRID_WIDTH: f32 = 80.0;
const SCREEN_GRID_HEIGHT: f32 = 50.0;

const SPRITE_ANIMATION_COUNT: usize = 3;
const SPRITE_ANIMATION_DELAY: f32 = 0.1;
const SPRITE_DEFAULT_X: f32 = 39.0;
const SPRITE_DEFAULT_Y: f32 = 24.0;
const SPRITE_DIRECTION_DELAY: f32 = 1.5;
const SPRITE_HEIGHT: f32 = 72.0;
const SPRITE_WIDTH: f32 = 72.0;

const TITLE: &str = "Bevy Roguelike";

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    animation_timer: AnimationTimer,
    direction: Direction,
    direction_timer: DirectionAnimationTimer,
    position: Position,
    _p: Player,

    #[bundle]
    sprite: SpriteSheetBundle,
}

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
struct DirectionAnimationTimer(Timer);

fn animate_direction(
    time: Res<Time>,
    mut query: Query<(
        &mut DirectionAnimationTimer,
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
            sprite.index = (sprite.index + 1) % SPRITE_ANIMATION_COUNT + (SPRITE_ANIMATION_COUNT * *facing as usize);
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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT), 3, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        PlayerBundle {
            animation_timer: AnimationTimer(Timer::from_seconds(SPRITE_ANIMATION_DELAY, TimerMode::Repeating)),
            direction: Direction::Down,
            direction_timer: DirectionAnimationTimer(Timer::from_seconds(SPRITE_DIRECTION_DELAY, TimerMode::Repeating)),
            position: Position { x: SPRITE_DEFAULT_X, y: SPRITE_DEFAULT_Y },
            _p: Player,
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(SPRITE_DEFAULT_X, SPRITE_DEFAULT_Y, 0.0),
                ..default()
            },
        }
    );
}

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
        .add_system(animate_sprite)
        .add_system(animate_direction)
        .run();
}

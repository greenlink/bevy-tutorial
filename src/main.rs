use bevy::{prelude::*, winit::WinitSettings, input::keyboard::*};

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Knight;

enum KnightState {
    RunLeft,
    RunRight,
    IdleLeft,
    IdleRight
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("../assets/chars/knight/Run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 64.0), 2, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        Knight,
    ));
}

fn animate_sprite(time: Res<Time>, 
    texture_atlases: Res<Assets<TextureAtlas>>, 
    mut query: Query<(
    &mut AnimationTimer, 
    &mut TextureAtlasSprite, 
    &Handle<TextureAtlas>)>) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn move_knight (
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>
) {  
    for (mut timer, mut sprite, mut texture_atlas_handle) in &mut query {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

        if keyboard_input.pressed(KeyCode::Right) {
            timer.tick(time.delta());
            if timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        } 
    }     
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(move_knight)
        .run();
}


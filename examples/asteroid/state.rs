use super::arena::*;
use bevy::prelude::*;
/// Component to tag an entity as only needed in one state
pub struct ForState<T> {
    pub states: Vec<T>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
}
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppGameState {
    /// Invalid used when AppState is NOT Game
    Invalid,
    Game,
    Pause,
    GameOver,
}

#[derive(Debug)]
pub struct RunState {
    pub player: Option<Entity>,
    pub arena: Option<Arena>,
    pub score: Option<u32>,
    // Store the most used asset handles
    pub font_handle: Handle<Font>,
    pub laser_texture_handle: Handle<ColorMaterial>,
    pub laser_audio_handle: Handle<AudioSource>,
    pub meteor_big_handle: Handle<ColorMaterial>,
    pub meteor_med_handle: Handle<ColorMaterial>,
    pub meteor_small_handle: Handle<ColorMaterial>,
}

impl RunState {
    pub fn new(
        asset_server: &AssetServer,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> RunState {
        RunState {
            player: None,
            arena: None,
            score: None,
            font_handle: asset_server.load("asteroid/kenvector_future.ttf"),
            laser_texture_handle: materials.add(asset_server.load("asteroid/laserRed07.png").into()),
            laser_audio_handle: asset_server.load("asteroid/sfx_laser1.mp3"),
            meteor_big_handle: materials.add(asset_server.load("asteroid/meteorBrown_big1.png").into()),
            meteor_med_handle: materials.add(asset_server.load("asteroid/meteorBrown_med1.png").into()),
            meteor_small_handle: materials.add(asset_server.load("asteroid/meteorBrown_small1.png").into()),
        }
    }
}

pub fn appstate_enter_despawn(
    mut commands: Commands,
    state: Res<State<AppState>>,
    query: Query<(Entity, &ForState<AppState>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(&state.current()) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn appgamestate_enter_despawn(
    mut commands: Commands,
    state: ResMut<State<AppGameState>>,
    query: Query<(Entity, &ForState<AppGameState>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(&state.current()) {
            commands.entity(entity).despawn();
        }
    }
}

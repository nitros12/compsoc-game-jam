use bevy::prelude::*;

use crate::button;
use crate::gamestate::{GameState, GameStage};
use crate::shop_scene;

pub struct PopUpsPlugin;

impl Plugin for PopUpsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(GameStage::Main, GameState::Main, setup.system())
            .on_state_exit(GameStage::Main, GameState::Main, teardown.system())
            .add_system(handle_cauldron_click.system())
            .add_system(handle_jam_book_click.system())
            //.add_system(generate_story.system())
    ;
    }
}

struct JamBook;
struct Cauldron;

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let jam_book_handle = asset_server.load("sprites/jambook.png");
    let cauldron_handle = asset_server.load("sprites/cauldron.png");

    commands
        .spawn(ButtonBundle {
            material: materials.add(jam_book_handle.into()),
            style: Style {
                size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(button::ButtonState::default())
        .with(JamBook)
        .spawn(ButtonBundle {
            material: materials.add(cauldron_handle.into()),
            style: Style {
                size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(button::ButtonState::default())
        .with(Cauldron);
}

fn teardown(
    commands: &mut Commands,
    q_jambook: Query<Entity, With<JamBook>>,
    q_cauldron: Query<Entity, With<Cauldron>>,
) {
    for entity in q_jambook.iter() {
        commands.despawn(entity);
    }

    for entity in q_cauldron.iter() {
        commands.despawn(entity);
    }
}

fn handle_cauldron_click(
    mut state: ResMut<State<GameState>>,
    q_cauldron: Query<&Cauldron>,
    mut event_reader: EventReader<button::ButtonPressedEvent>,
) {
    for button::ButtonPressedEvent(entity) in event_reader.iter() {
        if let Ok(Cauldron) = q_cauldron.get_component(*entity) {
            state.set_next(GameState::Cauldron).unwrap();
        }
    }
}

fn handle_jam_book_click(
    commands: &mut Commands,
    q_jambook: Query<&JamBook>,
    mut event_reader: EventReader<button::ButtonPressedEvent>,
) {
    for button::ButtonPressedEvent(entity) in event_reader.iter() {
        if let Ok(JamBook) = q_jambook.get_component(*entity) {
            // clicked
        }
    }
}

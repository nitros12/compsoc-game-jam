use bevy::prelude::*;

use crate::button;
use crate::gamestate::{GameStage, GameState};
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

struct JamBookButton;
struct CauldronButton;

struct JamBook;

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
        .with(JamBookButton)
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
        .with(CauldronButton);
}

fn teardown(
    commands: &mut Commands,
    q_jambook_button: Query<Entity, With<JamBookButton>>,
    q_cauldron_button: Query<Entity, With<CauldronButton>>,
    q_jambook: Query<Entity, With<JamBook>>,
) {
    for entity in q_jambook_button.iter() {
        commands.despawn(entity);
    }

    for entity in q_jambook.iter() {
        commands.despawn(entity);
    }

    for entity in q_cauldron_button.iter() {
        commands.despawn(entity);
    }
}

fn spawn_jam_book(commands: &mut Commands, materials: &mut Assets<ColorMaterial>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(60.0)),
                justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(10.0),
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::YELLOW_GREEN.into()),
            ..Default::default()
        })
        .with(JamBook);
}

fn handle_cauldron_click(
    mut state: ResMut<State<GameState>>,
    q_cauldron_button: Query<&CauldronButton>,
    mut event_reader: EventReader<button::ButtonPressedEvent>,
) {
    for button::ButtonPressedEvent(entity) in event_reader.iter() {
        if let Ok(CauldronButton) = q_cauldron_button.get_component(*entity) {
            state.set_next(GameState::Cauldron).unwrap();
        }
    }
}

fn handle_jam_book_click(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_jambook_button: Query<&JamBookButton>,
    q_jambook: Query<Entity, With<JamBook>>,
    mut event_reader: EventReader<button::ButtonPressedEvent>,
) {
    for button::ButtonPressedEvent(entity) in event_reader.iter() {
        if let Some(entity) = q_jambook.iter().next() {
            commands.despawn_recursive(entity);
            return;
        }

        if let Ok(JamBookButton) = q_jambook_button.get_component(*entity) {
            spawn_jam_book(commands, &mut *materials)
        }
    }
}

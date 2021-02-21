use bevy::prelude::*;

use crate::jam::JamIngredient;
use crate::shop_scene;
use crate::{button, jam::JamAssets};
use crate::{
    gamestate::{GameStage, GameState},
    jam::JamEffect,
};

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

fn spawn_jam_book(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    asset_server: &AssetServer,
    jam_assets: &JamAssets,
) {
    let recipe_book_handle = asset_server.load("sprites/recipebook.png");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with(JamBook)
        .with_children(|parent| {
            // left side
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(400.0 * 1.5), Val::Px(300.0 * 1.5)),
                        ..Default::default()
                    },
                    material: materials.add(recipe_book_handle.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                padding: Rect::all(Val::Percent(5.0)),
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            material: materials.add(Color::NONE.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            for ingredient in JamIngredient::all() {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(
                                                Val::Percent(100.0),
                                                Val::Percent(100.0),
                                            ),
                                            padding: Rect::all(Val::Percent(2.0)),
                                            flex_direction: FlexDirection::Row,
                                            ..Default::default()
                                        },
                                        material: materials.add(Color::NONE.into()),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                                                ..Default::default()
                                            },
                                            material: materials
                                                .add(ingredient.asset_for(jam_assets).into()),
                                            ..Default::default()
                                        });

                                        parent.spawn(TextBundle {
                                            style: Style {
                                                margin: Rect::all(Val::Px(2.0)),
                                                ..Default::default()
                                            },
                                            text: Text::with_section(
                                                ingredient.name(),
                                                TextStyle {
                                                    font: font.clone(),
                                                    font_size: 15.0,
                                                    color: Color::BLACK,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });

                                        for effect in ingredient.effects() {
                                            parent.spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                                                    ..Default::default()
                                                },
                                                material: materials
                                                    .add(effect.asset_for(jam_assets).into()),
                                                ..Default::default()
                                            });
                                        }
                                    });
                            }
                        });

                    // right side
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                padding: Rect::all(Val::Percent(5.0)),
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            material: materials.add(Color::NONE.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            for effect in JamEffect::all() {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(
                                                Val::Percent(100.0),
                                                Val::Percent(100.0),
                                            ),
                                            ..Default::default()
                                        },
                                        material: materials.add(Color::NONE.into()),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        // icon
                                        parent.spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                                                ..Default::default()
                                            },
                                            material: materials
                                                .add(effect.asset_for(jam_assets).into()),
                                            ..Default::default()
                                        });

                                        // name
                                        // description
                                        parent.spawn(TextBundle {
                                            style: Style {
                                                max_size: Size::new(Val::Px(220.0), Val::Undefined),
                                                ..Default::default()
                                            },
                                            text: Text {
                                                sections: vec![
                                                    TextSection {
                                                        value: effect.name().to_string(),
                                                        style: TextStyle {
                                                            font: font.clone(),
                                                            font_size: 14.0,
                                                            color: Color::BLACK,
                                                        },
                                                    },
                                                    TextSection {
                                                        value: format!(
                                                            "\n{}",
                                                            effect.description()
                                                        ),
                                                        style: TextStyle {
                                                            font: font.clone(),
                                                            font_size: 11.0,
                                                            color: Color::BLACK,
                                                        },
                                                    },
                                                ],
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        });
                                    });
                            }
                        });
                });
        });
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
    asset_server: Res<AssetServer>,
    jam_assets: Res<JamAssets>,
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
            spawn_jam_book(commands, &mut *materials, &*asset_server, &*jam_assets);
        }
    }
}

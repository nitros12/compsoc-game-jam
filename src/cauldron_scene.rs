use crate::button;
use crate::dragging::{DropTarget, DroppedOntoEvent};
use crate::gamestate::{GameStage, GameState};
use crate::jam::JamIngredient;
use bevy::prelude::*;

pub struct CauldronScenePlugin;

struct Background;
struct Cauldron;
struct CauldronContent;
struct Return;

impl Plugin for CauldronScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(GameStage::Main, GameState::Cauldron, setup.system())
            .on_state_update(
                GameStage::Main,
                GameState::Cauldron,
                handle_return_click.system(),
            )
            .on_state_update(
                GameStage::Main,
                GameState::Cauldron,
                recolour_cauldron.system(),
            )
            .on_state_update(
                GameStage::Main,
                GameState::Cauldron,
                handle_content_drop.system(),
            )
            .on_state_exit(GameStage::Main, GameState::Cauldron, teardown.system())
            .insert_resource(CauldronContents(vec![]));
    }
}

fn teardown(commands: &mut Commands, q_background: Query<Entity, With<Background>>) {
    for entity in q_background.iter() {
        commands.despawn(entity);
    }
}

struct CauldronContents(Vec<JamIngredient>);

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let cauldron_bg_handle = asset_server.load("sprites/cauldron_back.png");
    let shop_front_shelf_handle = asset_server.load("sprites/frontshelf.png");
    let cauldron_top_handle = asset_server.load("sprites/cauldron_top.png");
    let return_handle = asset_server.load("sprites/return_button.png");
    let cauldron_content_handle = asset_server.load("sprites/cauldroncontent.png");
    let cauldron_content_atlas =
        TextureAtlas::from_grid(cauldron_content_handle, Vec2::new(256.0, 256.0), 4, 1);
    let cauldron_content_atlas_handle = texture_atlases.add(cauldron_content_atlas);

    commands
        .spawn(SpriteBundle {
            material: materials.add(cauldron_bg_handle.into()),
            ..Default::default()
        })
        .with(Background)
        .spawn(SpriteBundle {
            material: materials.add(shop_front_shelf_handle.into()),
            transform: Transform::from_xyz(0.0, 0.0, 4.0),
            ..Default::default()
        })
        .with(Background)
        .spawn(SpriteBundle {
            material: materials.add(cauldron_top_handle.into()),
            transform: Transform::from_xyz(0.0, -150.0, 1.0),
            ..Default::default()
        })
        .with(Background)
        .with(DropTarget)
        .with(Cauldron)
        .spawn(SpriteSheetBundle {
            texture_atlas: cauldron_content_atlas_handle,
            transform: Transform::from_xyz(0.0, -150.0, 4.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Background)
        .with(CauldronContent)
        .spawn(ButtonBundle {
            material: materials.add(return_handle.into()),
            style: Style {
                size: Size::new(Val::Px(128.0), Val::Px(128.0)),
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
        .with(Return)
        .with(Background);
}

fn handle_return_click(
    mut state: ResMut<State<GameState>>,
    q_return: Query<&Return>,
    mut event_reader: EventReader<button::ButtonPressedEvent>,
) {
    for button::ButtonPressedEvent(entity) in event_reader.iter() {
        if let Ok(Return) = q_return.get_component(*entity) {
            state.set_next(GameState::Main).unwrap();
        }
    }
}

fn average_colours<I: Iterator<Item = Color>>(it: I) -> Color {
    let mut total = 0;
    let (r, g, b) = it.fold((0.0, 0.0, 0.0), |(r, g, b), colour| {
        total += 1;

        (
            r + colour.r().powi(2),
            g + colour.g().powi(2),
            b + colour.b().powi(2),
        )
    });

    if total > 0 {
        Color::rgb(
            (r / total as f32).sqrt(),
            (g / total as f32).sqrt(),
            (b / total as f32).sqrt(),
        )
    } else {
        Color::rgba(0.0, 0.0, 0.0, 0.0)
    }
}

fn recolour_cauldron(
    contents: Res<CauldronContents>,
    mut q_content: Query<&mut TextureAtlasSprite, With<CauldronContent>>,
) {
    let colour = average_colours(contents.0.iter().map(|i| i.colour()));

    for mut c in q_content.iter_mut() {
        c.color.set_r(colour.r());
        c.color.set_g(colour.g());
        c.color.set_b(colour.b());
        c.color.set_a(colour.a());
    }
}

fn handle_content_drop(
    mut contents: ResMut<CauldronContents>,
    q_cauldron: Query<&Cauldron>,
    q_jam_ingredient: Query<&JamIngredient>,
    mut event_reader: EventReader<DroppedOntoEvent>,
) {
    for DroppedOntoEvent { src, dst } in event_reader.iter() {
        if let (Ok(ingredient), Ok(Cauldron)) = (
            q_jam_ingredient.get_component::<JamIngredient>(*src),
            q_cauldron.get_component(*dst),
        ) {
            contents.0.push(*ingredient);
        }
    }
}

use bevy::prelude::*;
use crate::gamestate::{GameStage, GameState};

pub struct CauldronScenePlugin;

struct Background;

impl Plugin for CauldronScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(GameStage::Main, GameState::Cauldron, setup.system())
        .on_state_exit(GameStage::Main, GameState::Cauldron, teardown.system());
        }
    }

    fn teardown(commands: &mut Commands, q_background: Query<Entity, With<Background>>) {
        for entity in q_background.iter() {
            commands.despawn(entity);
        }
    }

    fn setup(
        commands: &mut Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let cauldron_bg_handle = asset_server.load("sprites/cauldron_back.png");
        let shop_front_shelf_handle = asset_server.load("sprites/frontshelf.png");
        let cauldron_top_handle = asset_server.load("sprites/cauldron_top.png");
        commands
        .spawn(SpriteBundle {
            material: materials.add(cauldron_bg_handle.into()),
            ..Default::default()
        })
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
        });
    }

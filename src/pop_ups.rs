use bevy::prelude::*;
use crate::shop_scene;

pub struct PopUpsPlugin;

impl Plugin for PopUpsPlugin {
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_system(setup.system())
            //.add_system(generate_story.system())
    ;}
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let jam_book_handle = asset_server.load("sprites/jambook.png");
    let cauldron_handle = asset_server.load("sprites/cauldron.png");
    commands
        .spawn(SpriteBundle
        {
            material: materials.add(jam_book_handle.into()),
            transform: Transform::from_xyz(-335.0, -240.0, 7.0),
            ..Default::default()
        })
        .spawn(SpriteBundle
        {
            material: materials.add(cauldron_handle.into()),
            transform: Transform::from_xyz(335.0, -240.0, 7.0),
            ..Default::default()
        });
}

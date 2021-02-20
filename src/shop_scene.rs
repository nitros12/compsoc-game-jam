use bevy::prelude::*;

pub struct ShopScenePlugin;

struct Background;

impl Plugin for ShopScenePlugin {
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_system(setup.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shopfront_handle = asset_server.load("sprites/front.png");
    let background_handle = asset_server.load("sprites/background.png");
    commands
        .spawn(SpriteBundle
        {
            material: materials.add(background_handle.into()),
            ..Default::default()
        })
        .with(Background)
        .spawn(SpriteBundle
        {
            material: materials.add(shopfront_handle.into()),
            ..Default::default()
        })
        .with(Background);
}

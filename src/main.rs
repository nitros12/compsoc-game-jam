use bevy::prelude::*;

mod button;
mod shop_scene;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor
        {
            title: "Game Jam Jam Game".to_string(),
            width: 800.0,
            height: 600.0,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(shop_scene::ShopScenePlugin)
        //.add_plugin(button::ButtonPlugin)
        .add_startup_system(setup_ui.system())
        .run();
}

fn setup_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(UiCameraBundle::default()).
        spawn(OrthographicCameraBundle::new_2d());
}

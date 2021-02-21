use bevy::prelude::*;

mod button;
mod dragging;
mod jam;
mod shop_scene;
mod story_gen;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Game Jam Jam Game".to_string(),
            width: 800.0,
            height: 600.0,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(dragging::DragPlugin)
        .add_plugin(shop_scene::ShopScenePlugin)
        .add_plugin(jam::JamPlugin)
        .add_plugin(story_gen::StoryGenPlugin)
        .add_startup_system(setup_ui.system())
        .run();
}

fn setup_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let jam_texture = asset_server.load("sprites/jam_jar.png");

    commands
        .spawn(UiCameraBundle::default())
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(SpriteBundle {
            material: materials.add(jam_texture.clone().into()),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..Default::default()
        })
        .with(dragging::Hoverable)
        .with(dragging::Draggable)
        .spawn(SpriteBundle {
            material: materials.add(jam_texture.clone().into()),
            transform: Transform::from_xyz(50.0, 50.0, 3.0),
            ..Default::default()
        })
        .with(dragging::DropTarget);
}

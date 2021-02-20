use bevy::prelude::*;

mod button;
mod dragging;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(button::ButtonPlugin)
        .add_plugin(dragging::DragPlugin)
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
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
                border: Rect::all(Val::Px(20.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.65, 0.65, 0.0).into()),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: materials.add(jam_texture.clone().into()),
            ..Default::default()
        })
        .with(dragging::Hoverable)
        .with(dragging::Draggable);
}

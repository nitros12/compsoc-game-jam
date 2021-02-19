use bevy::prelude::*;

mod button;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(button::ButtonPlugin)
        .add_startup_system(setup_ui.system())
        .run();
}

fn setup_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(UiCameraBundle::default()).spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(210.0),
                bottom: Val::Px(10.0),
                ..Default::default()
            },
            border: Rect::all(Val::Px(20.0)),
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.65, 0.65, 0.0).into()),
        ..Default::default()
    });
}

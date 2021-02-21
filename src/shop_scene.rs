use bevy::prelude::*;

pub struct ShopScenePlugin;

struct Background;

struct Moveable {
    move_timer: Timer,
    start: Vec2,
    end: Vec2,
    delay_timer: Timer,
}

impl Plugin for ShopScenePlugin {
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_system(setup.system())
            .add_system(animate_sprites.system())
            .add_system(move_sprites.system())
    ;}
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let shopfront_handle = asset_server.load("sprites/front.png");
    let background_handle = asset_server.load("sprites/background.png");
    let tumbleweed_handle = asset_server.load("sprites/tumbleweedsheet.png");
    let tumbleweed_atlas = TextureAtlas::from_grid(tumbleweed_handle, Vec2::new(32.0, 32.0), 4, 1);
    let tumbleweed_atlas_handle = texture_atlases.add(tumbleweed_atlas);
    let buggy_handle = asset_server.load("sprites/buggy-sheet.png");
    let buggy_atlas = TextureAtlas::from_grid(buggy_handle, Vec2::new(128.0, 64.0), 4, 1);
    let buggy_atlas_handle = texture_atlases.add(buggy_atlas);

    commands
        .spawn(SpriteBundle
        {
            material: materials.add(background_handle.into()),
            ..Default::default()
        })
        .with(Background)

        .spawn(SpriteSheetBundle
        {
            texture_atlas: tumbleweed_atlas_handle,
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Moveable{
            move_timer: Timer::from_seconds(20.0, true),
            start: Vec2::new(-420.0, -50.0),
            end: Vec2::new(420.0, -50.0),
            delay_timer: Timer::from_seconds(15.0, true),
        })

        .spawn(SpriteSheetBundle
        {
            texture_atlas: buggy_atlas_handle,
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Moveable{
            move_timer: Timer::from_seconds(5.0, true),
            start: Vec2::new(520.0, -70.0),
            end: Vec2::new(-520.0, -70.0),
            delay_timer: Timer::from_seconds(40.0, true),
        })

        .spawn(SpriteBundle
        {
            material: materials.add(shopfront_handle.into()),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..Default::default()
        })
        .with(Background);
}

fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn move_sprites(
    time: Res<Time>,
    mut query: Query<(&mut Moveable, &mut Transform)>,
) {
    for (mut moveable, mut transform) in query.iter_mut()
    {
        if !moveable.move_timer.tick(time.delta_seconds()).just_finished() && !moveable.move_timer.paused()
        {
            let new_pos = moveable.start + (moveable.end - moveable.start) * moveable.move_timer.percent();
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        }
        else if !moveable.delay_timer.tick(time.delta_seconds()).just_finished()
        {
            moveable.move_timer.pause();
        } else {
            moveable.move_timer.reset();
            moveable.move_timer.unpause();
        }
    }
}

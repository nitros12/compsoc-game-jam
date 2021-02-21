use bevy::prelude::*;
use rand::seq::SliceRandom;

pub struct ShopScenePlugin;

struct Background;

static PHRASES: &[&[&str]] = &[
/*Intro*/    &["HUNGER I was scavenging for food when ", "The other day, ", "In a firefight, ", "Before the war, "],
/*Villain*/  &["STRENGTH a raider far stronger than me ", "a rival gang ", "ANTI-VENOM a mutated snake with potent venom ", "an Old War soldier ", "an enemy fuel convoy ", "CURE DISEASE a feral dog, riddled with diseases, "],
/*Adjective*/&["angrily ", "furiously ", "violently ", "suddenly "],
/*Action*/   &["COAGULANT stabbed ", "robbed ", "destroyed ", "hunted ", "shot at "],
/*Hero*/     &["my raiding party ", "me ", "my war-dog ", "SPEED my armoured truck, leaving me slow, ", "HUNGER my food supplies "],
/*joining*/  &["whilst I was ", "when I was ", "after I was caught ", "for "],
/*Action*/   &["INVISIBILTY trying to steal ", "destroying ", "SPEED escaping with ", "running over ", "gambling away ", "POISON poisoning "],
/*belonging*/&["their water supply, ", "their supplies, ", "their credits, ", "their jam, ", "FLAMMABLE their fuel, ", "their Old World relics, ", "FLIGHT their pre-war iron bird "],
/*belonging*/&["so we ", "so I ", "and then I ", "and then we "],
/*belonging*/&["engaged them in hand to hand combat, ", "began shooting at them, ", "turned and ran away, ", "offered them a truce, ", "told them to surrender, "],
/*belonging*/&["but then ", "unfortunately this was interrupted when ", "before this could happen ", "suddenly, out of nowhere "],
/*belonging*/&["a huge explosion went off, which caused ", "a passionate glance was exchanged, which caused ", "ANTI-VENOM a poisoned trap clamped on my leg , causing ", "a severe gust of rad-wind tore through the valley, causing ", "STRENGTH my body became suddenly weak, causing "],
/*belonging*/&["COAGULANT my leg to fall off. ", "CURE my raid members to become violently sick. ", "FLAMMABLE my matches to get wet. ", "NIGHTVISION everything to go dark. "]
];

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
            .add_system(gen_story.system())
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
        .with(Background)

        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                position_type: PositionType::Absolute,
                max_size: Size {
                    width: Val::Px(520.0),
                    height: Val::Px(100.0),
                    ..Default::default()
                },
                position: Rect {
                    top: Val::Px(500.0),
                    left: Val::Px(140.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            text: Text::with_section(
                "Read the instructions, the game will start soon!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .with(Timer::from_seconds(5.0, true));

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

fn gen_story(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut Text)>
)
{
    for (mut timer, mut text) in query.iter_mut(){
        if !timer.tick(time.delta_seconds()).just_finished()
        {
            return;
        }

        let mut story = String::from("");
        for x in 0..13
        {
            let var1 = PHRASES[x].choose(&mut rand::thread_rng()).unwrap().to_string();
            &story.push_str(&var1);
        }
        &story.push_str("As you can tell, I am in deperate need of assistance, do you have any jam that could help me ensure this doesn't happen again?");
        text.sections[0].value = format!("{:2}", story)
    }
}

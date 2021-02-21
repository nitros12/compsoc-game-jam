use std::collections::HashSet;

use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::cauldron_scene::CauldronContents;
use crate::jam;
use crate::jam::{JamEffect, JamIngredient};
use crate::utils::average_colours;
use crate::{
    dragging::{self, DroppedOntoEvent},
    gamestate::{GameStage, GameState},
};

pub struct ShopScenePlugin;

struct StoryAssets {
    story_timer: Timer,
    story_text: String,
    story_requirements: HashSet<JamEffect>,
    story_met: bool,
}

struct Story;
struct JamJar;
struct Score(u64);

static PHRASES: &[&[(Option<JamEffect>, &str)]] = &[
    /*Intro*/
    &[
        (Some(JamEffect::Hunger), "I was scavenging for food when "),
        (None, "The other day, "),
        (None, "In a firefight, "),
        (None, "Before the war, "),
    ],
    /*Villain*/
    &[
        (
            Some(JamEffect::SuperHumanStrength),
            "a raider far stronger than me ",
        ),
        (None, "a rival gang "),
        (
            Some(JamEffect::Antivenom),
            "a mutated snake with potent venom ",
        ),
        (None, "an Old War soldier "),
        (None, "an enemy fuel convoy "),
        (
            Some(JamEffect::CureDisease),
            "a feral dog, riddled with diseases, ",
        ),
    ],
    /*Adjective*/
    &[
        (None, "angrily "),
        (None, "furiously "),
        (None, "violently "),
        (None, "suddenly "),
    ],
    /*Action*/
    &[
        (Some(JamEffect::Coagulant), "stabbed "),
        (None, "robbed "),
        (None, "destroyed "),
        (None, "hunted "),
        (None, "shot at "),
    ],
    /*Hero*/
    &[
        (None, "my raiding party "),
        (None, "me "),
        (None, "my war-dog "),
        (
            Some(JamEffect::Speed),
            "my armoured truck, leaving me slow, ",
        ),
        (Some(JamEffect::Hunger), "my food supplies "),
    ],
    /*joining*/
    &[
        (None, "whilst I was "),
        (None, "when I was "),
        (None, "after I was caught "),
        (None, "for "),
    ],
    /*Action*/
    &[
        (Some(JamEffect::Invisibility), "trying to steal "),
        (None, "destroying "),
        (Some(JamEffect::Speed), "escaping with "),
        (None, "running over "),
        (None, "gambling away "),
        (Some(JamEffect::Poison), "poisoning "),
    ],
    /*belonging*/
    &[
        (None, "their water supply, "),
        (None, "their supplies, "),
        (None, "their credits, "),
        (None, "their jam, "),
        (Some(JamEffect::Flammable), "their fuel, "),
        (None, "their Old World relics, "),
        (Some(JamEffect::Flight), "their pre-war iron bird "),
    ],
    /*belonging*/
    &[
        (None, "so we "),
        (None, "so I "),
        (None, "and then I "),
        (None, "and then we "),
    ],
    /*belonging*/
    &[
        (None, "engaged them in hand to hand combat, "),
        (None, "began shooting at them, "),
        (None, "turned and ran away, "),
        (None, "offered them a truce, "),
        (None, "told them to surrender, "),
        (Some(JamEffect::HideousLaughter), "tried to diffuse the situation with a joke ")
    ],
    /*belonging*/
    &[
        (None, "but then "),
        (None, "unfortunately this was interrupted when "),
        (None, "before this could happen "),
        (None, "suddenly, out of nowhere "),
    ],
    /*belonging*/
    &[
        (None, "a huge explosion went off, which caused "),
        (None, "a passionate glance was exchanged, which caused "),
        (
            Some(JamEffect::Antivenom),
            "a poisoned trap clamped on my leg , causing ",
        ),
        (
            None,
            "a severe gust of rad-wind tore through the valley, causing ",
        ),
        (
            Some(JamEffect::SuperHumanStrength),
            "my body became suddenly weak, causing ",
        ),
    ],
    /*belonging*/
    &[
        (Some(JamEffect::Coagulant), "my leg to fall off. "),
        (
            Some(JamEffect::CureDisease),
            "my raid members to become violently sick. ",
        ),
        (Some(JamEffect::Flammable), "my matches to get wet. "),
        (Some(JamEffect::NightVision), "everything to go dark. "),
    ],
];

struct Moveable {
    move_timer: Timer,
    start: Vec2,
    end: Vec2,
    delay_timer: Timer,
}

impl Plugin for ShopScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_assets.system())
            .add_startup_system_to_stage(StartupStage::PreStartup, setup_jam_jar_assets.system())
            .on_state_enter(GameStage::Main, GameState::Main, setup.system())
            .add_system(move_sprites.system())
            .add_system(animate_sprites.system())
            .add_system(gen_story.system())
            .on_state_update(
                GameStage::Main,
                GameState::Main,
                jam_jar_clone_on_drag.system(),
            )
            .on_state_update(
                GameStage::Main,
                GameState::Main,
                jam_jar_remove_on_drop.system(),
            )
            .on_state_update(GameStage::Main, GameState::Main, handle_jam_drop.system())
            .on_state_update(GameStage::Main, GameState::Main, recolour_jam_jar.system())
            .on_state_exit(GameStage::Main, GameState::Main, teardown.system());
    }
}

struct Background;

fn teardown(commands: &mut Commands, q_background: Query<Entity, With<Background>>) {
    for entity in q_background.iter() {
        commands.despawn_recursive(entity);
    }
}

fn setup_assets(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let story_timer = Timer::from_seconds(20.0, true);

    let story_text =
    "Welcome to the Lad's Post-Apocalyptic Jam Store! The aim of the game is simple, satisfy our needy customers! Each customer will have a specific set of effects that they want their order of jam to fulfill, and this will be communicated to you via a story of their escapades! Use the JamBook in the bottom left to determine which ingredients you need to use, and mix those ingredients in the Cauldron Room! Be warned, the customers are impatient!"
    .to_string();

    commands.insert_resource(StoryAssets {
        story_timer,
        story_text,
        story_requirements: HashSet::new(),
        story_met: true,
    });
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    story_assets: Res<StoryAssets>,
) {
    let shopfront_handle = asset_server.load("sprites/front.png");
    let shop_score_handle = asset_server.load("sprites/score_board.png");
    let background_handle = asset_server.load("sprites/background.png");
    let tumbleweed_handle = asset_server.load("sprites/tumbleweedsheet.png");
    let tumbleweed_atlas = TextureAtlas::from_grid(tumbleweed_handle, Vec2::new(32.0, 32.0), 4, 1);
    let tumbleweed_atlas_handle = texture_atlases.add(tumbleweed_atlas);
    let buggy_handle = asset_server.load("sprites/buggy-sheet.png");
    let buggy_atlas = TextureAtlas::from_grid(buggy_handle, Vec2::new(128.0, 64.0), 4, 1);
    let buggy_atlas_handle = texture_atlases.add(buggy_atlas);

    commands
        .spawn(SpriteBundle {
            material: materials.add(shop_score_handle.into()),
            transform: Transform::from_xyz(0.0, 0.0, 4.0),
            ..Default::default()
        })
        .with(Background)
        .spawn(SpriteBundle {
            material: materials.add(background_handle.into()),
            ..Default::default()
        })
        .with(Background)
        .spawn(SpriteSheetBundle {
            texture_atlas: tumbleweed_atlas_handle,
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Moveable {
            move_timer: Timer::from_seconds(20.0, true),
            start: Vec2::new(-420.0, -50.0),
            end: Vec2::new(420.0, -50.0),
            delay_timer: Timer::from_seconds(15.0, true),
        })
        .with(Background)
        .spawn(SpriteSheetBundle {
            texture_atlas: buggy_atlas_handle,
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Moveable {
            move_timer: Timer::from_seconds(5.0, true),
            start: Vec2::new(520.0, -70.0),
            end: Vec2::new(-520.0, -70.0),
            delay_timer: Timer::from_seconds(40.0, true),
        })
        .with(Background)
        .spawn(SpriteBundle {
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
                story_assets.story_text.to_string(),
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
        .with(Background)
        .with(Story)
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
                    top: Val::Px(34.0),
                    right: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            text: Text::with_section(
                0.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .with(Background)
        .with(Score);

    spawn_jamjar(commands, &*asset_server, &mut *materials);
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

fn move_sprites(time: Res<Time>, mut query: Query<(&mut Moveable, &mut Transform)>) {
    for (mut moveable, mut transform) in query.iter_mut() {
        if !moveable
            .move_timer
            .tick(time.delta_seconds())
            .just_finished()
            && !moveable.move_timer.paused()
        {
            let new_pos =
                moveable.start + (moveable.end - moveable.start) * moveable.move_timer.percent();
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        } else if !moveable
            .delay_timer
            .tick(time.delta_seconds())
            .just_finished()
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
    mut assets: ResMut<StoryAssets>,
    mut query: Query<(&mut Story, &mut Text)>,
) {
    for (mut story, mut text) in query.iter_mut() {
        if !assets
            .story_timer
            .tick(time.delta_seconds())
            .just_finished()
        {
            return;
        }

        let mut effects = Vec::new();
        let mut temp_story = String::from("");
        for x in 0..13 {
            let (effect, text_fragment) = PHRASES[x].choose(&mut rand::thread_rng()).unwrap();
            temp_story.push_str(text_fragment);
            if let Some(effect) = effect {
                effects.push(effect);
            }
        }
        temp_story.push_str("As you can tell, I am in deperate need of assistance, do you have any jam that could help me ensure this doesn't happen again?");
        assets.story_text = temp_story;
        assets.story_requirements = effects.into_iter().cloned().collect();
        assets.story_met = false;
        text.sections[0].value = format!("{:2}", assets.story_text);
    }
}

fn spawn_jamjar(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    let jam_jar = asset_server.load("sprites/jam_jar.png");

    commands
        .spawn(SpriteBundle {
            material: materials.add(jam_jar.into()),
            transform: Transform::from_xyz(-150.0, -105.0, 14.0),
            ..Default::default()
        })
        .with(JamJar)
        .with(Background)
        .with(dragging::Hoverable)
        .with(dragging::Draggable);
}

fn jam_jar_clone_on_drag(
    commands: &mut Commands,
    q_jamjar: Query<&JamJar>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut event_reader: EventReader<dragging::DraggedEvent>,
) {
    for dragging::DraggedEvent(entity) in event_reader.iter() {
        if let Ok(JamJar) = q_jamjar.get_component(*entity) {
            spawn_jamjar(commands, &*asset_server, &mut *materials);
        }
    }
}

fn jam_jar_remove_on_drop(
    commands: &mut Commands,
    q_jam_ingredient: Query<&JamJar>,
    mut event_reader: EventReader<dragging::DroppedEvent>,
) {
    for dragging::DroppedEvent(entity) in event_reader.iter() {
        if let Ok(JamJar) = q_jam_ingredient.get_component(*entity) {
            commands.despawn_recursive(*entity);
        }
    }
}

fn handle_jam_drop(
    contents: Res<CauldronContents>,
    mut story: ResMut<StoryAssets>,
    q_jam_jar: Query<&JamJar>,
    mut q_score: Query<(&mut Text, &mut Score)>,
    mut event_reader: EventReader<DroppedOntoEvent>,
) {
    if story.story_met {
        return;
    }

    for DroppedOntoEvent { src, dst } in event_reader.iter() {
        if let (Ok(JamJar), _) = (q_jam_jar.get_component(*src), ()) {
            let effects = JamIngredient::calculate_effects(&contents.0);

            if story.story_requirements.is_subset(&effects) {
                story.story_met = true;

                for (mut text, mut score) in q_score.iter_mut() {
                    score.0 += 1;
                    text.sections[0].value = score.0.to_string();
                }
            }

            // TODO: make client go away
        }
    }
}

struct JamJarAssets {
    normal: Handle<Texture>,
    filled: Handle<Texture>,
    bg: Handle<Texture>,
}

fn setup_jam_jar_assets(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let normal = asset_server.load("sprites/jam_jar.png");
    let filled = asset_server.load("sprites/fill_jar.png");
    let bg = asset_server.load("sprites/fill_jar_filling.png");

    commands.insert_resource(JamJarAssets { normal, filled, bg });
}

fn recolour_jam_jar(
    commands: &mut Commands,
    contents: Res<CauldronContents>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    jam_jar_assets: Res<JamJarAssets>,
    mut q_material: Query<&mut Handle<ColorMaterial>>,
    mut q_jamjar: Query<(Entity, Option<&Children>), With<JamJar>>,
) {
    let colour = average_colours(contents.0.iter().map(|i| i.colour()));

    if colour.a() < 0.5 {
        for (entity, children) in q_jamjar.iter_mut() {
            let mut material = q_material.get_component_mut(entity).unwrap();

            if let Some(children) = children {
                for entity in children.as_ref() {
                    commands.despawn(*entity);
                }

                if !children.is_empty() {
                    *material = materials.add(jam_jar_assets.normal.clone().into());
                }
            }
        }
    } else {
        for (entity, children) in q_jamjar.iter_mut() {
            let mut material = q_material.get_component_mut(entity).unwrap();

            if let Some(children) = children {
                for child in children.as_ref() {
                    let bg = q_material
                        .get_component::<Handle<ColorMaterial>>(*child)
                        .unwrap();

                    let mat = materials.get_mut(bg).unwrap();
                    mat.color.set_r(colour.r());
                    mat.color.set_g(colour.g());
                    mat.color.set_b(colour.b());
                    mat.color.set_a(colour.a());
                }
            } else {
                let bg = materials.add(jam_jar_assets.bg.clone().into());
                let mat = materials.get_mut(&bg).unwrap();
                mat.color.set_r(colour.r());
                mat.color.set_g(colour.g());
                mat.color.set_b(colour.b());
                mat.color.set_a(colour.a());

                let child = commands
                    .spawn(SpriteBundle {
                        material: bg,
                        transform: Transform::from_xyz(0.0, 0.0, 14.0),
                        ..Default::default()
                    })
                    .current_entity()
                    .unwrap();

                commands.push_children(entity, &[child]);

                *material = materials.add(jam_jar_assets.filled.clone().into());
            }
        }
    }
}

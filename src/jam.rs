use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::dragging;
use crate::gamestate::{GameStage, GameState};

pub struct JamPlugin;

impl Plugin for JamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_assets.system())
            .on_state_enter(GameStage::Main, GameState::Cauldron, setup.system())
            .on_state_update(
                GameStage::Main,
                GameState::Cauldron,
                jam_clone_on_drag.system(),
            )
            .on_state_update(
                GameStage::Main,
                GameState::Cauldron,
                jam_remove_on_drop.system(),
            )
            .on_state_exit(GameStage::Main, GameState::Cauldron, teardown.system());
    }
}

fn setup_assets(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let ingredients = JamIngredient::all()
        .into_iter()
        .map(|&i| (i, asset_server.load(i.asset_path())))
        .collect();

    let effects = JamEffect::all()
        .into_iter()
        .map(|&i| (i, asset_server.load(i.asset_path())))
        .collect();

    commands.insert_resource(JamAssets {
        ingredients,
        effects,
    });
}

fn setup(
    commands: &mut Commands,
    assets: Res<JamAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for &ingredient in JamIngredient::all() {
        spawn_ingredient(commands, ingredient, &*assets, &mut *materials);
    }
}

fn teardown(commands: &mut Commands, q_ingredients: Query<Entity, With<JamIngredient>>) {
    for entity in q_ingredients.iter() {
        commands.despawn(entity);
    }
}

fn spawn_ingredient(
    commands: &mut Commands,
    ingredient: JamIngredient,
    assets: &JamAssets,
    materials: &mut Assets<ColorMaterial>,
) {
    commands
        .spawn(SpriteBundle {
            material: materials.add(ingredient.asset_for(assets).into()),
            transform: ingredient.initial_transform(),
            ..Default::default()
        })
        .with(ingredient)
        .with(dragging::Hoverable)
        .with(dragging::Draggable);
}

fn jam_clone_on_drag(
    commands: &mut Commands,
    jam_assets: Res<JamAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_ingredients: Query<&JamIngredient>,
    mut event_reader: EventReader<dragging::DraggedEvent>,
) {
    for dragging::DraggedEvent(entity) in event_reader.iter() {
        if let Ok(ingredient) = q_ingredients.get_component(*entity) {
            spawn_ingredient(commands, *ingredient, &*jam_assets, &mut *materials);
        }
    }
}

fn jam_remove_on_drop(
    commands: &mut Commands,
    q_jam_ingredient: Query<&JamIngredient>,
    mut event_reader: EventReader<dragging::DroppedEvent>,
) {
    for dragging::DroppedEvent(entity) in event_reader.iter() {
        if let Ok(_) = q_jam_ingredient.get_component::<JamIngredient>(*entity) {
            commands.despawn(*entity);
        }
    }
}

pub struct JamAssets {
    ingredients: HashMap<JamIngredient, Handle<Texture>>,
    effects: HashMap<JamEffect, Handle<Texture>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum JamIngredient {
    Petrol,
    Urine,
    GunPowder,
    BathWater,
    AppleSeeds,
    Strawberries,
    Lemons,
    Damsons,
    HumanFlesh,
    MotorOil,
    Absinth,
    Bleach,
    Sand,
    Sugar,
    Salt,
    Sakura,
}

impl JamIngredient {
    pub fn all() -> &'static [JamIngredient] {
        &[
            JamIngredient::Petrol,
            JamIngredient::Urine,
            JamIngredient::GunPowder,
            JamIngredient::BathWater,
            JamIngredient::AppleSeeds,
            JamIngredient::Strawberries,
            JamIngredient::Lemons,
            JamIngredient::Damsons,
            JamIngredient::HumanFlesh,
            JamIngredient::MotorOil,
            JamIngredient::Absinth,
            JamIngredient::Bleach,
            JamIngredient::Sand,
            JamIngredient::Sugar,
            JamIngredient::Salt,
            JamIngredient::Sakura,
        ]
    }

    fn initial_position(self) -> (f32, f32) {
        match self {
            JamIngredient::Petrol => (-280.0, 250.0),
            JamIngredient::Urine => (-200.0, 250.0),
            JamIngredient::GunPowder => (-120.0, 250.0),
            JamIngredient::BathWater => (-40.0, 250.0),
            JamIngredient::AppleSeeds => (40.0, 250.0),
            JamIngredient::Strawberries => (120.0, 250.0),
            JamIngredient::Lemons => (200.0, 250.0),
            JamIngredient::Damsons => (280.0, 250.0),

            JamIngredient::HumanFlesh => (-280.0, 170.0),
            JamIngredient::MotorOil => (-200.0, 170.0),
            JamIngredient::Absinth => (-120.0, 170.0),
            JamIngredient::Bleach => (-40.0, 170.0),
            JamIngredient::Sand => (40.0, 170.0),
            JamIngredient::Sugar => (120.0, 170.0),
            JamIngredient::Salt => (200.0, 170.0),
            JamIngredient::Sakura => (280.0, 170.0),
        }
    }

    pub fn colour(self) -> Color {
        match self {
            JamIngredient::Petrol => Color::rgb_u8(237, 237, 84),
            JamIngredient::Urine => Color::rgb_u8(255, 172, 0),
            JamIngredient::GunPowder => Color::rgb_u8(140, 133, 113),
            JamIngredient::BathWater => Color::rgb_u8(207, 246, 246),
            JamIngredient::AppleSeeds => Color::rgb_u8(6, 38, 39),
            JamIngredient::Strawberries => Color::rgb_u8(220, 103, 80),
            JamIngredient::Lemons => Color::rgb_u8(183, 220, 80),
            JamIngredient::Damsons => Color::rgb_u8(95, 69, 118),
            JamIngredient::HumanFlesh => Color::rgb_u8(142, 53, 41),
            JamIngredient::MotorOil => Color::rgb_u8(18, 37, 25),
            JamIngredient::Absinth => Color::rgb_u8(0, 234, 82),
            JamIngredient::Bleach => Color::rgb_u8(7, 171, 247),
            JamIngredient::Sand => Color::rgb_u8(186, 162, 58),
            JamIngredient::Sugar => Color::rgb_u8(170, 216, 222),
            JamIngredient::Salt => Color::rgb_u8(170, 222, 194),
            JamIngredient::Sakura => Color::rgb_u8(220, 170, 216),
        }
    }

    fn initial_transform(self) -> Transform {
        let (x, y) = self.initial_position();
        Transform::from_xyz(x, y, 6.0)
    }

    fn asset_path(self) -> &'static str {
        match self {
            JamIngredient::Petrol => "sprites/petrol.png",
            JamIngredient::Urine => "sprites/urine.png",
            JamIngredient::GunPowder => "sprites/gunpowder.png",
            JamIngredient::BathWater => "sprites/bathwater.png",
            JamIngredient::AppleSeeds => "sprites/appleseeds.png",
            JamIngredient::Strawberries => "sprites/strawberry.png",
            JamIngredient::Lemons => "sprites/lemon.png",
            JamIngredient::Damsons => "sprites/damsons.png",
            JamIngredient::HumanFlesh => "sprites/humanflesh.png",
            JamIngredient::MotorOil => "sprites/motoroil.png",
            JamIngredient::Absinth => "sprites/absinthe.png",
            JamIngredient::Bleach => "sprites/bleach.png",
            JamIngredient::Sand => "sprites/sand.png",
            JamIngredient::Sugar => "sprites/sugar.png",
            JamIngredient::Salt => "sprites/salt.png",
            JamIngredient::Sakura => "sprites/sakura.png",
        }
    }

    pub fn asset_for(self, assets: &JamAssets) -> Handle<Texture> {
        assets.ingredients.get(&self).unwrap().clone()
    }

    pub fn name(self) -> &'static str {
        match self {
            JamIngredient::Petrol => "Petrol",
            JamIngredient::Urine => "Urine",
            JamIngredient::GunPowder => "Gun powder",
            JamIngredient::BathWater => "Bath water",
            JamIngredient::AppleSeeds => "Apple seeds",
            JamIngredient::Strawberries => "Strawberries",
            JamIngredient::Lemons => "Lemons",
            JamIngredient::Damsons => "Damsons",
            JamIngredient::HumanFlesh => "Human flesh",
            JamIngredient::MotorOil => "Motor oil",
            JamIngredient::Absinth => "Absinth",
            JamIngredient::Bleach => "Bleach",
            JamIngredient::Sand => "Sand",
            JamIngredient::Sugar => "Sugar",
            JamIngredient::Salt => "Salt",
            JamIngredient::Sakura => "Sakura",
        }
    }

    pub fn effects(self) -> &'static [JamEffect] {
        match self {
            JamIngredient::Petrol => &[JamEffect::SuperHumanStrength, JamEffect::Flammable],
            JamIngredient::Urine => &[JamEffect::NightVision, JamEffect::HideousLaughter],
            JamIngredient::GunPowder => &[JamEffect::Flight, JamEffect::Speed],
            JamIngredient::BathWater => &[JamEffect::GreaterHeal, JamEffect::CureDisease],
            JamIngredient::AppleSeeds => &[JamEffect::Poison],
            JamIngredient::Strawberries => &[JamEffect::CureDisease],
            JamIngredient::Lemons => &[JamEffect::GreaterHeal],
            JamIngredient::Damsons => &[JamEffect::Flight],
            JamIngredient::HumanFlesh => &[JamEffect::Hunger, JamEffect::Coagulant],
            JamIngredient::MotorOil => &[JamEffect::Speed, JamEffect::Poison],
            JamIngredient::Absinth => &[JamEffect::SuperHumanStrength, JamEffect::Flammable],
            JamIngredient::Bleach => &[JamEffect::Hunger, JamEffect::HideousLaughter],
            JamIngredient::Sand => &[JamEffect::Coagulant, JamEffect::Antivenom],
            JamIngredient::Sugar => &[JamEffect::Invisibility],
            JamIngredient::Salt => &[JamEffect::NightVision],
            JamIngredient::Sakura => &[JamEffect::Invisibility, JamEffect::Antivenom],
        }
    }

    pub fn calculate_effects(ingredients: &[Self]) -> HashSet<JamEffect> {
        let mut seen = HashMap::new();

        for effect in ingredients.iter().flat_map(|i| i.effects()) {
            *seen.entry(effect).or_insert(0) += 1;
        }

        seen.into_iter()
            .filter(|(_k, v)| *v > 1)
            .map(|(k, _)| *k)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum JamEffect {
    NightVision,
    SuperHumanStrength,
    Poison,
    Hunger,
    GreaterHeal,
    CureDisease,
    Antivenom,
    Coagulant,
    Flammable,
    Invisibility,
    Speed,
    Flight,
    HideousLaughter,
}

impl JamEffect {
    pub fn all() -> &'static [JamEffect] {
        &[
            JamEffect::NightVision,
            JamEffect::SuperHumanStrength,
            JamEffect::Poison,
            JamEffect::Hunger,
            JamEffect::GreaterHeal,
            JamEffect::CureDisease,
            JamEffect::Antivenom,
            JamEffect::Coagulant,
            JamEffect::Flammable,
            JamEffect::Invisibility,
            JamEffect::Speed,
            JamEffect::Flight,
            JamEffect::HideousLaughter,
        ]
    }

    fn asset_path(self) -> &'static str {
        match self {
            JamEffect::NightVision => "sprites/nightvision.png",
            JamEffect::SuperHumanStrength => "sprites/superstrength.png",
            JamEffect::Poison => "sprites/poison.png",
            JamEffect::Hunger => "sprites/hunger.png",
            JamEffect::GreaterHeal => "sprites/heal.png",
            JamEffect::CureDisease => "sprites/cure.png",
            JamEffect::Antivenom => "sprites/antivenom.png",
            JamEffect::Coagulant => "sprites/coagulant.png",
            JamEffect::Flammable => "sprites/flammable.png",
            JamEffect::Invisibility => "sprites/invisibility.png",
            JamEffect::Speed => "sprites/speed.png",
            JamEffect::Flight => "sprites/flight.png",
            JamEffect::HideousLaughter => "sprites/laughing.png",
        }
    }

    pub fn asset_for(self, assets: &JamAssets) -> Handle<Texture> {
        assets.effects.get(&self).unwrap().clone()
    }

    pub fn name(self) -> &'static str {
        match self {
            JamEffect::NightVision => "Night vision",
            JamEffect::SuperHumanStrength => "Super human strength",
            JamEffect::Poison => "Poison",
            JamEffect::Hunger => "Hunger",
            JamEffect::GreaterHeal => "Greater heal",
            JamEffect::CureDisease => "Cure Disease",
            JamEffect::Antivenom => "Antivenom",
            JamEffect::Coagulant => "Coagulant",
            JamEffect::Flammable => "Flammable",
            JamEffect::Invisibility => "Invisibility",
            JamEffect::Speed => "Speed",
            JamEffect::Flight => "Flight",
            JamEffect::HideousLaughter => "Hideous laughter",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            JamEffect::NightVision => "See, in the dark",
            JamEffect::SuperHumanStrength => "HULK! SMASH!",
            JamEffect::Poison => "You feel ill",
            JamEffect::Hunger => "I am very hungry, give me the butter",
            JamEffect::GreaterHeal => "Your wounds heal and your body feels light",
            JamEffect::CureDisease => "You are suddenly free from disease",
            JamEffect::Antivenom => "You are cured from all venoms",
            JamEffect::Coagulant => "Clots blood when applied",
            JamEffect::Flammable => "Sets fire to anything the jam touches",
            JamEffect::Invisibility => "Invisibility",
            JamEffect::Speed => "Radiation blasts your cells, you become faster",
            JamEffect::Flight => "Your body fils with energy, so much that you fly?",
            JamEffect::HideousLaughter => {
                "You perceive everything as hilariously funny and fall into a fit of laugher."
            }
        }
    }
}

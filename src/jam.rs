use std::collections::HashMap;

use bevy::prelude::*;

use crate::dragging;

pub struct JamPlugin;

impl Plugin for JamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_assets.system())
            .add_startup_system(setup_jams.system())
            .add_system_to_stage(CoreStage::PostUpdate, jam_clone_on_drag.system())
            // .add_system_to_stage(CoreStage::PostUpdate, jam_remove_on_drop.system())
            ;
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

fn setup_jams(
    commands: &mut Commands,
    assets: Res<JamAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for &ingredient in JamIngredient::all() {
        spawn_ingredient(commands, ingredient, &*assets, &mut *materials);
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
        .with(dragging::Draggable)
        ;
}

fn jam_clone_on_drag(
    commands: &mut Commands,
    jam_assets: Res<JamAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_dragged: Query<&JamIngredient, Added<dragging::Dragged>>,
) {
    for &ingredient in q_dragged.iter() {
        spawn_ingredient(commands, ingredient, &*jam_assets, &mut *materials);
    }
}

fn jam_remove_on_drop(
    commands: &mut Commands,
    q_dropped: Query<Entity, (With<JamIngredient>, Added<dragging::Dropped>)>,
) {
    for entity in q_dropped.iter() {
        commands.despawn(entity);
    }
}

pub struct JamAssets {
    ingredients: HashMap<JamIngredient, Handle<Texture>>,
    effects: HashMap<JamEffect, Handle<Texture>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
    fn all() -> &'static [JamIngredient] {
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

    fn initial_position(self) -> (f32, f32, f32) {
        match self {
            JamIngredient::Petrol => (-250.0, 50.0, 6.0),
            JamIngredient::Urine => (-230.0, 50.0, 7.0),
            JamIngredient::GunPowder => (-210.0, 50.0, 8.0),
            JamIngredient::BathWater => (-190.0, 50.0, 9.0),
            JamIngredient::AppleSeeds => (-170.0, 50.0, 10.0),
            JamIngredient::Strawberries => (-150.0, 50.0, 11.0),
            JamIngredient::Lemons => (-130.0, 50.0, 12.0),
            JamIngredient::Damsons => (-110.0, 50.0, 13.0),
            JamIngredient::HumanFlesh => (-90.0, 50.0, 14.0),
            JamIngredient::MotorOil => (-70.0, 50.0, 15.0),
            JamIngredient::Absinth => (-50.0, 50.0, 16.0),
            JamIngredient::Bleach => (-30.0, 50.0, 17.0),
            JamIngredient::Sand => (-10.0, 50.0, 18.0),
            JamIngredient::Sugar => (10.0, 50.0, 19.0),
            JamIngredient::Salt => (30.0, 50.0, 20.0),
            JamIngredient::Sakura => (50.0, 50.0, 21.0),
        }
    }

    fn initial_transform(self) -> Transform {
        let (x, y, z) = self.initial_position();
        Transform::from_xyz(x, y, z)
    }

    fn asset_path(self) -> &'static str {
        match self {
            // JamIngredient::Petrol => "sprites/petrol.png",
            // JamIngredient::Urine => "sprites/urine.png",
            // JamIngredient::GunPowder => "sprites/gun_powder.png",
            // JamIngredient::BathWater => "sprites/bath_water.png",
            // JamIngredient::AppleSeeds => "sprites/apple_seeds.png",
            // JamIngredient::Strawberries => "sprites/strawberries.png",
            // JamIngredient::Lemons => "sprites/lemons.png",
            // JamIngredient::Damsons => "sprites/damsons.png",
            // JamIngredient::HumanFlesh => "sprites/human_flesh.png",
            // JamIngredient::MotorOil => "sprites/motor_oil.png",
            // JamIngredient::Absinth => "sprites/absinth.png",
            // JamIngredient::Bleach => "sprites/bleach.png",
            // JamIngredient::Sand => "sprites/sand.png",
            // JamIngredient::Sugar => "sprites/sugar.png",
            // JamIngredient::Salt => "sprites/salt.png",
            // JamIngredient::Sakura => "sprites/sakura.png",
            JamIngredient::Petrol => "sprites/jambook.png",
            JamIngredient::Urine => "sprites/jambook.png",
            JamIngredient::GunPowder => "sprites/jambook.png",
            JamIngredient::BathWater => "sprites/jambook.png",
            JamIngredient::AppleSeeds => "sprites/jambook.png",
            JamIngredient::Strawberries => "sprites/jambook.png",
            JamIngredient::Lemons => "sprites/jambook.png",
            JamIngredient::Damsons => "sprites/jambook.png",
            JamIngredient::HumanFlesh => "sprites/jambook.png",
            JamIngredient::MotorOil => "sprites/jambook.png",
            JamIngredient::Absinth => "sprites/jambook.png",
            JamIngredient::Bleach => "sprites/jambook.png",
            JamIngredient::Sand => "sprites/jambook.png",
            JamIngredient::Sugar => "sprites/jambook.png",
            JamIngredient::Salt => "sprites/jambook.png",
            JamIngredient::Sakura => "sprites/jambook.png",
        }
    }

    fn asset_for(self, assets: &JamAssets) -> Handle<Texture> {
        assets.ingredients.get(&self).unwrap().clone()
    }

    fn name(self) -> &'static str {
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

    fn effects(self) -> &'static [JamEffect] {
        match self {
            JamIngredient::Petrol => &[JamEffect::SuperHumanStrength, JamEffect::Flight],
            JamIngredient::Urine => &[JamEffect::NightVision],
            JamIngredient::GunPowder => &[JamEffect::Flight, JamEffect::Speed],
            JamIngredient::BathWater => &[JamEffect::GreaterHeal, JamEffect::CureAll],
            JamIngredient::AppleSeeds => &[JamEffect::Poison],
            JamIngredient::Strawberries => &[JamEffect::CureAll],
            JamIngredient::Lemons => &[JamEffect::GreaterHeal],
            JamIngredient::Damsons => &[JamEffect::Speed],
            JamIngredient::HumanFlesh => &[JamEffect::Hunger],
            JamIngredient::MotorOil => &[JamEffect::Speed, JamEffect::Poison],
            JamIngredient::Absinth => &[JamEffect::SuperHumanStrength, JamEffect::HideousLaughter],
            JamIngredient::Bleach => &[JamEffect::Hunger, JamEffect::CureAll],
            JamIngredient::Sand => &[],
            JamIngredient::Sugar => &[],
            JamIngredient::Salt => &[],
            JamIngredient::Sakura => &[JamEffect::GreaterHeal, JamEffect::CureAll],
        }
    }

    fn calculate_effects(effects: &[Self]) -> Vec<JamEffect> {
        let mut seen = HashMap::new();

        for effect in effects.iter().flat_map(|i| i.effects()) {
            *seen.entry(effect).or_insert(0) += 1;
        }

        seen.into_iter()
            .filter(|(_k, v)| *v > 2)
            .map(|(k, _)| *k)
            .collect()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum JamEffect {
    NightVision,
    SuperHumanStrength,
    Poison,
    Hunger,
    GreaterHeal,
    CureAll,
    Speed,
    Flight,
    HideousLaughter,
}

impl JamEffect {
    fn all() -> &'static [JamEffect] {
        &[
            JamEffect::NightVision,
            JamEffect::SuperHumanStrength,
            JamEffect::Poison,
            JamEffect::Hunger,
            JamEffect::GreaterHeal,
            JamEffect::CureAll,
            JamEffect::Speed,
            JamEffect::Flight,
            JamEffect::HideousLaughter,
        ]
    }

    fn asset_path(self) -> &'static str {
        match self {
            // JamEffect::NightVision => "sprites/night_vision.png",
            // JamEffect::SuperHumanStrength => "sprites/super_human_strength.png",
            // JamEffect::Poison => "sprites/poison.png",
            // JamEffect::Hunger => "sprites/hunger.png",
            // JamEffect::GreaterHeal => "sprites/greater_heal.png",
            // JamEffect::CureAll => "sprites/cure_all.png",
            // JamEffect::Speed => "sprites/speed.png",
            // JamEffect::Flight => "sprites/flight.png",
            // JamEffect::HideousLaughter => "sprites/hideous_laughter.png",
            JamEffect::NightVision => "sprites/jambook.png",
            JamEffect::SuperHumanStrength => "sprites/jambook.png",
            JamEffect::Poison => "sprites/jambook.png",
            JamEffect::Hunger => "sprites/jambook.png",
            JamEffect::GreaterHeal => "sprites/jambook.png",
            JamEffect::CureAll => "sprites/jambook.png",
            JamEffect::Speed => "sprites/jambook.png",
            JamEffect::Flight => "sprites/jambook.png",
            JamEffect::HideousLaughter => "sprites/jambook.png",
        }
    }

    fn asset_for(self, assets: &JamAssets) -> Handle<Texture> {
        assets.effects.get(&self).unwrap().clone()
    }

    fn name(self) -> &'static str {
        match self {
            JamEffect::NightVision => "Night vision",
            JamEffect::SuperHumanStrength => "Super human strength",
            JamEffect::Poison => "Poison",
            JamEffect::Hunger => "Hunger",
            JamEffect::GreaterHeal => "Greater heal",
            JamEffect::CureAll => "Cure all",
            JamEffect::Speed => "Speed",
            JamEffect::Flight => "Flight",
            JamEffect::HideousLaughter => "Hideous laughter",
        }
    }

    fn description(self) -> &'static str {
        match self {
            JamEffect::NightVision => "See, in the dark",
            JamEffect::SuperHumanStrength => "HULK! SMASH!",
            JamEffect::Poison => "You feel ill",
            JamEffect::Hunger => "I am very hungry, give me the butter",
            JamEffect::GreaterHeal => "Your wounds heal and your body feels light",
            JamEffect::CureAll => "You are granted temporary relief from the radiation poisoning",
            JamEffect::Speed => "Radiation mutates the cells in your body, you become faster",
            JamEffect::Flight => "Your body fils with energy, so much that you fly?",
            JamEffect::HideousLaughter => {
                "You perceive everything as hilariously funny and fall into a fit of laugher."
            }
        }
    }
}

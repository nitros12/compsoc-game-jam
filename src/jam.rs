use std::collections::HashMap;

use bevy::prelude::*;

pub struct JamPlugin;

impl Plugin for JamPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
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

struct JamAssets {
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

    fn asset_path(self) -> &'static str {
        match self {
            JamIngredient::Petrol => "sprites/petrol.png",
            JamIngredient::Urine => "sprites/urine.png",
            JamIngredient::GunPowder => "sprites/gun_powder.png",
            JamIngredient::BathWater => "sprites/bath_water.png",
            JamIngredient::AppleSeeds => "sprites/apple_seeds.png",
            JamIngredient::Strawberries => "sprites/strawberries.png",
            JamIngredient::Lemons => "sprites/lemons.png",
            JamIngredient::Damsons => "sprites/damsons.png",
            JamIngredient::HumanFlesh => "sprites/human_flesh.png",
            JamIngredient::MotorOil => "sprites/motor_oil.png",
            JamIngredient::Absinth => "sprites/absinth.png",
            JamIngredient::Bleach => "sprites/bleach.png",
            JamIngredient::Sand => "sprites/sand.png",
            JamIngredient::Sugar => "sprites/sugar.png",
            JamIngredient::Salt => "sprites/salt.png",
            JamIngredient::Sakura => "sprites/sakura.png",
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
            JamEffect::NightVision => "sprites/night_vision.png",
            JamEffect::SuperHumanStrength => "sprites/super_human_strength.png",
            JamEffect::Poison => "sprites/poison.png",
            JamEffect::Hunger => "sprites/hunger.png",
            JamEffect::GreaterHeal => "sprites/greater_heal.png",
            JamEffect::CureAll => "sprites/cure_all.png",
            JamEffect::Speed => "sprites/speed.png",
            JamEffect::Flight => "sprites/flight.png",
            JamEffect::HideousLaughter => "sprites/hideous_laughter.png",
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
                "You perceives everything as hilariously funny and falls into fits of laugher."
            }
        }
    }
}

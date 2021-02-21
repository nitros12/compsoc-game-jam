use bevy::prelude::*;
use rand::seq::SliceRandom;

pub struct StoryGenPlugin;

static PHRASES: &[&[&str]] = &[
/*Intro*/    &["HUNGER I was scavenging for food when", "The other day, ", "In a firefight, ", "Before the war, "],
/*Villain*/  &["STRENGTH a raider far stronger than me", "a rival gang ", "ANTI-VENOM a mutated snake with potent venom", "an Old War soldier ", "an enemy fuel convoy", "CURE DISEASE a feral dog, riddled with diseases, "],
/*Adjective*/&["angrily ", "furiously ", "violently ", "suddenly "],
/*Action*/   &["COAGULANT stabbed ", "robbed ", "destroyed ", "hunted ", "shot at "],
/*Hero*/     &["my raiding party ", "me ", "my war-dog ", "SPEED my armoured truck, leaving me slow, ", "HUNGER my food supplies "],
/*joining*/  &["whilst I was ", "when I was ", "after I was caught", "for "],
/*Action*/   &["INVISIBILTY trying to steal ", "destroying ", "SPEED escaping with ", "running over ", "gambling away ", "POISON poisoning "],
/*belonging*/&["their water supply, ", "their supplies, ", "their credits, ", "their jam, ", "FLAMMABLE their fuel, ", "their Old World relics, ", "FLIGHT their pre-war iron bird"],
/*belonging*/&["so we ", "so I ", "and then I ", "and then we "],
/*belonging*/&["engaged them in hand to hand combat, ", "began shooting at them, ", "turned and ran away, ", "offered them a truce, ", "told them to surrender, "],
/*belonging*/&["but then ", "unfortunately this was interrupted when ", "before this could happen ", "suddenly, out of nowhere "],
/*belonging*/&["a huge explosion went off, which caused ", "a passionate glance was exchanged, which caused ", "ANTI-VENOM a poisoned trap clamped on my leg , causing ", "a severe gust of rad-wind tore through the valley, causing ", "STRENGTH my body became suddenly weak, causing"],
/*belonging*/&["COAGULANT my leg to fall off. ", "CURE my raid members to become violently sick. ", "FLAMMABLE my matches to get wet. ", "NIGHTVISION everything to go dark. "]
];


impl Plugin for StoryGenPlugin {
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_system(setup.system())
            //.add_system(generate_story.system())
    ;}
}

fn setup (
    commands: &mut Commands,
) {
    let mut story = String::from("");
    for x in 0..13
    {
        let var1 = PHRASES[x].choose(&mut rand::thread_rng()).unwrap().to_string();
        &story.push_str(&var1);
    }
    &story.push_str("As you can tell, I am in deperate need of assistance, do you have any jam that could help me ensure this doesn't happen again?");
    println!("{}", story);
}

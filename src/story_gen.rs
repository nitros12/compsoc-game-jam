use bevy::prelude::*;
use rand::seq::SliceRandom;

pub struct StoryGenPlugin;

static PHRASES: &[&[&str]] = &[
/*Intro*/    &["I was scavenging when", "The other day", "In a firefight ", "Before the war"],
/*Villain*/  &["a raider", "a rival gang", "a mutated beaver", "an Old War soldier", "a wasteland spider", "a feral dog"],
/*Adjective*/&["angrily", "furiously", "sadly", "violently", "suddenly"],
/*Action*/   &["stabbed", "robbed", "declared war on", "hunted", "shot at"],
/*Hero*/     &["my raiding party", "me", "my war-dog", "my armoured truck", "my supply cache"],
/*joining*/  &["before", "when", "after", "for"],
/*Action*/   &["stealing", "destroying", "escaping with", "running over", "gambling away", "poisoning"],
/*belonging*/&["our water supply", "our supplies", "my credits", "my jam", "our fuel", "my Old World relics"]
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
    let var1 = PHRASES[0].choose(&mut rand::thread_rng()).unwrap();
    let var2 = PHRASES[1].choose(&mut rand::thread_rng()).unwrap();
    let var3 = PHRASES[2].choose(&mut rand::thread_rng()).unwrap();
    let var4 = PHRASES[3].choose(&mut rand::thread_rng()).unwrap();
    let var5 = PHRASES[4].choose(&mut rand::thread_rng()).unwrap();
    let var6 = PHRASES[5].choose(&mut rand::thread_rng()).unwrap();
    let var7 = PHRASES[6].choose(&mut rand::thread_rng()).unwrap();
    let var8 = PHRASES[7].choose(&mut rand::thread_rng()).unwrap();
    println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", var1, var2, var3, var4, var5, var6, var7, var8);
}

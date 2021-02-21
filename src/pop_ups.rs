use bevy::prelude::*;
use crate::shop_scene;

pub struct PopUpsPlugin;

impl Plugin for StoryGenPlugin {
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_system(setup.system())
            //.add_system(generate_story.system())
    ;}
}

use bevy::prelude::*;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        println!("TestPlugin::build");
    }
}
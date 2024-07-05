use crate::app::*;
use crate::game_parameters::*;
use bevy::prelude::*;
mod app;
mod game_parameters;

fn main() {
    let mut app = create_app(create_default_game_parameters());
    let add_camera_fun = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fun);

    assert!(!app.is_plugin_added::<AssetPlugin>());
    app.add_plugins(DefaultPlugins);
    assert!(app.is_plugin_added::<AssetPlugin>());

    app.run();
}

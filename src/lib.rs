mod scene_handler;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use scene_handler::SceneHandlerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SceneHandlerPlugin,
        ));
        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
use bevy::prelude::*;

pub struct SceneHandlerPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for SceneHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_camera);
    }
}

fn setup_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        MainCamera,
    ));
}
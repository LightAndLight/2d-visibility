pub mod agent;
pub mod controls;
pub mod movement;
pub mod player;
pub mod sector;
pub mod sight;

use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
};

fn setup(mut commands: Commands) {
    trace!("setup");

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::WHITE),
        },
        ..default()
    });
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.edit_schedule(CoreSchedule::Main, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_plugin(player::PlayerPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(controls::ControlsPlugin)
        .add_plugin(agent::AgentPlugin)
        .add_plugin(sight::SightPlugin)
        .add_startup_system(setup)
        .insert_resource(sight::SightConfig {
            display_fields_of_view: false,
            display_view_angles: true,
        });
    }
}

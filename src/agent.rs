use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{controls, movement, player::Player};

#[derive(Bundle)]
pub struct AgentBundle {
    sprite: SpriteBundle,
    speed: movement::Speed,
    direction: movement::Direction,
}

impl AgentBundle {
    pub fn new() -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2 { x: 10.0, y: 10.0 }),
                    ..default()
                },
                ..default()
            },
            speed: movement::Speed { value: 100.0 },
            direction: movement::Direction { value: Vec2::ZERO },
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.sprite.transform = transform;
        self
    }
}

impl Default for AgentBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
pub struct Agent {}

fn behave(mut query: Query<&mut Agent>) {
    for agent in query.iter_mut() {}
}

fn setup(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let target = player_query.get_single().unwrap();

    commands.spawn(AgentBundle::default().with_transform(Transform::from_xyz(-10.0, -10.0, 0.0)));
    commands.spawn(AgentBundle::default().with_transform(Transform::from_xyz(10.0, -10.0, 0.0)));
    commands.spawn(AgentBundle::default().with_transform(Transform::from_xyz(-10.0, 10.0, 0.0)));
    commands.spawn(AgentBundle::default().with_transform(Transform::from_xyz(10.0, 10.0, 0.0)));
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct AgentSet;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.in_base_set(StartupSet::PostStartup))
            .add_system(behave.in_set(AgentSet))
            .configure_set(
                AgentSet
                    .before(controls::ControlsSet)
                    .after(movement::MovementSet),
            );
    }
}

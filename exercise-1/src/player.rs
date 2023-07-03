use bevy::prelude::*;

use crate::sight::Visible;

#[derive(Component)]
pub struct Player;

fn setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2 { x: 10.0, y: 10.0 }),
                ..default()
            },
            ..default()
        })
        .insert(crate::movement::Speed { value: 100.0 })
        .insert(crate::movement::Direction { value: Vec2::ZERO })
        .insert(crate::controls::Controlled)
        .insert(Player)
        .insert(Visible);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

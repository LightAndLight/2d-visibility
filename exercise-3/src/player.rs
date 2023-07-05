use bevy::prelude::*;

use crate::{
    controls::Controlled,
    light::LightSet,
    movement::{self, MovementSet, Speed},
    sight::{CheckVisibility, Sighted, Visible},
};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite_bundle: SpriteBundle,
    speed: Speed,
    direction: movement::Direction,
    controlled: Controlled,
    sighted: Sighted,
    visible: Visible,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            player: Player,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2 { x: 10.0, y: 10.0 }),
                    ..default()
                },
                ..default()
            },
            speed: Speed { value: 100.0 },
            direction: movement::Direction { value: Vec2::ZERO },
            controlled: Controlled,
            sighted: Sighted,
            visible: Visible,
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self::new()
    }
}

fn object_visibility(
    check_visibility: CheckVisibility,
    player_query: Query<Entity, With<Player>>,
    mut visible_query: Query<
        (Entity, &mut bevy::render::view::Visibility),
        (With<Visible>, Without<Player>),
    >,
) {
    let player = player_query.get_single().unwrap();

    for (entity, mut visibility) in visible_query.iter_mut() {
        if check_visibility.sees(player, entity) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct PlayerSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSet.after(MovementSet).after(LightSet));

        app.add_system(object_visibility.in_set(PlayerSet));
    }
}

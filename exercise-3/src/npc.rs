use bevy::prelude::*;

use crate::{
    controls, light, movement,
    player::Player,
    sight::{CheckVisibility, Sighted, Visible},
};

#[derive(Component)]
pub struct Npc;

#[derive(Bundle)]
pub struct NpcBundle {
    npc: Npc,
    sprite: SpriteBundle,
    sighted: Sighted,
    visible: Visible,
}

impl NpcBundle {
    pub fn new() -> Self {
        Self {
            npc: Npc,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2 { x: 10.0, y: 10.0 }),
                    ..default()
                },
                ..default()
            },
            sighted: Sighted,
            visible: Visible,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.sprite.transform = transform;
        self
    }
}

impl Default for NpcBundle {
    fn default() -> Self {
        Self::new()
    }
}

fn see_player(
    check_visibility: CheckVisibility,
    player_query: Query<Entity, With<Player>>,
    mut sprite_query: Query<(Entity, &mut Sprite), (With<Npc>, With<Sighted>)>,
) {
    let player_entity = player_query.get_single().unwrap();

    for (npc_entity, mut npc_sprite) in sprite_query.iter_mut() {
        if check_visibility.sees(npc_entity, player_entity) {
            npc_sprite.color = Color::GREEN;
        } else {
            npc_sprite.color = Color::GRAY;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct NpcSet;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            NpcSet
                .before(controls::ControlsSet)
                .after(movement::MovementSet)
                .after(light::LightSet),
        );

        app.add_system(see_player.in_set(NpcSet));
    }
}

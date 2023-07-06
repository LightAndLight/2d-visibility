use bevy::prelude::*;

use crate::{
    controls::Controlled,
    light::{LightSet, PlayerShadow, SegmentShadow},
    movement::{self, MovementSet, Speed},
    sight::{Sighted, Visible},
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
    mut visible_entities: Query<
        (
            Entity,
            &GlobalTransform,
            &Sprite,
            &mut bevy::render::view::Visibility,
        ),
        (With<Visible>, Without<Player>),
    >,
    player_shadows: Query<(&PlayerShadow, &Children)>,
    segment_shadows: Query<&SegmentShadow>,
) {
    fn entity_in_player_shadows(
        entity: Entity,
        global_transform: &GlobalTransform,
        size: Vec2,
        player_shadows: &Query<(&PlayerShadow, &Children)>,
        segment_shadows: &Query<&SegmentShadow>,
    ) -> bool {
        player_shadows
            .iter()
            .any(|(player_shadow, player_shadow_children)| {
                entity != player_shadow.occluder
                    && player_shadow_children.iter().any(|player_shadow_child| {
                        if let Ok(segment_shadow) = segment_shadows.get(*player_shadow_child) {
                            [
                                Vec3 {
                                    x: -size.x / 2.0,
                                    y: size.y / 2.0,
                                    z: 0.0,
                                },
                                Vec3 {
                                    x: size.x / 2.0,
                                    y: size.y / 2.0,
                                    z: 0.0,
                                },
                                Vec3 {
                                    x: size.x / 2.0,
                                    y: -size.y / 2.0,
                                    z: 0.0,
                                },
                                Vec3 {
                                    x: -size.x / 2.0,
                                    y: -size.y / 2.0,
                                    z: 0.0,
                                },
                            ]
                            .into_iter()
                            .all(|location| {
                                segment_shadow
                                    .contains_point(&global_transform.transform_point(location))
                            })
                        } else {
                            false
                        }
                    })
            })
    }

    for (entity, global_transform, sprite, mut visibility) in visible_entities.iter_mut() {
        if entity_in_player_shadows(
            entity,
            global_transform,
            sprite.custom_size.unwrap(),
            &player_shadows,
            &segment_shadows,
        ) {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
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

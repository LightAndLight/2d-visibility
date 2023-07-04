use std::collections::HashSet;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{movement::MovementSet, player::Player, sight::Occluder};

#[derive(Component)]
struct PlayerRay {
    occluder: Entity,
    end: Vec3,
}

const LINE_THICKNESS: f32 = 2.0;

#[derive(Debug)]
struct Line {
    start: Vec3,
    end: Vec3,
}

impl From<Line> for Mesh {
    fn from(value: Line) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![value.start, value.end]);
        mesh.set_indices(Some(Indices::U32(vec![0, 1])));
        mesh
    }
}

fn add_player_rays(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    occluders: Query<(Entity, &Occluder), Added<Occluder>>,
) {
    let player_transform = player_query.get_single().unwrap();

    for (entity, occluder) in occluders.iter() {
        for segment in occluder.iter_segments() {
            debug!("drawing segment: {:?}", segment);

            commands.spawn((
                PlayerRay {
                    occluder: entity,
                    end: segment.0,
                },
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(
                            Line {
                                start: player_transform.translation,
                                end: segment.0,
                            }
                            .into(),
                        )
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    ..default()
                },
            ));

            commands.spawn((
                PlayerRay {
                    occluder: entity,
                    end: segment.1,
                },
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(
                            Line {
                                start: player_transform.translation,
                                end: segment.1,
                            }
                            .into(),
                        )
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    ..default()
                },
            ));
        }
    }
}

fn update_player_rays(
    mut meshes: ResMut<Assets<Mesh>>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut player_rays: Query<(&PlayerRay, &Mesh2dHandle)>,
) {
    let player_transform = player_query.get_single().unwrap();
    for (player_ray, mesh_handle) in player_rays.iter_mut() {
        let new_line = Line {
            start: player_transform.translation,
            end: player_ray.end,
        };
        trace!("moving player ray to {:?}", new_line);

        let mesh = meshes.get_mut(&mesh_handle.0).unwrap();
        *mesh = new_line.into();
    }
}

fn remove_player_rays(
    mut commands: Commands,
    player_rays: Query<(Entity, &PlayerRay)>,
    mut removed_occluders: RemovedComponents<Occluder>,
    mut removed_occluders_set: Local<HashSet<Entity>>,
) {
    removed_occluders_set.clear();
    removed_occluders_set.extend(removed_occluders.iter());

    for (entity, player_ray) in player_rays.iter() {
        if removed_occluders_set.contains(&player_ray.occluder) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_player_rays)
            .add_system(update_player_rays.after(MovementSet).run_if(
                |player_query: Query<Entity, (Changed<Transform>, With<Player>)>| {
                    !player_query.is_empty()
                },
            ))
            .add_system(remove_player_rays.in_base_set(CoreSet::PostUpdate));
    }
}

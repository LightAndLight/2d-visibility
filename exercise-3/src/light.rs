use std::collections::HashSet;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    movement::MovementSet,
    player::Player,
    sight::{Occluder, Segment},
};

/// Render shadow quad outlines and barycenters.
pub const DEBUG: bool = false;

#[derive(Component)]
struct PlayerRay {
    occluder: Entity,
    through: Vec3,
    end: Vec3,
}

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

/// Compute the closest point at which a ray will intersect the edge of the screen.
fn project_ray_to_window_edge(width: f32, height: f32, ray: Ray) -> f32 {
    assert!(ray.direction != Vec3::ZERO);
    assert!(-width / 2.0 <= ray.origin.x && ray.origin.x <= width / 2.0);
    assert!(-height / 2.0 <= ray.origin.y && ray.origin.y <= height / 2.0);

    let t_left_right = if ray.direction.x != 0.0 {
        Some(f32::max(
            (-width / 2.0 - ray.origin.x) / ray.direction.x,
            (width / 2.0 - ray.origin.x) / ray.direction.x,
        ))
    } else {
        None
    };

    let t_top_bottom = if ray.direction.x != 0.0 {
        Some(f32::max(
            (-height / 2.0 - ray.origin.y) / ray.direction.y,
            (height / 2.0 - ray.origin.y) / ray.direction.y,
        ))
    } else {
        None
    };

    let t = match (t_left_right, t_top_bottom) {
        (None, None) => unreachable!(),
        (None, Some(t)) => t,
        (Some(t), None) => t,
        (Some(t_left_right), Some(t_top_bottom)) => f32::max(t_left_right, t_top_bottom),
    };

    assert!(t >= 0.0);

    t
}

fn angle_ccw(barycentre: &Vec3, point: &Vec3) -> f32 {
    let v = *point - *barycentre;
    if v.y >= 0.0 {
        v.angle_between(Vec3::X)
    } else {
        std::f32::consts::TAU - v.angle_between(Vec3::X)
    }
}

#[test]
fn angle_ccw_test_1() {
    assert_eq!(angle_ccw(&Vec3::ZERO, &Vec3::X), 0.0);

    assert_eq!(
        angle_ccw(&Vec3::ZERO, &(Vec3::X + Vec3::Y)),
        std::f32::consts::FRAC_PI_4
    );

    assert!((angle_ccw(&Vec3::ZERO, &Vec3::Y) - std::f32::consts::FRAC_PI_2).abs() < 0.001);

    assert_eq!(
        angle_ccw(&Vec3::ZERO, &(-Vec3::X + Vec3::Y)),
        std::f32::consts::FRAC_PI_2 + std::f32::consts::FRAC_PI_4
    );

    assert_eq!(angle_ccw(&Vec3::ZERO, &-Vec3::X), std::f32::consts::PI);

    assert_eq!(
        angle_ccw(&Vec3::ZERO, &(-Vec3::X - Vec3::Y)),
        std::f32::consts::PI + std::f32::consts::FRAC_PI_4
    );

    assert_eq!(
        angle_ccw(&Vec3::ZERO, &-Vec3::Y),
        std::f32::consts::PI + std::f32::consts::FRAC_PI_2
    );

    assert_eq!(
        angle_ccw(&Vec3::ZERO, &(Vec3::X - Vec3::Y)),
        std::f32::consts::TAU - std::f32::consts::FRAC_PI_4
    );
}

#[derive(Debug)]
struct Quad(Vec3, Vec3, Vec3, Vec3);

impl From<Quad> for Mesh {
    fn from(value: Quad) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut values = [value.0, value.1, value.2, value.3];

        let barycentre = (value.0 + value.1 + value.2 + value.3) / 4.0;

        values.sort_by(|a, b| angle_ccw(&barycentre, a).total_cmp(&angle_ccw(&barycentre, b)));

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::from(values));
        mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 0, 2, 3])));
        mesh
    }
}

fn project_points_to_window_edge(
    width: f32,
    height: f32,
    from_point: &Vec3,
    through_point: &Vec3,
) -> Vec3 {
    let ray = Ray {
        origin: *from_point,
        direction: *through_point - *from_point,
    };
    let t = project_ray_to_window_edge(width, height, ray);
    ray.get_point(t)
}

#[derive(Component)]
struct PlayerShadow {
    occluder: Entity,
}

#[derive(Component)]
struct SegmentShadow {
    segment: Segment,
}

#[derive(Component)]
struct ShadowBarycentre;

fn add_player_shadows(
    windows: Query<&Window>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    occluders: Query<(Entity, &Occluder), Added<Occluder>>,
) {
    if !occluders.is_empty() {
        let window = windows.get_single().unwrap();

        let player_transform = player_query.get_single().unwrap();

        let material_red = materials.add(ColorMaterial::from(Color::RED));
        let material_dark_gray = materials.add(ColorMaterial::from(Color::DARK_GRAY));

        for (entity, occluder) in occluders.iter() {
            let player_shadow_entity = commands
                .spawn((PlayerShadow { occluder: entity }, SpatialBundle::default()))
                .id();

            for segment in occluder.iter_segments() {
                let ray_1_end = project_points_to_window_edge(
                    window.width(),
                    window.height(),
                    &player_transform.translation,
                    &segment.0,
                );

                let ray_2_end = project_points_to_window_edge(
                    window.width(),
                    window.height(),
                    &player_transform.translation,
                    &segment.1,
                );

                let v1 = segment.0;
                let v2 = ray_1_end;
                let v3 = ray_2_end;
                let v4 = segment.1;

                commands
                    .entity(player_shadow_entity)
                    .with_children(|parent| {
                        parent
                            .spawn((
                                SegmentShadow { segment },
                                MaterialMesh2dBundle {
                                    mesh: meshes.add(Quad(v1, v2, v3, v4).into()).into(),
                                    material: material_dark_gray.clone(),
                                    ..default()
                                },
                            ))
                            .with_children(|parent| {
                                if DEBUG {
                                    parent.spawn((
                                        ShadowBarycentre,
                                        MaterialMesh2dBundle {
                                            mesh: meshes
                                                .add(
                                                    shape::Circle {
                                                        radius: 5.0,
                                                        ..default()
                                                    }
                                                    .into(),
                                                )
                                                .into(),
                                            material: material_red.clone(),
                                            transform: Transform::from_translation(
                                                (v1 + v2 + v3 + v4) / 4.0 + Vec3::Z,
                                            ),
                                            ..default()
                                        },
                                    ));
                                }
                            });
                    });

                if DEBUG {
                    commands.spawn((
                        PlayerRay {
                            occluder: entity,
                            through: segment.0,
                            end: ray_1_end,
                        },
                        MaterialMesh2dBundle {
                            mesh: meshes
                                .add(
                                    Line {
                                        start: player_transform.translation,
                                        end: ray_1_end,
                                    }
                                    .into(),
                                )
                                .into(),
                            material: material_red.clone(),
                            transform: Transform::from_translation(Vec3::Z),
                            ..default()
                        },
                    ));

                    commands.spawn((
                        PlayerRay {
                            occluder: entity,
                            through: segment.1,
                            end: ray_2_end,
                        },
                        MaterialMesh2dBundle {
                            mesh: meshes
                                .add(
                                    Line {
                                        start: player_transform.translation,
                                        end: ray_2_end,
                                    }
                                    .into(),
                                )
                                .into(),
                            material: material_red.clone(),
                            transform: Transform::from_translation(Vec3::Z),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

fn update_player_shadows(
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut player_rays: Query<(&mut PlayerRay, &Mesh2dHandle)>,
    player_shadows: Query<&PlayerShadow>,
    segment_shadows: Query<(&Parent, &SegmentShadow, &Mesh2dHandle, &Children)>,
    mut shadow_barycentres: Query<(&ShadowBarycentre, &mut Transform), Without<Player>>,
) {
    let window = windows.get_single().unwrap();

    let player_transform = player_query.get_single().unwrap();

    for (mut player_ray, mesh_handle) in player_rays.iter_mut() {
        let end = project_points_to_window_edge(
            window.width(),
            window.height(),
            &player_transform.translation,
            &player_ray.through,
        );

        let new_line = Line {
            start: player_transform.translation,
            end,
        };

        *meshes.get_mut(&mesh_handle.0).unwrap() = new_line.into();
        player_ray.end = end;
    }

    for (segment_shadow_parent, segment_shadow, segment_shadow_mesh_handle, children) in
        segment_shadows.iter()
    {
        if player_shadows.contains(segment_shadow_parent.get()) {
            let ray_1_end = project_points_to_window_edge(
                window.width(),
                window.height(),
                &player_transform.translation,
                &segment_shadow.segment.0,
            );

            let ray_2_end = project_points_to_window_edge(
                window.width(),
                window.height(),
                &player_transform.translation,
                &segment_shadow.segment.1,
            );

            let v1 = segment_shadow.segment.0;
            let v2 = ray_1_end;
            let v3 = ray_2_end;
            let v4 = segment_shadow.segment.1;

            *meshes.get_mut(&segment_shadow_mesh_handle.0).unwrap() = Quad(v1, v2, v3, v4).into();

            for child in children {
                let (ShadowBarycentre, mut transform) = shadow_barycentres.get_mut(*child).unwrap();
                transform.translation = (v1 + v2 + v3 + v4) / 4.0 + Vec3::Z;
            }
        }
    }
}

fn remove_player_shadows(
    mut commands: Commands,
    player_rays: Query<(Entity, &PlayerRay)>,
    player_shadows: Query<(Entity, &PlayerShadow)>,
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

    for (entity, player_shadow) in player_shadows.iter() {
        if removed_occluders_set.contains(&player_shadow.occluder) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LightSet;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(LightSet.after(MovementSet));

        app.add_system(
            add_player_shadows
                .in_set(LightSet)
                .after(update_player_shadows),
        )
        .add_system(update_player_shadows.in_set(LightSet).run_if(
            |player_query: Query<Entity, (Changed<Transform>, With<Player>)>| {
                !player_query.is_empty()
            },
        ));

        app.add_system(remove_player_shadows.in_base_set(CoreSet::PostUpdate));
    }
}

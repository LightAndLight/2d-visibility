use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::sector;

#[derive(Component)]
pub struct Sighted {
    pub fov: f32,
    pub distance: f32,
}

#[derive(Component)]
pub struct Visible;

fn visibility(
    mut _sighted_query: Query<&mut Sighted>,
    _visible_query: Query<Entity, With<Visible>>,
) {
}

#[derive(Component)]
pub struct FieldOfView;

fn enable_field_of_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Sighted)>,
) {
    for (entity, sighted) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                FieldOfView,
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(
                            sector::Sector {
                                radius: sighted.distance,
                                angle: sighted.fov,
                            }
                            .into(),
                        )
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
                    ..default()
                },
            ));
        });
    }
}

fn disable_field_of_view(mut commands: Commands, query: Query<Entity, With<FieldOfView>>) {
    for entity in query.iter() {
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove_parent();
        entity_commands.despawn_recursive();
    }
}

#[derive(Component)]
pub struct ViewAngle;

fn enable_view_angle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Sighted)>,
) {
    for (entity, sighted) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn((ViewAngle, SpatialBundle::default()))
                .with_children(|parent| {
                    parent.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(
                                shape::Quad {
                                    size: Vec2 { x: 1000.0, y: 1.0 },
                                    ..default()
                                }
                                .into(),
                            )
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
                        transform: {
                            let mut t = Transform::from_translation(Vec3 {
                                x: 500.0,
                                y: 0.0,
                                z: 0.0,
                            });
                            t.rotate_around(Vec3::ZERO, Quat::from_rotation_z(sighted.fov / 2.0));
                            t
                        },
                        ..default()
                    });
                    parent.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(
                                shape::Quad {
                                    size: Vec2 { x: 1000.0, y: 1.0 },
                                    ..default()
                                }
                                .into(),
                            )
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
                        transform: {
                            let mut t = Transform::from_translation(Vec3 {
                                x: 500.0,
                                y: 0.0,
                                z: 0.0,
                            });
                            t.rotate_around(Vec3::ZERO, Quat::from_rotation_z(-sighted.fov / 2.0));
                            t
                        },
                        ..default()
                    });
                });
        });
    }
}

fn disable_view_angle(mut commands: Commands, query: Query<Entity, With<ViewAngle>>) {
    for entity in query.iter() {
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove_parent();
        entity_commands.despawn_recursive();
    }
}

#[derive(Resource)]
pub struct SightConfig {
    pub display_fields_of_view: bool,
    pub display_view_angles: bool,
}

pub struct SightPlugin;

impl Plugin for SightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(visibility);

        app.add_system(
            enable_field_of_view
                .run_if(resource_exists_and_changed::<SightConfig>().and_then(
                    |sight_config: Res<SightConfig>| sight_config.display_fields_of_view,
                )),
        )
        .add_system(disable_field_of_view.run_if(
            resource_removed::<SightConfig>().or_else(
                resource_exists_and_changed::<SightConfig>().and_then(
                    |sight_config: Res<SightConfig>| !sight_config.display_fields_of_view,
                ),
            ),
        ));

        app.add_system(
            enable_view_angle.run_if(
                resource_exists_and_changed::<SightConfig>()
                    .and_then(|sight_config: Res<SightConfig>| sight_config.display_view_angles),
            ),
        )
        .add_system(
            disable_view_angle.run_if(
                resource_removed::<SightConfig>().or_else(
                    resource_exists_and_changed::<SightConfig>().and_then(
                        |sight_config: Res<SightConfig>| !sight_config.display_view_angles,
                    ),
                ),
            ),
        );
    }
}

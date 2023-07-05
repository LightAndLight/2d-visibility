use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(Component)]
pub struct Sighted;

#[derive(Component)]
pub struct Visible;

#[derive(Component)]
pub struct Occluder {
    pub top_left: Vec3,
    pub bottom_right: Vec3,
}

impl Occluder {
    pub fn iter_segments(&self) -> impl Iterator<Item = Segment> + '_ {
        #[derive(Clone, Copy)]
        enum Side {
            Top,
            Bottom,
            Left,
            Right,
        }

        let mut next_side = Some(Side::Top);
        std::iter::from_fn(move || {
            next_side.map(|side| match side {
                Side::Top => {
                    next_side = Some(Side::Bottom);
                    Segment(
                        self.top_left,
                        Vec3 {
                            x: self.bottom_right.x,
                            ..self.top_left
                        },
                    )
                }
                Side::Bottom => {
                    next_side = Some(Side::Left);
                    Segment(
                        Vec3 {
                            x: self.top_left.x,
                            ..self.bottom_right
                        },
                        self.bottom_right,
                    )
                }
                Side::Left => {
                    next_side = Some(Side::Right);
                    Segment(
                        self.top_left,
                        Vec3 {
                            y: self.bottom_right.y,
                            ..self.top_left
                        },
                    )
                }
                Side::Right => {
                    next_side = None;
                    Segment(
                        Vec3 {
                            y: self.top_left.y,
                            ..self.bottom_right
                        },
                        self.bottom_right,
                    )
                }
            })
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Segment(pub Vec3, pub Vec3);

/*
The b `a` consists of all the points along `f(s) = a.0 + s * (a.1 - a.0) for 0 <= s <= 1`.

The b `b` consists of all the points along `g(t) = b.0 + t * (b.1 - b.0) for 0 <= t <= 1`.

They intersect when `a.0 + s * (a.1 - a.0) = b.0 + t * (b.1 - b.0)` has a solution
where `0 <= s <= 1` and `0 <= t <= 1`.

```
a.0 + s * (a.1 - a.0) = b.0 + t * (b.1 - b.0)
```

decomposes into:

```
a.0.x + s * (a.1 - a.0).x = b.0.x + t * (b.1 - b.0).x
a.0.y + s * (a.1 - a.0).y = b.0.y + t * (b.1 - b.0).y
```

```
a.0.x + s * (a.1.x - a.0.x) = b.0.x + t * (b.1.x - b.0.x)
a.0.y + s * (a.1.y - a.0.y) = b.0.y + t * (b.1.y - b.0.y)
```

Solving the system of equations gives me:

```
s =
  ((b.1.y - b.0.y) * (a.0.x - b.0.x) - (b.1.x - b.0.x) * (a.0.y - b.0.y))
  /
  ((a.1.y - a.0.y) * (b.1.x - b.0.x) - (a.1.x - a.0.x) * (b.1.y - b.0.y))
```

and

```
t =
  ((a.1.y - a.0.y) * (b.0.x - a.0.x) + (a.1.x - a.0.x) * (a.0.y - b.0.y))
  /
  ((a.1.x - a.0.x) * (b.1.y - b.0.y) - (a.1.y - a.0.y) * (b.1.x - b.0.x))
```

See also: https://en.wikipedia.org/wiki/Intersection_(geometry)#Two_line_segments
*/
fn segment_intersects_segment(a: &Segment, b: &Segment) -> bool {
    let s_numerator = (b.1.y - b.0.y) * (a.0.x - b.0.x) - (b.1.x - b.0.x) * (a.0.y - b.0.y);
    let s_denominator = (a.1 - a.0).y * (b.1.x - b.0.x) - (a.1 - a.0).x * (b.1.y - b.0.y);

    if s_denominator == 0.0 {
        return false;
    }

    let t_numerator = (a.1 - a.0).y * (b.0.x - a.0.x) + (a.1 - a.0).x * (a.0.y - b.0.y);
    let t_denominator = (a.1 - a.0).x * (b.1.y - b.0.y) - (a.1 - a.0).y * (b.1.x - b.0.x);

    if t_denominator == 0.0 {
        return false;
    }

    let s = s_numerator / s_denominator;
    let t = t_numerator / t_denominator;

    (0.0..=1.0).contains(&s) && (0.0..=1.0).contains(&t)
}

#[test]
fn segment_intersects_segment_test_1() {
    // collinear
    let a = Segment(Vec3::ZERO, 1.0 * Vec3::X);
    let b = Segment(Vec3::ZERO, 1.0 * Vec3::X);

    assert!(!segment_intersects_segment(&a, &b))
}

#[test]
fn segment_intersects_segment_test_2() {
    // parallel
    let a = Segment(Vec3::ZERO, 1.0 * Vec3::X);
    let b = Segment(1.0 * Vec3::Y, 1.0 * Vec3::Y + 1.0 * Vec3::X);

    assert!(!segment_intersects_segment(&a, &b))
}

#[test]
fn segment_intersects_segment_test_3() {
    // orthogonal, non-intersecting
    let a = Segment(-1.0 * Vec3::X, 1.0 * Vec3::X);
    let b = Segment(1.0 * Vec3::Y, 2.0 * Vec3::Y);

    assert!(!segment_intersects_segment(&a, &b))
}

#[test]
fn segment_intersects_segment_test_4() {
    // orthogonal, intersecting
    let a = Segment(-1.0 * Vec3::X, 1.0 * Vec3::X);
    let b = Segment(-1.0 * Vec3::Y, 1.0 * Vec3::Y);

    assert!(segment_intersects_segment(&a, &b))
}

fn segment_intersects_occluder(segment: &Segment, occluder: &Occluder) -> bool {
    let top = Segment(
        occluder.top_left,
        Vec3 {
            x: occluder.bottom_right.x,
            ..occluder.top_left
        },
    );
    if segment_intersects_segment(segment, &top) {
        return true;
    }

    let left = Segment(
        occluder.top_left,
        Vec3 {
            y: occluder.bottom_right.y,
            ..occluder.top_left
        },
    );
    if segment_intersects_segment(segment, &left) {
        return true;
    }

    let bottom = Segment(
        Vec3 {
            x: occluder.top_left.x,
            ..occluder.bottom_right
        },
        occluder.bottom_right,
    );
    if segment_intersects_segment(segment, &bottom) {
        return true;
    }

    let right = Segment(
        Vec3 {
            y: occluder.top_left.y,
            ..occluder.bottom_right
        },
        occluder.bottom_right,
    );
    if segment_intersects_segment(segment, &right) {
        return true;
    }

    false
}

#[derive(SystemParam)]
pub struct CheckVisibility<'w, 's> {
    sighteds: Query<'w, 's, &'static Sighted>,
    visibles: Query<'w, 's, &'static Visible>,
    occluders: Query<'w, 's, &'static Occluder>,
    transforms: Query<'w, 's, &'static Transform>,
}

impl<'w, 's> CheckVisibility<'w, 's> {
    pub fn sees(&self, viewer: Entity, viewee: Entity) -> bool {
        assert!(self.sighteds.get(viewer).is_ok(), "viewer is not Sighted");

        match self.visibles.get(viewee) {
            Err(_) => false,
            Ok(_) => {
                let viewer_transform = self.transforms.get(viewer).unwrap();
                let viewee_transform = self.transforms.get(viewee).unwrap();

                let line_of_sight =
                    Segment(viewer_transform.translation, viewee_transform.translation);

                !self
                    .occluders
                    .iter()
                    .any(|occluder| segment_intersects_occluder(&line_of_sight, occluder))
            }
        }
    }
}

#[derive(Component)]
struct DisplayOccluder;

fn display_occluders(mut commands: Commands, query: Query<(Entity, &Occluder)>) {
    for (entity, occluder) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            let width = occluder.bottom_right.x - occluder.top_left.x;
            let height = occluder.top_left.y - occluder.bottom_right.y;

            let color = Color::ORANGE;
            let thickness = 2.0;

            parent.spawn((
                DisplayOccluder,
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2 {
                            x: thickness,
                            y: height + 2.0 * thickness,
                        }),
                        ..default()
                    },
                    transform: Transform::from_xyz(-width / 2.0 - thickness / 2.0, 0.0, 0.0),
                    ..default()
                },
            ));

            parent.spawn((
                DisplayOccluder,
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2 {
                            x: thickness,
                            y: height + 2.0 * thickness,
                        }),
                        ..default()
                    },
                    transform: Transform::from_xyz(width / 2.0 + thickness / 2.0, 0.0, 0.0),
                    ..default()
                },
            ));

            parent.spawn((
                DisplayOccluder,
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2 {
                            x: width + 2.0 * thickness,
                            y: thickness,
                        }),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -height / 2.0 - thickness / 2.0, 0.0),
                    ..default()
                },
            ));

            parent.spawn((
                DisplayOccluder,
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2 {
                            x: width + 2.0 * thickness,
                            y: thickness,
                        }),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, height / 2.0 + thickness / 2.0, 0.0),
                    ..default()
                },
            ));
        });
    }
}

fn hide_occluders(mut commands: Commands, query: Query<Entity, With<DisplayOccluder>>) {
    for entity in query.iter() {
        let mut entity_commands = commands.entity(entity);
        entity_commands.remove_parent();
        entity_commands.despawn_recursive();
    }
}

#[derive(Default, Resource)]
pub struct SightConfig {
    pub display_occluders: bool,
}

pub struct SightPlugin;

impl Plugin for SightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SightConfig>();

        app.add_system(
            display_occluders.run_if(
                resource_changed::<SightConfig>()
                    .and_then(|sight_config: Res<SightConfig>| sight_config.display_occluders),
            ),
        )
        .add_system(
            hide_occluders.run_if(
                resource_removed::<SightConfig>()
                    .or_else(resource_changed::<SightConfig>().and_then(
                        |sight_config: Res<SightConfig>| !sight_config.display_occluders,
                    )),
            ),
        );
    }
}

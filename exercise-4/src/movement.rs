use bevy::prelude::*;

#[derive(Component)]
pub struct Speed {
    pub value: f32,
}

#[derive(Component)]
pub struct Direction {
    pub value: Vec2,
}

impl From<Vec2> for Direction {
    fn from(value: Vec2) -> Self {
        Direction { value }
    }
}

fn update_position(mut query: Query<(&mut Transform, &Speed, &Direction)>, time: Res<Time>) {
    for (mut transform, speed, direction) in query.iter_mut() {
        if direction.value != Vec2::ZERO {
            let normalized_direction = direction.value.normalize_or_zero();
            transform.translation.x += speed.value * normalized_direction.x * time.delta_seconds();
            transform.translation.y += speed.value * normalized_direction.y * time.delta_seconds();
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct MovementSet;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_position.in_set(MovementSet));
    }
}

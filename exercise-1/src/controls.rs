use bevy::prelude::*;

use crate::movement;

#[derive(Component)]
pub struct Controlled;

fn set_direction(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut movement::Direction, With<Controlled>>,
) {
    if input.just_pressed(KeyCode::W) {
        for mut direction in query.iter_mut() {
            direction.value += Vec2::Y;
        }
    }

    if input.just_released(KeyCode::W) {
        for mut direction in query.iter_mut() {
            direction.value -= Vec2::Y;
        }
    }

    if input.just_pressed(KeyCode::A) {
        for mut direction in query.iter_mut() {
            direction.value -= Vec2::X;
        }
    }

    if input.just_released(KeyCode::A) {
        for mut direction in query.iter_mut() {
            direction.value += Vec2::X;
        }
    }

    if input.just_pressed(KeyCode::S) {
        for mut direction in query.iter_mut() {
            direction.value -= Vec2::Y;
        }
    }

    if input.just_released(KeyCode::S) {
        for mut direction in query.iter_mut() {
            direction.value += Vec2::Y;
        }
    }

    if input.just_pressed(KeyCode::D) {
        for mut direction in query.iter_mut() {
            direction.value += Vec2::X;
        }
    }

    if input.just_released(KeyCode::D) {
        for mut direction in query.iter_mut() {
            direction.value -= Vec2::X;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct ControlsSet;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(set_direction.in_set(ControlsSet));
    }
}

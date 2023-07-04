use bevy::prelude::*;

use crate::{
    movement::MovementSet,
    sight::{CheckVisibility, Sighted, Visible},
};

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
        .insert(Sighted)
        .insert(Visible);
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);

        app.add_system(object_visibility.after(MovementSet));
    }
}

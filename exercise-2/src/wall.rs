use bevy::prelude::*;

use crate::sight::Occluder;

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub sprite: SpriteBundle,
    pub occluder: Occluder,
}

impl WallBundle {
    pub fn new() -> Self {
        Self {
            wall: Wall,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2 { x: 10.0, y: 100.0 }),
                    ..default()
                },
                ..default()
            },
            occluder: Occluder {
                top_left: Vec3 {
                    x: -5.0,
                    y: 50.0,
                    z: 0.0,
                },
                bottom_right: Vec3 {
                    x: 5.0,
                    y: -50.0,
                    z: 0.0,
                },
            },
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.sprite.transform = transform * self.sprite.transform;
        self.occluder.top_left = transform * self.occluder.top_left;
        self.occluder.bottom_right = transform * self.occluder.bottom_right;
        self
    }
}

impl Default for WallBundle {
    fn default() -> Self {
        Self::new()
    }
}

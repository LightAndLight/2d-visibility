use bevy::prelude::*;

use crate::sight::{Occluder, Visible};

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub sprite_bundle: SpriteBundle,
    pub occluder: Occluder,
    pub visible: Visible,
}

impl WallBundle {
    pub fn new() -> Self {
        Self {
            wall: Wall,
            sprite_bundle: SpriteBundle {
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
            visible: Visible,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.sprite_bundle.transform = transform * self.sprite_bundle.transform;
        self.occluder.top_left = transform * self.occluder.top_left;
        self.occluder.bottom_right = transform * self.occluder.bottom_right;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.sprite_bundle.sprite.custom_size = Some(size);
        self.occluder.top_left = self.sprite_bundle.transform.translation
            - Vec3 {
                x: size.x / 2.0,
                y: -size.y / 2.0,
                z: 0.0,
            };
        self.occluder.bottom_right = self.sprite_bundle.transform.translation
            + Vec3 {
                x: size.x / 2.0,
                y: -size.y / 2.0,
                z: 0.0,
            };
        self
    }
}

impl Default for WallBundle {
    fn default() -> Self {
        Self::new()
    }
}

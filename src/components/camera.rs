use amethyst::core::{Parent, Transform};
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};

use super::{LEVEL_HEIGHT, LEVEL_WIDTH, PIXEL_PER_CASE};

pub const CAMERA_WIDTH: f32 = PIXEL_PER_CASE * LEVEL_WIDTH as f32;
pub const CAMERA_HEIGHT: f32 = PIXEL_PER_CASE * LEVEL_HEIGHT as f32;

pub fn initialise_camera(parent: Entity, world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_z(2.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            CAMERA_WIDTH,
            0.0,
            CAMERA_HEIGHT,
        )))
        .with(transform)
        .with(Parent { entity: parent })
        .build()
}

use amethyst::prelude::*;
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::Entity;
use amethyst::renderer::{Camera, Projection};

use crate::assets::GameAssets;

pub const PIXEL_PER_CASE: f32 = 32.0;
pub const LEVEL_WIDTH: usize = 10;
pub const LEVEL_HEIGHT: usize = 10;
pub const CAMERA_WIDTH: f32 = PIXEL_PER_CASE * LEVEL_WIDTH as f32;
pub const CAMERA_HEIGHT: f32 = PIXEL_PER_CASE * LEVEL_HEIGHT as f32;

pub struct LevelState {
    assets: GameAssets,
}

impl LevelState {
    pub fn new(assets: GameAssets) -> LevelState {
        LevelState {
            assets,
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LevelState {

    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        initialise_camera(world);

        // Create player
        world.create_entity()
            .with(GlobalTransform::new())
            .with(Transform::default())
            .with(self.assets.character_sprite())
            .build();
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world);
        Trans::None
    }
}

fn initialise_camera(world: &mut World) -> Entity {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            CAMERA_WIDTH,
            CAMERA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build()
}

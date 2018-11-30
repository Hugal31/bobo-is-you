use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::Entity;
use amethyst::input::is_close_requested;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};

use crate::assets::*;
use crate::components::*;

pub const CAMERA_WIDTH: f32 = PIXEL_PER_CASE * LEVEL_WIDTH as f32;
pub const CAMERA_HEIGHT: f32 = PIXEL_PER_CASE * LEVEL_HEIGHT as f32;

pub struct LevelState {
    assets: GameAssets,
}

impl LevelState {
    pub fn new(assets: GameAssets) -> LevelState {
        LevelState { assets }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LevelState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        initialise_camera(world);

        // Create player
        world
            .create_entity()
            .with(GlobalTransform::new())
            .with(Transform::default())
            .with(CellCoordinate::default())
            .with(Named::Bobo)
            .with(self.assets.entity_sprite(ENTITY_SPRITE_BOBO))
            .build();

        world
            .create_entity()
            .with(GlobalTransform::new())
            .with(Transform::default())
            .with(CellCoordinate::new(4, 4))
            .with(Named::Wall)
            .with(self.assets.entity_sprite(ENTITY_SPRITE_WALL))
            .build();

        world.create_entity()
            .with(GlobalTransform::new())
            .with(Transform::default())
            .with(CellCoordinate::new(2, 2))
            .with(self.assets.entity_sprite(ENTITY_SPRITE_INST_BOBO))
            .build();

        world.create_entity()
            .with(GlobalTransform::new())
            .with(Transform::default())
            .with(CellCoordinate::new(3, 2))
            .with(self.assets.entity_sprite(ENTITY_SPRITE_INST_IS))
            .build();


        world.create_entity()
            .with(GlobalTransform::new())
            .with(Transform::default())
            .with(CellCoordinate::new(4, 2))
            .with(self.assets.entity_sprite(ENTITY_SPRITE_INST_YOU))
            .build();

        world.create_entity()
            .with(GlobalTransform::new())
            .with(Transform::default())
            .with(CellCoordinate::new(3, 1))
            .with(self.assets.entity_sprite(ENTITY_SPRITE_INST_WALL))
            .build();


        world.create_entity()
            .with(GlobalTransform::new())
            .with(Transform::default())
            .with(CellCoordinate::new(3, 3))
            .with(self.assets.entity_sprite(ENTITY_SPRITE_INST_STOP))
            .build();
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world);
        Trans::None
    }
}

fn initialise_camera(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            CAMERA_WIDTH,
            CAMERA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)),
        ))
        .build()
}

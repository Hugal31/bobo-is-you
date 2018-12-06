use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::{GlobalTransform, Parent};
use amethyst::ecs::Entity;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};
use amethyst::winit::VirtualKeyCode;

use crate::components::*;
use crate::events::*;

pub const CAMERA_WIDTH: f32 = PIXEL_PER_CASE * LEVEL_WIDTH as f32;
pub const CAMERA_HEIGHT: f32 = PIXEL_PER_CASE * LEVEL_HEIGHT as f32;

pub struct LevelState {
    level_entity: Entity,
}

impl LevelState {
    pub fn new(level_entity: Entity) -> LevelState {
        LevelState { level_entity }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, BoboStateEvent> for LevelState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        initialise_camera(self.level_entity, world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world
            .delete_entity(self.level_entity)
            .expect("Level entity was deleted");
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: BoboStateEvent,
    ) -> Trans<GameData<'a, 'b>, BoboStateEvent> {
        match &event {
            BoboStateEvent::Window(e)
                if is_close_requested(e) || is_key_down(&e, VirtualKeyCode::Escape) =>
            {
                Trans::Quit
            }
            BoboStateEvent::Window(e) if is_key_down(e, VirtualKeyCode::R) => Trans::Pop,
            BoboStateEvent::Game(GameEvent::Win) => Trans::Pop,
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, BoboStateEvent> {
        data.data.update(data.world);
        Trans::None
    }
}

fn initialise_camera(parent: Entity, world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            CAMERA_WIDTH,
            CAMERA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(Matrix4::from_translation(Vector3::new(
            0.0, 0.0, 2.0,
        ))))
        .with(Parent { entity: parent })
        .build()
}

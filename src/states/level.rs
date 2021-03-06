use amethyst::core::{Parent, Transform};
use amethyst::ecs::Entity;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};
use amethyst::winit::VirtualKeyCode;

use super::LevelLoaderState;
use crate::components::*;
use crate::events::*;

pub const CAMERA_WIDTH: f32 = PIXEL_PER_CASE * LEVEL_WIDTH as f32;
pub const CAMERA_HEIGHT: f32 = PIXEL_PER_CASE * LEVEL_HEIGHT as f32;

pub struct LevelState {
    level_entity: Entity,
    level_name: String,
}

impl LevelState {
    pub fn new<S>(level_entity: Entity, level_name: S) -> LevelState
    where
        S: Into<String>,
    {
        LevelState {
            level_entity,
            level_name: level_name.into(),
        }
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
            BoboStateEvent::Window(e) if is_key_down(e, VirtualKeyCode::R) => Trans::Switch(
                Box::new(LevelLoaderState::for_level(self.level_name.as_ref())),
            ),
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

use amethyst::assets::Prefab;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::input::{InputEvent, is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, Transparent};
use amethyst::winit::VirtualKeyCode;

use crate::assets::EditorAssets;
use crate::components::*;
use crate::prefabs::{DirectOrLink, EntityPrefabData, LevelPrefabData};
use crate::editor::{
    components::*,
    events::EditorStateEvent,
    inputs::EditorInputAction,
};

#[derive(Debug)]
pub struct LevelEditorState {
    level_entity: Option<Entity>,
}

impl LevelEditorState {
    pub fn new() -> Self {
        LevelEditorState { level_entity: None }
    }

    fn save(&self, world: &mut World) {
        type SystemData<'a> = (
            ReadStorage<'a, CellCoordinate>,
            ReadStorage<'a, Instruction>,
            ReadStorage<'a, Named>,
            ReadStorage<'a, SpriteRender>,
            ReadStorage<'a, Transparent>,
        );
         let mut prefab_entities = Vec::<LevelPrefabData>::new();

        world.exec(|(coords, insts, names, sprites, transparent): SystemData| {
            for (&position, instruction, &name, sprite, transp) in (&coords, insts.maybe(), &names, &sprites, transparent.maybe()).join() {
                debug!("Adding to {:?}", position);
                prefab_entities.push(LevelPrefabData {
                    position,
                    entity: DirectOrLink::Direct(EntityPrefabData {
                        instruction: instruction.cloned(),
                        name: Some(name),
                        sprite: Some(sprite.sprite_number),
                        transparent: transp.is_some(),
                    }),
                });
            }
        });

        let mut prefab = Prefab::new();
        prefab_entities.into_iter().for_each(|e| { prefab.add(Some(0), Some(e)); });

        info!("Serialized: {}", ::ron::ser::to_string(&prefab).expect("Should have serialized"));
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, EditorStateEvent> for LevelEditorState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        let level_entity = world.create_entity().build();

        let cursor_sprite = world
            .read_resource::<EditorAssets>()
            .sprite(EditorAssets::SPRITE_CURSOR);

        world
            .create_entity()
            .with(CellCoordinate::default())
            .with(Cursor)
            .with(Transform::default())
            .with(cursor_sprite)
            .build();

        camera::initialise_camera(level_entity, world);

        self.level_entity = Some(level_entity);
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData>,
        event: EditorStateEvent,
    ) -> Trans<GameData<'a, 'b>, EditorStateEvent> {
        match &event {
            EditorStateEvent::Window(e)
                if is_close_requested(e) || is_key_down(&e, VirtualKeyCode::Escape) =>
            {
                Trans::Quit
            },
            EditorStateEvent::Input(InputEvent::ActionPressed(EditorInputAction::Save)) => {
                self.save(data.world);
                Trans::None
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, EditorStateEvent> {
        data.data.update(data.world);

        Trans::None
    }
}

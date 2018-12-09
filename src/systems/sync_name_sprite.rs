use amethyst::core::specs::BitSet;
use amethyst::ecs::prelude::{
    ComponentEvent, Join, ReadExpect, ReadStorage, ReaderId, Resources, System, WriteStorage,
};
use amethyst::renderer::SpriteRender;

use crate::assets::*;
use crate::components::Named;

// TODO Should sync inserted?
#[derive(Default)]
pub struct SyncNameAndSpriteSystem {
    named_events_id: Option<ReaderId<ComponentEvent>>,

    modified: BitSet,
}

impl<'a> System<'a> for SyncNameAndSpriteSystem {
    type SystemData = (
        ReadExpect<'a, GameAssets>,
        ReadStorage<'a, Named>,
        WriteStorage<'a, SpriteRender>,
    );

    fn setup(&mut self, res: &mut Resources) {
        use amethyst::ecs::SystemData;
        Self::SystemData::setup(res);

        let mut names = WriteStorage::<Named>::fetch(res);
        self.named_events_id = Some(names.register_reader());
    }

    fn run(&mut self, (assets, names, mut sprites): Self::SystemData) {
        names
            .channel()
            .read(self.named_events_id.as_mut().expect("setup was not called"))
            .filter_map(|event| match event {
                ComponentEvent::Modified(id) => Some(id),
                _ => None,
            })
            .for_each(|id| {
                self.modified.add(*id);
            });

        for (name, sprite, _) in (&names, &mut sprites, &self.modified).join() {
            let sprite_number = match name {
                Named::Bobo => ENTITY_SPRITE_BOBO,
                Named::Instruction => continue,
                Named::Flag => ENTITY_SPRITE_FLAG,
                Named::Wall => ENTITY_SPRITE_WALL,
            };

            *sprite = assets.entity_sprite(sprite_number);
        }
    }
}

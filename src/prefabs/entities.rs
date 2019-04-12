use amethyst::assets::{PrefabData, PrefabError};
use amethyst::ecs::{Entity, ReadExpect, WriteStorage};
use amethyst::renderer::{SpriteRender, Transparent};
use serde::{Serialize, Deserialize};

use crate::assets::GameAssets;
use crate::components::{Named, Instruction};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntityPrefabData {
    #[serde(default)]
    instruction: Option<Instruction>,
    #[serde(default)]
    name: Option<Named>,
    // TODO Use another prefab?
    #[serde(default)]
    sprite: Option<usize>,
    #[serde(default)]
    transparent: bool,
}

impl<'a> PrefabData<'a> for EntityPrefabData {

    type SystemData = (
        ReadExpect<'a, GameAssets>,
        WriteStorage<'a, Instruction>,
        WriteStorage<'a, Named>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
    );

    type Result = ();

    fn add_to_entity(&self,
                     entity: Entity,
                     system_data: &mut Self::SystemData,
                     _entities: &[Entity]) -> Result<Self::Result, PrefabError> {
        if let Some(inst) = self.instruction {
            system_data.1.insert(entity, inst)?;
        }

        if let Some(name) = self.name {
            system_data.2.insert(entity, name)?;
        }

        if let Some(sprite) = self.sprite {
            system_data.3.insert(entity, system_data.0.entity_sprite(sprite))?;
        }

        if self.transparent {
            system_data.4.insert(entity, Transparent)?;
        }

        Ok(())
    }
}

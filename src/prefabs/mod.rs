use amethyst::assets::{PrefabData, PrefabError};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Entity, ReadExpect, WriteStorage};
use amethyst::renderer::{SpriteRender, Transparent};
use serde::{Deserialize, Serialize};

use crate::assets::*;
use crate::components::{CellCoordinate, Instruction, Named};

// TODO Use better type. Must avoid using twice the same resource in PrefabDatas
type LevelSystemData<'a> = (
    WriteStorage<'a, GlobalTransform>,
    WriteStorage<'a, Transform>,
    WriteStorage<'a, CellCoordinate>,
    ReadExpect<'a, GameAssets>,
    WriteStorage<'a, Named>,
    WriteStorage<'a, Instruction>,
    WriteStorage<'a, SpriteRender>,
    WriteStorage<'a, Transparent>,
);

#[derive(Deserialize, Serialize)]
pub struct LevelPrefabData {
    position: CellCoordinate,
    typ: LevelPrefabEntityType,
}

impl<'a> PrefabData<'a> for LevelPrefabData {
    type SystemData = LevelSystemData<'a>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        data: &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<Self::Result, PrefabError> {
        self.position.add_to_entity(entity, data, entities)?;
        self.typ.add_to_entity(entity, data, entities)?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub enum LevelPrefabEntityType {
    Bobo,
    Flag,
    Instruction(Instruction),
    Wall,
}

impl<'a> PrefabData<'a> for LevelPrefabEntityType {
    type SystemData = LevelSystemData<'a>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        data: &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<(), PrefabError> {
        let (name, sprite_number) = match self {
            LevelPrefabEntityType::Bobo => {
                data.1
                    .get_mut(entity)
                    .expect("Transform should have been added")
                    .translation
                    .z = 1.0;
                (Named::Bobo, ENTITY_SPRITE_BOBO)
            }
            LevelPrefabEntityType::Flag => unimplemented!(),
            LevelPrefabEntityType::Instruction(i) => {
                data.5.insert(entity, *i)?;
                data.1
                    .get_mut(entity)
                    .expect("Transform should have been added")
                    .translation
                    .z = 1.0;
                (
                    Named::Instruction,
                    match i {
                        Instruction::Name(n) => {
                            match n {
                                Named::Bobo => ENTITY_SPRITE_INST_BOBO,
                                //Named::Flag => ENTITY_SPRITE_INST_FLAG,
                                Named::Wall => ENTITY_SPRITE_INST_WALL,
                                _ => unimplemented!(),
                            }
                        }
                        Instruction::Is => ENTITY_SPRITE_INST_IS,
                        Instruction::Cap(c) if c.is_push => ENTITY_SPRITE_INST_PUSH,
                        Instruction::Cap(c) if c.is_stop => ENTITY_SPRITE_INST_STOP,
                        Instruction::Cap(c) if c.is_you => ENTITY_SPRITE_INST_YOU,
                        Instruction::Cap(c) if c.is_win => unimplemented!(), //ENTITY_SPRITE_INST_WIN,
                        Instruction::Cap(_) => panic!("Invalid cap"),
                    },
                )
            }
            LevelPrefabEntityType::Wall => (Named::Wall, ENTITY_SPRITE_WALL),
        };

        data.4.insert(entity, name)?;
        data.6.insert(entity, data.3.entity_sprite(sprite_number))?;
        data.7.insert(entity, Transparent)?;

        Ok(())
    }
}

impl<'a> PrefabData<'a> for CellCoordinate {
    type SystemData = LevelSystemData<'a>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        data: &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<Self::Result, PrefabError> {
        data.0.insert(entity, GlobalTransform::default())?;
        data.1.insert(entity, Transform::default())?;
        data.2.insert(entity, self.clone())?;

        Ok(())
    }
}

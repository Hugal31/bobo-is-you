mod cached;
mod entities;

use amethyst::assets::{Prefab, PrefabData, PrefabError, ProgressCounter, RonFormat};
use amethyst::core::Transform;
use amethyst::derive::PrefabData;
use amethyst::ecs::prelude::{Entity, WriteStorage};
use serde::{Deserialize, Serialize};

use crate::components::CellCoordinate;

use self::cached::CachedAssetPrefab;

pub use self::entities::EntityPrefabData;

#[derive(Deserialize, Serialize)]
#[serde(untagged, bound(deserialize=""))]
pub enum DirectOrLink<T>
where
    T: for<'d> Deserialize<'d> + Send + Sync + 'static,
{
    #[serde(skip)]
    CachedRon(CachedAssetPrefab<Prefab<T>, RonFormat>),
    Direct(T),
    Link(String),
}

impl<'a, T> PrefabData<'a> for DirectOrLink<T>
where
    T: for<'d> Deserialize<'d> + Send + Sync + 'static,
    T: PrefabData<'a>,
{
    type SystemData = (
        <CachedAssetPrefab<Prefab<T>, RonFormat> as PrefabData<'a>>::SystemData,
        <T as PrefabData<'a>>::SystemData,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        data: &mut Self::SystemData,
        entities: &[Entity]) -> Result<Self::Result, PrefabError>
    {
        match self {
            DirectOrLink::CachedRon(cached) => return cached.add_to_entity(entity, &mut data.0, entities)
                .map(|_| ()),
            DirectOrLink::Direct(t) => t.add_to_entity(entity, &mut data.1, entities)
                .map(|_| ()),
            DirectOrLink::Link(_) => unreachable!(), // load_sub_assets is called first
        }
    }

    fn load_sub_assets(&mut self,
                       progress: &mut ProgressCounter,
                       system_data: &mut Self::SystemData) -> Result<bool, PrefabError>
    {
        let new_self = match self {
            DirectOrLink::CachedRon(cached) => return cached.load_sub_assets(progress, &mut system_data.0),
            DirectOrLink::Direct(t) => {
                return t.load_sub_assets(progress, &mut system_data.1);
            },
            DirectOrLink::Link(name) => {
                // TODO Check ends with RON
                let mut cached = CachedAssetPrefab::File(name.clone(), RonFormat, ());
                cached.load_sub_assets(progress, &mut system_data.0)?;
                DirectOrLink::CachedRon(cached)
            },
        };

        *self = new_self;

        Ok(true)
    }
}

#[derive(Deserialize, Serialize, PrefabData)]
pub struct LevelPrefabData {
    position: CellCoordinate,
    entity: DirectOrLink<EntityPrefabData>,
}

impl<'a> PrefabData<'a> for CellCoordinate {
    type SystemData = (
        WriteStorage<'a, CellCoordinate>,
        WriteStorage<'a, Transform>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        data: &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<Self::Result, PrefabError> {
        data.0.insert(entity, self.clone())?;
        data.1.insert(entity, Transform::default())?;

        Ok(())
    }
}

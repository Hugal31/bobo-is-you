use amethyst::assets::{Asset, AssetStorage, Format, Handle, Loader,
                       PrefabData, PrefabError, ProgressCounter};
use amethyst::ecs::{Entity, Read, ReadExpect, Write, WriteStorage};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

//#[derive(Default)]
pub struct HandleCache<A>(HashMap<String, Handle<A>>);

impl<A> Default for HandleCache<A>
{
    fn default() -> Self {
        HandleCache(HashMap::default())
    }
}

#[derive(Deserialize, Serialize)]
pub enum CachedAssetPrefab<A, F>
where
    A: Asset,
    F: Format<A>,
{
    /// From existing handle
    #[serde(skip)]
    Handle(Handle<A>),

    /// From file, (name, format, format options)
    File(String, F, F::Options),
}

impl <'a, A, F> PrefabData<'a> for CachedAssetPrefab<A, F>
where
    A: Asset,
    F: Format<A> + Clone,
    F::Options: Clone,
{
    type SystemData = (
        ReadExpect<'a, Loader>,
        WriteStorage<'a, Handle<A>>,
        Read<'a, AssetStorage<A>>,
        Write<'a, HandleCache<A>>,
    );
    type Result = Handle<A>;

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        _: &[Entity],
    ) -> Result<Handle<A>, PrefabError> {
        let handle = match *self {
            CachedAssetPrefab::Handle(ref handle) => handle.clone(),
            CachedAssetPrefab::File(..) => unreachable!(), // Because load_sub_assets is always run first
        };

        system_data.1.insert(entity, handle.clone())?;

        Ok(handle)
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        system_data: &mut Self::SystemData,
    ) -> Result<bool, PrefabError> {
        let (to_load, handle) = if let CachedAssetPrefab::File(ref name, ref format, ref options) = *self {
            if let Some(cached_handle) = (system_data.3).0.get(name) {
                debug!("Used a cached asset {}", name); // TODO Say which one?
                (false, cached_handle.clone())
            } else {
                let handle = system_data.0.load(
                    name.as_ref(),
                    format.clone(),
                    options.clone(),
                    progress,
                    &system_data.2,
                );
                (system_data.3).0.insert(name.clone(), handle.clone());
                (true, handle)
            }
        } else {
            return Ok(false);
        };

        *self = CachedAssetPrefab::Handle(handle);
        Ok(to_load)
    }
}

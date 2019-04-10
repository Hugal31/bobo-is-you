use amethyst::assets::ProgressCounter;
use amethyst::prelude::*;

use crate::assets::LoadableAsset;

pub struct LoaderState<'a, 'b, E: Send + Sync + 'static> {
    next_state: Option<Box<dyn State<GameData<'a, 'b>, E>>>,
    to_load: LoadableAsset,
    /// Progress tracker.
    progress: ProgressCounter,
}

impl<'a, 'b, E: Send + Sync + 'static> LoaderState<'a, 'b, E> {
    pub fn new(to_load: LoadableAsset, next_state: Box<dyn State<GameData<'a, 'b>, E>>) -> Self {
        LoaderState {
            next_state: Some(next_state),
            to_load,
            progress: ProgressCounter::default(),
        }
    }
}

impl<'a, 'b, E: Send + Sync + 'static> State<GameData<'a, 'b>, E> for LoaderState<'a, 'b, E> {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        let StateData { world, .. } = data;

        self.to_load.load(world, &mut self.progress);
    }

    fn update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, E> {
        data.data.update(data.world);

        if self.progress.is_complete() {
            debug!("Loading complete!");
            Trans::Switch(
                self.next_state
                    .take()
                    .expect("Should not call update() after Trans::Switch"),
            )
        } else {
            Trans::None
        }
    }
}

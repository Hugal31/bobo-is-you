use amethyst::core::bundle::*;
use amethyst::ecs::DispatcherBuilder;

use crate::systems::*;

pub struct BoboIsYouBundle;

impl SystemBundle<'_, '_> for BoboIsYouBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<()> {
        Ok(())
    }
}

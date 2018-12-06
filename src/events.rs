use amethyst::core::EventReader;
use amethyst::derive::*;
use amethyst::ecs::{Read, Resources, SystemData};
use amethyst::renderer::Event as WindowEvent;
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::UiEvent;

#[derive(Clone, EventReader)]
#[reader(BoboStateEventReader)]
pub enum BoboStateEvent {
    Window(WindowEvent),
    Ui(UiEvent),
    Game(GameEvent),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEvent {
    Win,
}

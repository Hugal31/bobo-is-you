use amethyst::core::EventReader;
use amethyst::derive::*;
use amethyst::ecs::{Read, Resources, SystemData};
use amethyst::input::InputEvent;
use amethyst::renderer::Event as WindowEvent;
use amethyst::shrev::{EventChannel, ReaderId};
// use amethyst::ui::UiEvent;

use crate::editor::inputs::EditorInputAction;

#[derive(Clone, EventReader)]
#[reader(EditorStateEventReader)]
pub enum EditorStateEvent {
    Window(WindowEvent),
    // Ui(UiEvent),
    Input(InputEvent<EditorInputAction>)
}

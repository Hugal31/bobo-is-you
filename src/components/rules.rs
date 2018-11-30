#![allow(dead_code)]

use amethyst::ecs::{storage::VecStorage, Component};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Named {
    Bobo,
    Flag,
    Wall,
}

impl Component for Named {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Capabilities {
    pub is_you: bool,
    pub is_stop: bool,
    pub is_win: bool,
}

impl Capabilities {
    pub fn is_you() -> Capabilities {
        Capabilities {
            is_you: true,
            ..Default::default()
        }
    }

    pub fn is_stop() -> Capabilities {
        Capabilities {
            is_stop: true,
            ..Default::default()
        }
    }

    pub fn is_win() -> Capabilities {
        Capabilities {
            is_win: true,
            ..Default::default()
        }
    }
}

pub struct Rules {
    pub bobo: Capabilities,
    pub flag: Capabilities,
    pub wall: Capabilities,
}

impl Rules {
    pub fn caps_for(&self, named: Named) -> &Capabilities {
        use self::Named::*;
        match named {
            Bobo => &self.bobo,
            Flag => &self.flag,
            Wall => &self.wall,
        }
    }

    pub fn caps_mut_for(&mut self, named: Named) -> &mut Capabilities {
        use self::Named::*;
        match named {
            Bobo => &mut self.bobo,
            Flag => &mut self.flag,
            Wall => &mut self.wall,
        }
    }
}

impl Default for Rules {
    fn default() -> Rules {
        Rules {
            bobo: Capabilities::is_you(),
            flag: Capabilities::is_win(),
            wall: Capabilities::is_stop(),
        }
    }
}

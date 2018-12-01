#![allow(dead_code)]

use std::ops::BitOr;

use amethyst::ecs::{storage::VecStorage, Component};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Named {
    Bobo,
    Instruction,
    Flag,
    Wall,
}

impl Component for Named {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Instruction {
    Name(Named),
    Is,
    Cap(Capabilities),
}

impl Component for Instruction {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Capabilities {
    pub is_you: bool,
    pub is_push: bool,
    pub is_stop: bool,
    pub is_win: bool,
}

impl Capabilities {
    pub fn empty() -> Capabilities {
        Capabilities::default()
    }

    pub fn is_you() -> Capabilities {
        Capabilities {
            is_you: true,
            ..Default::default()
        }
    }

    pub fn is_push() -> Capabilities {
        Capabilities {
            is_push: true,
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

impl BitOr for Capabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Capabilities {
            is_you: self.is_you | rhs.is_you,
            is_push: self.is_push | rhs.is_push,
            is_stop: self.is_stop | rhs.is_stop,
            is_win: self.is_win | rhs.is_win,
        }
    }
}

#[derive(Debug)]
pub struct Rules {
    pub bobo: Capabilities,
    pub flag: Capabilities,
    pub instructions: Capabilities,
    pub wall: Capabilities,
}

impl Rules {
    pub fn reset(&mut self) {
        self.bobo = Capabilities::empty();
        self.flag = Capabilities::empty();
        self.instructions = Capabilities::is_push();
        self.wall = Capabilities::empty();
    }

    pub fn caps_for(&self, named: Named) -> Capabilities {
        use self::Named::*;
        match named {
            Bobo => self.bobo,
            Flag => self.flag,
            Instruction => self.instructions,
            Wall => self.wall,
        }
    }

    pub fn caps_mut_for(&mut self, named: Named) -> &mut Capabilities {
        use self::Named::*;
        match named {
            Bobo => &mut self.bobo,
            Flag => &mut self.flag,
            Instruction => &mut self.instructions,
            Wall => &mut self.wall,
        }
    }
}

impl Default for Rules {
    fn default() -> Rules {
        Rules {
            bobo: Capabilities::is_you(),
            flag: Capabilities::is_win(),
            instructions: Capabilities::is_push(),
            wall: Capabilities::is_stop(),
        }
    }
}

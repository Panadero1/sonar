use serde::{Deserialize, Serialize};

use crate::{entity::player::Player, ui::img::Img, utility::animation::Animation, world::space::GamePos};

use super::{Tile, get_default_anim};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPillar {
    pos: GamePos,
    anim: Animation
}

#[typetag::serde]
impl Tile for TestPillar {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn on_player_enter(&mut self, _player: &mut Player) {
        // Doesn't need to do anything
    }

    fn update(&mut self) {
        // need update on clock add
    }
}

impl TestPillar {
    pub fn new(pos: GamePos) -> TestPillar {
        TestPillar {
            pos,
            anim: get_default_anim((2, 0))
        }
    }
}
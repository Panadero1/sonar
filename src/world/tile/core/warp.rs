use serde::{Deserialize, Serialize};

use crate::{
    draw::animation::Animation,
    world::{
        entity::player::Player,
        space::GamePos,
        tile::{get_default_anim, mountain::boulder::Boulder, PostOperation, Tile, TileVariant},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Warp {
    pos: GamePos,
    anim: Animation,
    load_name: String,
}

#[typetag::serde]
impl Tile for Warp {
    fn get_pos(&self) -> GamePos {
        self.pos
    }

    fn get_anim_mut(&mut self) -> &mut Animation {
        &mut self.anim
    }

    fn next(&self) -> Option<Box<dyn Tile>> {
        Some(Box::new(Boulder::new((0, 0).into(), TileVariant::Center)))
    }

    fn create(&self, pos: GamePos, _variant: TileVariant) -> Box<dyn Tile> {
        Box::new(Warp::new(pos))
    }

    fn on_player_enter(&mut self, _player: &mut Player, _move_pos: GamePos) -> Vec<PostOperation> {
        vec![PostOperation::LoadRegion(self.load_name.to_string())]
    }

    fn pick_tile(&self) -> Box<dyn Tile> {
        Box::new(Self {
            pos: (0, 0).into(),
            anim: get_default_anim((0, 0)),
            load_name: String::from("a"),
        })
    }
}

impl Warp {
    pub fn new(pos: GamePos) -> Warp {
        let mut name = String::new();

        println!("Enter the name of the zone for this tile to load: ");

        std::io::stdin().read_line(&mut name).unwrap();

        Warp {
            load_name: name.trim().to_string(),
            pos,
            anim: get_default_anim((2, 4)),
        }
    }

    pub fn default() -> Warp {
        Warp {
            pos: (0, 0).into(),
            anim: get_default_anim((2, 4)),
            load_name: String::from(""),
        }
    }
}

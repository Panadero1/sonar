use crate::{
    entity::{player::Player, tile::Tile, Entity},
    screen::camera::Camera,
    ui::img::{Img, ImgManager},
    utility::animation::{Animation, AnimationSelectError},
};

use self::space::GamePos;
use serde::{Deserialize, Serialize};
use speedy2d::{color::Color, Graphics2D};

pub mod space;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub regions: Vec<Region>,
    pub player: Player,
    pub camera: Camera,
}

impl World {
    pub fn new(regions: Vec<Region>, player: Player, camera: Camera) -> World {
        World { regions, player, camera }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    tiles: Vec<Tile>,
}

impl Region {
    pub fn new(tiles: Vec<Tile>) -> Region {
        Region { tiles }
    }

    pub fn draw_before_player(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        camera: &Camera,
        player_pos: GamePos,
    ) {
        for tile in self
            .tiles
            .iter_mut()
            .filter(|t| t.get_pos().y <= player_pos.y)
        {
            tile.draw(graphics, manager, camera);
        }
    }
    pub fn draw_after_player(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        camera: &Camera,
        player_pos: GamePos,
    ) {
        for tile in self
            .tiles
            .iter_mut()
            .filter(|t| t.get_pos().y > player_pos.y)
        {
            tile.draw(graphics, manager, camera);
        }
    }
}

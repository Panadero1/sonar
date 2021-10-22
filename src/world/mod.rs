use crate::{entity::{Entity, player::Player, tile::Tile}, screen::camera::Camera, ui::img::{Img, ImgManager}, utility::animation::{Animation, AnimationSelectError}};

use self::{space::GamePos, time::Clock};
use serde::{Deserialize, Serialize};
use speedy2d::{color::Color, Graphics2D};

pub mod space;
pub mod time;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub regions: Vec<Region>,
    pub player: Player,
    pub camera: Camera,
    pub clock: Clock,
}

impl World {
    pub fn new(regions: Vec<Region>, player: Player, camera: Camera, clock: Clock) -> World {
        World { regions, player, camera, clock }
    }
    pub fn update(&mut self) {
        self.clock.tick();
        for r in &mut self.regions {
            r.update(&self.clock);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    tiles: Vec<Box<dyn Tile>>,
}

impl Region {
    pub fn new(tiles: Vec<Box<dyn Tile>>) -> Region {
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
    pub fn tile_at_pos(&mut self, pos: GamePos) -> Option<&mut Box<dyn Tile>> {
        self.tiles.iter_mut().find(|t| t.get_pos() == pos)
    }
    pub fn update(&mut self, clock: &Clock) {
        for t in &mut self.tiles {
            t.update_anim(clock);
        }
    }
}

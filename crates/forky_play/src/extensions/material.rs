use bevy::prelude::*;
use extend::ext;

#[ext]
pub impl Assets<StandardMaterial> {
    //TODO forward faces back
    fn from_white(&mut self) -> Handle<StandardMaterial> {
        self.add(Color::WHITE)
    }

    fn from_color(&mut self, color: Color) -> Handle<StandardMaterial> {
        self.add(color)
    }
    fn from_rgb(&mut self, r: f32, g: f32, b: f32) -> Handle<StandardMaterial> {
        self.add(Color::srgba(r, g, b, 1.))
    }
}

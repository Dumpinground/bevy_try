use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

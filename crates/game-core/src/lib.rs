pub use bevy::{prelude::*,};




//not sure the usefulness of a core but will see later. Might come in use when I think of all systems working together.



pub mod stages {
    pub const INIT: &'static str = "init";
    pub const POST_INIT: &'static str = "post_init";
    pub const PRE_UPDATE: &'static str = "pre_update";
    pub const UPDATE: &'static str = "update";
    pub const POST_UPDATE: &'static str = "post_update";
}

pub struct CorePlugin;
impl Plugin for CorePlugin {
  fn build( &self, app: &mut AppBuilder) {
    
  }
}

#[derive(Debug)]
pub struct InputTimer(pub Timer);
impl Default for InputTimer {
    fn default() -> Self {
        InputTimer(Timer::from_seconds(0.1, false))
    }
}



pub struct WorldSettings {
    pub tile_size: f32,
    pub base_player_speed: f32,
    pub base_npc_speed: f32,
    pub base_scale: f32,
  }
  
  impl Default for WorldSettings {
    fn default() -> Self {
      WorldSettings {
        tile_size: 16.,
        base_player_speed: 8.,
        base_npc_speed: 8.,
        base_scale: 5.,
      }
    }
  }

pub enum Direction {
  None,
  Left,
  Right,
  Up,
  Down,
}
pub struct GameTimer(Timer);
///Component: vector 1, vector 2 
pub struct Movement{
  vector1: Direction,
  vector2: Direction,
}

pub fn timer_ticker_system(
  timer: ResMut<GameTimer>,
  time: Res<Time>,
) {

}

/// movement system should be the same for NPC and Player imo
/// we need movment, time resource that ticks down, timer that can finish, 
/// and the entity we are moving
pub fn move_system(
  movement: Res<Movement>,
  timer: Res<Timer>,
  entity: Entity
) {

}












#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

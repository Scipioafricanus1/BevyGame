use bevy::{prelude::*,};
use game_entities::*;



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
    app.add_resource( GameTimer( Timer::from_seconds(0.1, false) ))
    .add_startup_system_to_stage(stages::INIT, spawn_player.system());
    
  }
}

fn core_setup(
  commands: &mut Commands,
  materials: Res<Materials>,
  mut color_materials: ResMut<Assets<ColorMaterial>>
) {
  commands
  .insert_resource(Materials {
    player_material: color_materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
  });
}

fn spawn_player(commands: &mut Commands, materials: Res<Materials>) {
  commands.spawn(SpriteBundle {
    material: materials.player_material.clone(),
    ..Default::default()
  }).with(PlayerEntity::default());
}


struct GameTimer(Timer);

//system for change in time every loop
fn timer_ticker_system(
  mut timer:  ResMut<GameTimer>,
  time: Res<Time>,
) {
  timer.0.tick(time.delta_seconds());
}




struct Materials {
  player_material: Handle<ColorMaterial>,
}









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

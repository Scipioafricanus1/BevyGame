use bevy::{
    prelude::*,
    prelude::KeyCode::*
};

use game_scenes::*;
use game_core::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        //add init and post init stages, then add post update stage after update
        .add_startup_stage(stages::INIT, SystemStage::parallel())
        .add_startup_stage(stages::POST_INIT, SystemStage::parallel())
        .add_startup_system(setup.system())
        //if init is a startup, you have to use add_startup_system_to_stage instead of just add system to stage
        .add_startup_system_to_stage(stages::INIT, spawn_player.system())
        .add_system(player_movement.system())
        .run();
}
///initialize some things first thing
fn setup( commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
  /* create a sprite on the map 
  for now, lets just get some basic code down then separate it into the stages later when we realize how it should work.*/
  commands.spawn(Camera2dBundle::default());
  //you gotta add the materials you're going to have on the map as a resource
  commands.insert_resource(Materials {
    player_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
  });

}

struct Player;
struct Materials {
  player_material: Handle<ColorMaterial>,
}

fn spawn_player(commands: &mut Commands, materials: Res<Materials>) {
  commands
    .spawn(SpriteBundle {
      material: materials.player_material.clone(),
      sprite: Sprite::new(Vec2::new(10.0, 10.0)),
      ..Default::default()
    })
    .with(Player);
}

fn player_movement( keyboard_input: Res<Input<KeyCode>>,
  mut player_positions: Query<&mut Transform, With<Player>>) {
  for mut transform in player_positions.iter_mut() {
    // transform.translation.y += 2.;
    if keyboard_input.pressed(KeyCode::Left) {
      transform.translation.x -= 2.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
      transform.translation.x += 2.;
    }
    if keyboard_input.pressed(KeyCode::Up) {
      transform.translation.y += 2.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
      transform.translation.y -= 2.;
    }
  }
}

/*Planning of ECS and basics
You could build the same set of mobs with components for: Position, Renderable, Hostile, MeleeAI, RangedAI, and some sort of CombatStats component 
(to tell you about their weaponry, hit points, etc.).
  An Orc Warrior would need a position so you know where they are, a renderable so you know how to draw them.
   It's Hostile, so you mark it as such. Give it a MeleeAI and a set of game stats, and you have everything you need to 
   make it approach the player and try to hit them. An Archer might be the same thing, but replacing MeleeAI with RangedAI. 
   A hybrid could keep all the components, but either have both AIs or an additional one if you want custom behavior.
 If your orc becomes friendly, you could remove the Hostile component - and add a Friendly one.
*/

//Things I'd need for an enemy unit:  Position, Renderable, Hostile component, Melee or Ranged AI, Combat Stats component.
//when using an ecs system, the easiest way to grab a group or a singular unit is to query that unit's specific component types.
// if multiple units have the same components, and you wan this one unit to have have its own functionality specific to it, then
// you obviously need another component for this specific functionality. e.g. a hostile enemy vs. a friendly enemy have separate
//components for hostile and friendly that affect the AI components.
//the player character has extra components for movement by player.

//use an enum to declare which map tiles are walkable and which aren't.
//when moving units, check to see if the next tiles are impassible (walls, etc.) tile types.
//Scenes in Bevy engine:
/*
In bevy, you could add a scene file that does the work here. 
*/


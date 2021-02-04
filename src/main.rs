use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

use game_scenes::*;
use game_core::*;
use rand::prelude::random;
use std::time::Duration;

const ARENA_WIDTH : u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
          title: "Game".to_string(),
          width: 500.0,
          height: 500.0,
          ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(PlayerMoveTimer(Timer::new(
          Duration::from_millis(150. as u64),
          true,
        )))
        .add_resource(PlayerSegments::default())
        .add_resource(LastTailPosition::default())
        .add_plugins(DefaultPlugins)
        .add_event::<GrowthEvent>()
        .add_system(player_growth.system())
        .add_system(player_eating.system())
        //add init and post init stages, then add post update stage after update
        .add_startup_stage(stages::INIT, SystemStage::parallel())
        .add_startup_stage(stages::POST_INIT, SystemStage::parallel())
        .add_startup_system(setup.system())
        //if init is a startup, you have to use add_startup_system_to_stage instead of just add system to stage
        .add_startup_system_to_stage(stages::INIT, spawn_player.system())
        .add_system(player_movement.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(food_spawner.system())
        .add_system(player_timer.system())
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
    segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
  });

}
///keeps track of the movement of the snake with time and timer until it ticks the number correct ammount
fn player_timer(time: Res<Time>, mut player_timer: ResMut<PlayerMoveTimer>) {
  player_timer.0.tick(time.delta_seconds());
}

fn spawn_segment(
  commands: &mut Commands,
  material: &Handle<ColorMaterial>,
  position: Position,
) -> Entity {
  commands
    .spawn(SpriteBundle {
      material: material.clone(),
      ..Default::default()
    })
    .with(PlayerSegment)
    .with(position)
    .with(Size::square(0.65))
    .current_entity()
    .unwrap()
}

///NOTE: every time you add a new component that will be represented on the screen,
/// you will need a new resource to be initialized.
///  You can bundle them up in a materials struct for readability.
struct PlayerSegment;

#[derive(Default)]
struct PlayerSegments(Vec<Entity>);

#[derive(Default)]
struct LastTailPosition(Option<Position>);

#[derive(PartialEq, Copy, Clone)]
enum Direction {
  Left,
  Up,
  Right,
  Down,
}

impl Direction {
  fn opposite(self) -> Self {
    match self {
      Self::Left =>Self::Right,
      Self::Right => Self::Left,
      Self::Up => Self::Down,
      Self::Down=> Self::Up,
    }
  }
}

struct PlayerMoveTimer(Timer);
struct Player {
  direction: Direction,
}
struct Materials {
  player_material: Handle<ColorMaterial>,
  segment_material: Handle<ColorMaterial>,
  food_material: Handle<ColorMaterial>,
}

struct Food;

struct FoodSpawnTimer(Timer);
impl Default for FoodSpawnTimer {
  fn default() -> Self {
    Self(Timer::new(Duration::from_millis(1000), true))
  }
}
///System for spawning food with a local resource time
fn food_spawner(
  commands: &mut Commands,
  materials: Res<Materials>,
  time: Res<Time>,
  mut timer: Local<FoodSpawnTimer>,
) {
  if timer.0.tick(time.delta_seconds()).finished() {
    commands
      .spawn(SpriteBundle {
        material: materials.food_material.clone(),
        ..Default::default()
      })
      .with(Food)
      .with(Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
      })
      .with(Size::square(0.8));
  }
}
///On startup, we spawn the player with a vector of segments,
/// player, position, size, spritebundle components
fn spawn_player(
  commands: &mut Commands, 
  materials: Res<Materials>,
  mut segments: ResMut<PlayerSegments>,
) {
  segments.0 = vec![
    commands
    .spawn(SpriteBundle {
      material: materials.player_material.clone(),
      sprite: Sprite::new(Vec2::new(10.0, 10.0)),
      ..Default::default()
    })
    .with( Player {
      direction: Direction::Up,
    })
    .with(Position{ x: 3, y: 3})
    .with(Size::square(0.8))
    .current_entity()
    .unwrap(),
    spawn_segment(
      commands,
      &materials.segment_material,
      Position { x: 3, y: 2 },
    ),
  ];
}

fn player_eating(
  commands: &mut Commands,
  player_timer: ResMut<PlayerMoveTimer>,
  mut growth_events: ResMut<Events<GrowthEvent>>,
  food_positions: Query<(Entity, &Position), With<Food>>,
  head_positions: Query<&Position, With<Player>>,)
{
  if !player_timer.0.finished() {
    return;
  }
  for head_pos in head_positions.iter() {
    for (ent, food_pos) in food_positions.iter() {
      if food_pos == head_pos {
        commands.despawn(ent);
        growth_events.send(GrowthEvent);
      }
      
    }
  }
}

struct GrowthEvent;

fn player_growth(
  commands: &mut Commands,
  last_tail_position: Res<LastTailPosition>,
  growth_events: Res<Events<GrowthEvent>>,
  mut segments: ResMut<PlayerSegments>,
  mut growth_reader: Local<EventReader<GrowthEvent>>,
  materials: Res<Materials>,
) {
  if growth_reader.iter(&growth_events).next().is_some() {
    segments.0.push(spawn_segment(
      commands,
      &materials.segment_material,
      last_tail_position.0.unwrap(),
    ));
  }
}

///If player timer is finished and direction is not opposite of current player direction,
/// change the direction of the player. else continue the same direction.
fn player_movement( 
  keyboard_input: Res<Input<KeyCode>>,
  player_timer: ResMut<PlayerMoveTimer>,
  segments: ResMut<PlayerSegments>,
  mut last_tail_position: ResMut<LastTailPosition>,
  mut players: Query<(Entity, &mut Player)>,
  mut positions: Query<&mut Position>) {
  //so we don't have two queries accessing the same component,
  // we separate it above, then grab them by iter_mut().next()
  // and positions.get_mut(player_entity)
  if let Some((player_entity, mut player)) = players.iter_mut().next() {
    let segment_positions = segments
      .0
      .iter()
      .map(|e| *positions.get_mut(*e).unwrap())
      .collect::<Vec<Position>>();
    let mut player_pos = positions.get_mut(player_entity).unwrap();
    //after getting player_position from the two queries, we use input to
    //determine the 
    let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
      Direction::Left
    } else if keyboard_input.pressed(KeyCode::Up) {
      Direction::Up
    } else if keyboard_input.pressed(KeyCode::Right) {
      Direction::Right
    } else if keyboard_input.pressed(KeyCode::Down) {
      Direction::Down
    } else {
      player.direction
    };
    if dir != player.direction.opposite() {
      player.direction = dir;
    }
    if !player_timer.0.finished() {
      return;
    }

    match &player.direction {
      Direction::Left => {
        player_pos.x -=1;
      }
      Direction::Right => {
        player_pos.x +=1;
      }
      Direction::Up => {
        player_pos.y +=1;
      }
      Direction::Down => {
        player_pos.y -=1;
      }
    }
    segment_positions
      .iter()
      .zip(segments.0.iter().skip(1))
      .for_each(|(pos, segment)| {
        *positions.get_mut(*segment).unwrap() = *pos;
      });
    last_tail_position.0 = Some(*segment_positions.last().unwrap());
  }
    
}



///If sprite is 1 width, in a grid of 40, and the window size is 400 px,
/// then pixel width will be 10px
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
  let window = windows.get_primary().unwrap();
  for (sprite_size, mut sprite) in q.iter_mut() {
    sprite.size = Vec2::new(
      sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
      sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
    )
  }
}


fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
  fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
  }
  let window = windows.get_primary().unwrap();
  for (pos, mut transform) in q.iter_mut() {
    transform.translation = Vec3::new(
      convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
      convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
      0.0,
    );
  }
}
///copyable, cloneable, hashable position.
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
  x: i32,
  y: i32,
}

struct Size {
  width: f32,
  height: f32,
}

impl Size {
  pub fn square(x: f32) -> Self {
    Self {
      width: x,
      height:x,
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


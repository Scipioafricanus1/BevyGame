use bevy::{
    prelude::*, 
    render::pass::ClearColor,
    window::WindowMode,
};
use rand::Rng;
fn main() {
    App::build()
        .init_resource::<Map>()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .add_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 800,
            height: 600,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(startup_system.system())
        .run();
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

fn xy_idx(x:i32, y:i32) -> usize {
    (y as usize * 80 ) + x as usize
}
#[derive(PartialEq,Copy, Clone)]
enum TileType {
    Floor,
    Wall,
}
#[derive(Default)]
struct Map {
    map: Vec<TileType>
}

fn startup_system( mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>, mut map: ResMut<Map> ) {
    //call new_map and assign this to map resource
    map.map = new_map();

    //draw_map
    let mut y = 0;
    let mut x = 0;

    let map = &map.map;
    commands.spawn(Camera2dComponents::default());
    let mut i = 0;
    for tile in map.iter() {
        println!("tile Made: {} at x:{} y: {}", i, x, y);
        i+=1;
        match tile {
            TileType::Floor => {
                commands
                .spawn(SpriteComponents {
                    material: materials.add(Color::GREEN.into()),
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                    ..Default::default()
                });
            },
            TileType::Wall => {
                commands
                .spawn(SpriteComponents {
                    material: materials.add(Color::BLACK.into()),
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                    ..Default::default()
                });
            }
        }
        x += 10;
        if x > 799 {
            x = 0;
            y += 10;
        }
    }
}

fn new_map() -> Vec<TileType> {
    let columns:i32 = 80;
    let rows:i32 = 60;
    let mut map = vec![TileType::Floor; columns as usize * rows as usize];

    for x in 0..columns {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, rows-1)] = TileType::Wall;
    }
    for y in 0..rows {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(columns-1, y)] = TileType::Wall;
    }

    //create 400 random walls
    let mut rng = rand::thread_rng();
    for i in 0..400 {
        let x = rng.gen_range(0, columns-1);
        let y = rng.gen_range(0, rows-1);
        let idx = xy_idx(x, y);
        // if not the place for character.
        if idx != xy_idx(columns/2, rows/2) {
            map[idx] = TileType::Wall;
        }
    }
    map
}


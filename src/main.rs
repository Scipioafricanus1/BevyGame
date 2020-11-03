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
pub enum TileType {
    Floor,
    Wall,
}
#[derive(Default)]
pub struct Map {
    map: Vec<TileType>
}

fn startup_system( mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>, mut map: ResMut<Map> ) {
    map.map = new_map_rooms_and_corridors();

    //draw_map
    let mut y = 0;
    let mut x = 0;

    let map = &map.map;
    commands.spawn(Camera2dComponents {
        transform: Transform::from_translation(Vec3::new(400.0, 300.0, 0.0)),
        ..Default::default()
    });
    
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

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let columns:i32 = 80;
    let rows:i32 = 60;
    let mut map = vec![TileType::Wall; columns as usize * rows as usize];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    apply_room_to_map(&room1, &mut map);
    apply_room_to_map(&room2, &mut map);

    map
}

pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    /// makes rectangle with points x1, y1, x2, y2 using x, y, width, height
    pub fn new(x1: i32, y1: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1, y1, x2: x1+w, y2: y1 + h,            
        }
    }
    ///returns true if they both intersect. 
    pub fn intersect(&self, other: &Rect) -> bool {
        other.x1 <= self.x2 && other.x2 >= self.x1 && other.y1 <= self.y2 && other.y2 >= self.y1
    }


    ///computes the x, y coords for the center of the rectangle.
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }
}
///takes rect room, map, and draws floor pixels onto map.
fn apply_room_to_map(room: &Rect, map: &mut [TileType] ) {
    for y in room.y1+1..= room.y2 {
        for x in room.x1 + 1..= room.x2 {
            map[xy_idx(x,y)] = TileType::Floor;
        }
    }
}
use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
    sprite::TextureAtlasBuilder,
};
fn main() {
    App::build()
        .add_resource(monsterAnimationTimer(Timer::from_seconds(0.05)))
        .add_resource(characterAnimationTimer(Timer::from_seconds(0.1)))
        .add_default_plugins()
        // .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_startup_system(setup2.system())
        .add_system(animate_monster_system.system())
        .add_system(move_sprite_system.system())
        .run();
}

struct monsterAnimationTimer(Timer);
struct characterAnimationTimer(Timer);


pub struct Gabe {
    speed: f32,
}
pub struct Monster {
    speed: f32,
}


pub fn animate_monster_system(
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &Monster, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, monster,  mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len() as u32;
            timer.reset();
        }
    }
}
pub fn move_sprite_system(
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Gabe, &mut Translation, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
        for ( gabe, mut translation, mut timer, mut sprite,  texture_atlas_handle) in &mut query.iter() {
            let mut direction = 0.0;
            if keyboard_input.pressed(KeyCode::Left) {
                direction -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                direction += 1.0;
            }
    
            if timer.finished
                && (keyboard_input.pressed(KeyCode::Left) && !keyboard_input.pressed(KeyCode::Right))
            {
                let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
                let half_size = (texture_atlas.len() / 2) as u32;
                if sprite.index < half_size || sprite.index == (texture_atlas.len()-1 as usize) as u32 {
                  sprite.index = half_size;
                  println!("1: sprite.index: {}", sprite.index);
                }
                sprite.index = (sprite.index + 1 )% texture_atlas.len() as u32;
                println!("2: sprite.index: {}", sprite.index);
                timer.reset();
            } else if !keyboard_input.pressed(KeyCode::Left) && !keyboard_input.pressed(KeyCode::Right)
            {
                if sprite.index < 7 {
                    sprite.index = 0;
                } else {
                    sprite.index = 7;
                }
            } else if timer.finished && (keyboard_input.pressed(KeyCode::Right) && !keyboard_input.pressed(KeyCode::Left) )
            {
                let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
                let half_size = (texture_atlas.len() / 2) as u32;
                if sprite.index > half_size {
                    sprite.index = 0;
                }
                sprite.index = (sprite.index + 1) % half_size;
                timer.reset();
            } else if keyboard_input.pressed(KeyCode::Space) {
                // start animation lock for jump, but allow movement still in air, just more floaty? 
                //if the user just jumped, other animations cannot run for entirety of airtime. this requires collision detection with the ground.
                //We need a ground then!
            }
            // should have the texture decided for each button press.
            *translation.0.x_mut() += time.delta_seconds * direction * gabe.speed
            //TODO: Add a jump button. A quick force into the air.
            // if keyboard_input.pressed(KeyCode::Space) {
    
            // }
        }
}
    




fn setup2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(
            &mut textures,
            "assets/textures/rpg/chars/gabe/gabe-idle-run-fix.png",
        )
        .unwrap();
    // let texture_handle_ground = asset_server
        // .load_sync(
        //     &mut textures,
        //     "assets/textures/groundTextures/groundTexture.png",
        // )
        // .unwrap();
    let texture_handle_monster = asset_server
        .load_sync(
            &mut textures,
            "assets/textures/monsters/monster.png",
        )
        .unwrap();
    let texture_monster = textures.get(&texture_handle_monster).unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    // let texture_ground = textures.get(&texture_handle).unwrap();
    let texture_atlas_monster = TextureAtlas::from_grid(texture_handle_monster, texture_monster.size, 17, 1);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 14, 1);
    // let texture_atlas_ground = TextureAtlas::from_grid(texture_handle_ground, texture_ground.size, 4, 1);
    let texture_atlas_handle_monster = texture_atlases.add(texture_atlas_monster);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // let texture_atlas_handle_ground = texture_atlases.add(texture_atlas_ground);

    let monster = commands
        .spawn(Camera2dComponents::default())
        .spawn( SpriteSheetComponents {
            texture_atlas: texture_atlas_handle_monster,
            scale: Scale(5.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.05))
        .with(Monster{ speed: 500.0 }).current_entity();
        
        let gabe = commands.spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(8.0),
            ..Default::default()
        })
        
        .with(Timer::from_seconds(0.1))
        .with(Gabe { speed: 500.0 }).current_entity();
        // .spawn(SpriteSheetComponents {
        //     texture_atlas: texture_atlas_handle_ground,
        //     scale:Scale(2.0), 
        //     ..Default::default()
        // });

    }



    //planning

    //components
    struct Player {
        
    }

    struct UnitType {

    }

    

    //setup scene like so:

    fn setup4(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut textures: ResMut<Assets<Texture>>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {

    }
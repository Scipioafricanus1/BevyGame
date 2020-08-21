use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
    sprite::TextureAtlasBuilder,
};
fn main() {
    App::build()
        .init_resource::<RpgSpriteHandles>()
        .add_default_plugins()
        .add_startup_system(setup3.system())
        .add_system(move_sprite_system.system())
        .run();
}



#[derive(Default)]
pub struct RpgSpriteHandles {
    handles: Vec<HandleId>,
    atlas_loaded: bool,
}
fn setup3(mut rpg_sprite_handles: ResMut<RpgSpriteHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.handles = asset_server
        .load_asset_folder("assets/textures/rpg")
        .unwrap();
}

pub struct Gabe {
    speed: f32,
}

pub fn move_sprite_system(
    mut commands: Commands,
    mut rpg_sprite_handles:ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures:ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Gabe, &mut Translation, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    //might need to use this condition on the loading part. We shall see.
    if rpg_sprite_handles.atlas_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let Some(LoadState::Loaded(_)) =
        asset_server.get_group_load_state(&rpg_sprite_handles.handles)
    {
        for texture_id in rpg_sprite_handles.handles.iter() {
            let handle = Handle::from_id(*texture_id);
            let texture = textures.get(&handle).unwrap();
            texture_atlas_builder.add_texture(handle, &texture);
        }
        
        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        // let texture_atlas_texture = texture_atlas.texture;
        let vendor_handle = asset_server
            .get_handle("assets/textures/rpg/chars/gabe/gabe-idle-run-left.png")
            .unwrap();
        let texture_atlas_gabe = TextureAtlas::from_grid(vendor_handle, texture_atlas.size, 7, 1);
        // let vendor_index = texture_atlas.get_texture_index(vendor_handle).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas_gabe);
        
        // set up a scene to display our texture atlas
        commands
        .spawn(Camera2dComponents::default())
        //draw a sprite from the atlas
        .spawn(SpriteSheetComponents {
            scale: Scale(4.0),
            // translation: Translation(Vec3::new(150.0, 0.0, 0.0)),
            // sprite: TextureAtlasSprite::new(vendor_index as u32),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1))
        .with(Gabe {
            speed: 500.0,
        });
        //draw the atlas itself
        // .spawn(SpriteComponents {
        //     material: materials.add(texture_atlas_texture.into()),
        //     translation: Vec3::new(-300.0, 0.0, 0.0).into(),
        //     ..Default::default()
        // });

        for ( gabe, mut translation, mut timer, mut sprite,  texture_atlas_handle) in &mut query.iter() {
            let mut direction = 0.0;
            if keyboard_input.pressed(KeyCode::Left) {
                direction -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                direction += 1.0;
            }
    
            if timer.finished
                && (keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Right))
            {
                let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
    
                timer.reset();
            } else if !keyboard_input.pressed(KeyCode::Left) && !keyboard_input.pressed(KeyCode::Right)
            {
                sprite.index = 0;
            } else if !keyboard_input.pressed(KeyCode::Right) && !keyboard_input.pressed(KeyCode::Left)
            {
                let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            }
            // should have the texture decided for each button press.
            *translation.0.x_mut() += time.delta_seconds * direction * gabe.speed
            //TODO: Add a jump button. A quick force into the air.
            // if keyboard_input.pressed(KeyCode::Space) {
    
            // }
        }

        rpg_sprite_handles.atlas_loaded = true;
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
            "assets/textures/rpg/chars/gabe/gabe-idle-run.png",
        )
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(2.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1))
        .with(Gabe { speed: 500.0 });
}

use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
fn main() {
    App::build()
        .add_default_plugins()
        // .add_resource(Scoreboard { score: 0 })
        // .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        // .add_startup_system(setup.system())
        // .add_system(paddle_movement_system.system())
        // .add_system(ball_collision_system.system())
        // .add_system(ball_movement_system.system())
        // .add_system(scoreboard_system.system())
        .add_startup_system(setup2.system())
        .add_system(move_sprite_system.system())
        .run();
}

struct Paddle {
    speed: f32,
}

struct Ball {
    velocity: Vec3,
}

struct Scoreboard {
    score: usize,
}

struct Brick;
struct Wall;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //Add the game's entities to world
    commands
        //cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        //paddle
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            translation: Translation(Vec3::new(0.0, -250.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(120.0, 30.0),
            },
            ..Default::default()
        })
        .with(Paddle { speed: 500.0})
        //ball
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            translation: Translation(Vec3::new(0.0, -100.0, 1.0)),
            sprite: Sprite {
                size: Vec2::new(30.0, 30.0),
            },
            ..Default::default()
        })
        .with(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        })
        //scoreboard
        .spawn(TextComponents {
            text: Text {
                font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.2, 0.2, 0.8).into(),
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });

    //Add walls
    let wall_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    commands
        //left
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Wall)
        //right
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Wall)
        //bottom
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        .with(Wall)
        //top
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        .with(Wall);

    //Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec2::new(150.0, 30.0);
    let bricks_width = brick_columns as f32 * (brick_size.x() + brick_spacing) - brick_spacing;
    //center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x()) / 2.0, 100.0, 0.0);

    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y() + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x() + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            commands
                //brick
                .spawn(SpriteComponents {
                    material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                    sprite: Sprite { size: brick_size },
                    translation: Translation(brick_position),
                    ..Default::default()
                })
                .with(Brick);
        }
    }
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Translation)>,
) {
    for (paddle, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Space) {

        }

        *translation.0.x_mut() += time.delta_seconds * direction * paddle.speed;
    }
}

fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Translation)>) {
    for (ball, mut translation) in &mut ball_query.iter() {
        translation.0 += ball.velocity * time.delta_seconds;
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}

fn ball_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Translation, &Sprite)>,
    mut paddle_query: Query<(&Paddle, &Translation, &Sprite)>,
    mut brick_query: Query<(Entity, &Brick, &Translation, &Sprite)>,
    mut wall_query: Query<(&Wall, &Translation, &Sprite)>,
) {
    for (mut ball, ball_translation, sprite) in &mut ball_query.iter() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;
        let mut collision = None;

        //check collision with walls
        for (_wall, translation, sprite) in &mut wall_query.iter() {
            if collision.is_some() {
                break;
            }
            collision = collide(ball_translation.0, ball_size, translation.0, sprite.size);
        }

        //check collision with paddles(s)
        for (_paddle, translation, sprite) in &mut paddle_query.iter() {
            if collision.is_some() {
                break;
            }

            collision = collide(ball_translation.0, ball_size, translation.0, sprite.size);
        }

        //check collision with bricks
        for (brick_entity, _brick, translation, sprite) in &mut brick_query.iter() {
            if collision.is_some() {
                break;
            }

            collision = collide(ball_translation.0, ball_size, translation.0, sprite.size);
            if collision.is_some() {
                scoreboard.score += 1;
                commands.despawn(brick_entity);
            }
        }

        //reflect the ball when it collides
        let mut reflect_x = false;
        let mut reflect_y = false;

        //only reflect if the ball's velocity is going in the opposite direction of the collision
        match collision {
            Some(Collision::Left) => reflect_x = velocity.x() > 0.0,
            Some(Collision::Right) => reflect_x = velocity.x() < 0.0,
            Some(Collision::Top) => reflect_y = velocity.y() < 0.0,
            Some(Collision::Bottom) => reflect_y = velocity.y() > 0.0,
            None => {}
        }

        //reflect velocity on the x-axis if we hit something on the x-axis
        if reflect_x {
            *velocity.x_mut() = -velocity.x();
        }

        //reflect velocity on the y-axis if we hit something on the y-axis
        if reflect_y {
            *velocity.y_mut() = -velocity.y();
        }
    }
}



struct Gabe {
    speed: f32,
}

fn move_sprite_system (
    texture_atlases: Res<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Gabe, &Handle<TextureAtlas>, &mut Translation)>,
) {
    for (mut timer, mut sprite, gabe, texture_atlas_handle, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        if timer.finished && (keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Right) ){
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            
            timer.reset();
        } else if !keyboard_input.pressed(KeyCode::Left) & !keyboard_input.pressed(KeyCode::Right) {
            sprite.index = 0;
        }
        *translation.0.x_mut() += time.delta_seconds * direction * gabe.speed
        //TODO: Add a jump button. A quick force into the air.
        // if keyboard_input.pressed(KeyCode::Space) {
            
        // }
    }

}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            timer.reset();
        }
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
        .with(Gabe {
            speed: 500.0,
        });
}

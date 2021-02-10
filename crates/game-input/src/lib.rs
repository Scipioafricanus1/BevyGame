use bevy::prelude::*;

use game_core::stages;
use game_entities::*;



struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder ) {
        app.add_system(movement_system.system());
    }
}


///Animations: running, jumping, diving 
fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transforms:  Query<&mut Transform, With<PlayerEntity>>,
    commands: &mut Commands
) {
    if let Some(mut transform) = player_transforms.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 0.1;
        } else if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 0.1; 
        }
    }
    
}






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

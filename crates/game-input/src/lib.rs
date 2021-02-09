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
    player: Query<PlayerEntity>
) {

}





















#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

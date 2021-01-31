pub use bevy::{prelude::*, reflect::TypeRegistry, utils::Duration};


///System for loading scenes. Should be based on a saveGame scene id
pub fn load_scene_system() {

}

///System for saving Scenes. Should open up a UI that shows past save games unless autosave
pub fn save_scene_system() {
    //will make a scene for the new game, separate from the master scene if there is one,  calls update_tilemap() which returns the generated tilemap name, adds the name of the updated Tilemap name and path as a part of the scene. uses this when you look at load_scene UI
}

//Should I couple scenes with tilemap systems? Probably not because tilemaps will be more complicated but scenes might alter tilemaps
///System for loading tilemaps with spritesheets
pub fn load_tilemap() {

}

///System for updating tilemaps when an event acts upon a tile
pub fn update_tilemap() {
    //given some tile position, we update that tsx file's specific value with the new tile. So imagine a destroyed door, once the door is destroyed, the tilemap is changed
    //This new tsx file then acts as this save game's main tsx file rather than the default one. 
    //keeps the original tsx file for reference. So: CurrentTileMap, MasterTileMap. where currentTileMap is given a name related to the current save game probably so it's not being overwritten every time a new game is made.

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

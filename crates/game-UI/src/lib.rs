



///Contains all states of the UI that we will run through in game which alters the main UI
#[derive(Debug, Clone)]
pub enum UIState {
    EscapeMenu,
    SaveMenu,
    LoadMenu,
    OptionsMenu,
    ShopMenu,
    UpgradeMenu,
    InventoryMenu,
    DialogueMenu,
    NormalState,
}

//systems for managing the UI. not important for now



///If game is paused, all other systems must pause as well, if unpaused, gamestate continues from current position
pub fn handle_game_state() {

} 








#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

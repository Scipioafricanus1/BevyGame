







//List of all game entities and their blueprints


///Components to add: 
/// CharacterType: Human
/// Inventory
/// CombatStats
/// Weapon : Assault Rifle(magazine ammo: 10)
struct PlayerEntity;


///Components:
/// CharacterType: Any
/// Inventory
/// CombatStats
/// Weapon
/// MeleeAI or RangedAI
/// isHostile{false or true, depending on scene}  //some entities aren't aware of you at first, so they aren't currently hostile, but sounds will alert them.
struct EnemyEntity;


///Components:
/// CharacterType: Any
/// Inventory
/// CombatStats
/// Weapon
/// MeleeAI or RangedAI if it applies to them
/// isNPCHostile(false or true depending on NPC system that determines when you become a threat to them in particular)
/// Dialogue ( array of strings for what it wants to say to you, along with the responses you can choose to respond to.)
struct NPCEntity;
/*so a entity would have components that decide Whether it is a player or not. Whether it's hostile or not. Whether it's a human, or something else. 
its inventory,
*/


//when you wanna have things collide together.
struct Interactable;






























#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

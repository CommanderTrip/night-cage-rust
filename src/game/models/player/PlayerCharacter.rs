use crate::models::PlayerStates::PlayerStates;

pub struct PlayerCharacter {
    state: PlayerStates,
    has_key: bool,
}

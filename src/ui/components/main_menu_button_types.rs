use bevy::prelude::Component;

pub enum MainMenuButtonType {
    Start,
    Options,
    About,
    Exit
}

#[derive(Component)]
pub struct MainMenuButton {
    pub button_type: MainMenuButtonType
}
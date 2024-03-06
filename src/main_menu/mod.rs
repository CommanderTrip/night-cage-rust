mod styles;
mod systems;

use crate::ui::components::main_menu_button::handle_main_menu_buttons;
use bevy::prelude::*;

struct MyMainMenuPlugin {}

impl Plugin for MyMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_main_menu_buttons);
    }
}

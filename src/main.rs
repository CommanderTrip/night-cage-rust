mod main_menu;
mod ui;

use crate::ui::components::main_menu_button::handle_main_menu_buttons;
use crate::ui::views::main_menu::main_menu;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Night Cage - Rust".into(),
                        resolution: (1200.0, 700.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_main_menu_buttons)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // TODO: Can the main menu be made into a plugin instead? Is it worth it/preferred?
    main_menu(&mut commands, &asset_server);
}

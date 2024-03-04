mod ui;

use crate::ui::components::button::handle_button;
use crate::ui::views::main_menu::main_menu;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Backquote)),
        )
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_button)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // TODO: Can the main menu be made into a plugin instead? Is it worth it/preferred?
    main_menu(&mut commands, &asset_server);
}

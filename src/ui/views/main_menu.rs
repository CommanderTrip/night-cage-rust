use crate::ui::components::button::{primary_button_style, primary_button_text};
use crate::ui::styles::colors::CustomColors;
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::prelude::{
    default, AlignItems, BuildChildren, Commands, FlexDirection, ImageBundle, JustifyContent,
    NodeBundle, Res, Style, UiImage, UiRect, Val,
};

pub fn main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        // Main container
        .spawn((
            Name::new("Main UI"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn((
                Name::new("Main Menu Image"),
                ImageBundle {
                    image: UiImage::new(asset_server.load("tnc-title-no-text.png")),
                    ..default()
                },
            ));

            parent
                .spawn((
                    Name::new("Main Menu Button Holder"),
                    NodeBundle {
                        background_color: CustomColors::BLACK.into(),
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Auto),
                            row_gap: Val::Vh(2.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((Name::new("Start Button"), primary_button_style()))
                        .with_children(|parent| {
                            parent.spawn(primary_button_text(asset_server, "Start"));
                        });

                    parent
                        .spawn((Name::new("Options Button"), primary_button_style()))
                        .with_children(|parent| {
                            parent.spawn(primary_button_text(asset_server, "Options"));
                        });

                    parent
                        .spawn((Name::new("About Button"), primary_button_style()))
                        .with_children(|parent| {
                            parent.spawn(primary_button_text(asset_server, "About"));
                        });

                    parent
                        .spawn((Name::new("Exit Button"), primary_button_style()))
                        .with_children(|parent| {
                            parent.spawn(primary_button_text(asset_server, "Exit"));
                        });
                });
        });
}

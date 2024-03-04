use crate::ui::components::main_menu_button_types::{MainMenuButton, MainMenuButtonType};
use crate::ui::styles::colors::CustomColors;
use bevy::prelude::*;
use bevy::utils::default;

pub fn primary_button_style() -> ButtonBundle {
    // TODO: How do I get rounded corners?
    ButtonBundle {
        background_color: CustomColors::BLACK.into(),
        border_color: CustomColors::TAN.into(),
        style: Style {
            ..Default::default()
        },
        ..default()
    }
}

pub fn primary_button_text(asset_server: &Res<AssetServer>, text: &str) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load("fonts/Labrada-Regular.ttf"),
            font_size: 32.0,
            ..default()
        },
    )
}

#[allow(clippy::type_complexity)]
pub fn handle_button(
    mut interaction_query: Query<
        (&Interaction, &mut Style, &Children, &MainMenuButton),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut style, children, button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => match button.button_type {
                MainMenuButtonType::Start => {
                    println!("Start");
                }
                MainMenuButtonType::Options => {
                    println!("Options");
                }
                MainMenuButtonType::About => {
                    println!("About");
                }
                MainMenuButtonType::Exit => {
                    println!("Exit");
                }
            },

            // FIXME: When changing fonts, the size of the text field changes too. If you put the
            //  cursor in the right spot, you can flicker between NONE and HOVERED and it looks bad.
            Interaction::Hovered => {
                style.border = UiRect::bottom(Val::Px(2.0));
                text.sections[0].style.font = asset_server.load("fonts/Labrada-Italic.ttf")
            }
            Interaction::None => {
                style.border = UiRect::bottom(Val::Px(0.0));
                text.sections[0].style.font = asset_server.load("fonts/Labrada-Regular.ttf")
            }
        }
    }
}

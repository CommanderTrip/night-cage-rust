use crate::ui::components::main_menu_button_types::{MainMenuButton, MainMenuButtonType};
use crate::ui::styles::colors::CustomColors;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::default;

pub fn primary_button_style() -> ButtonBundle {
    // TODO: How do I get rounded corners?
    ButtonBundle {
        background_color: CustomColors::BLACK.into(),
        border_color: CustomColors::TAN.into(),
        style: Style {
            min_width: Val::Percent(50.0),
            justify_content: JustifyContent::Center,
            ..default()
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
pub fn handle_main_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut Style, &Children, &MainMenuButton),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    asset_server: Res<AssetServer>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut style, children, button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => match button.button_type {
                MainMenuButtonType::Start => {
                    println!("Start");
                }
                MainMenuButtonType::Options => navigate_to_options(),
                MainMenuButtonType::About => navigate_to_about(),
                MainMenuButtonType::Exit => {
                    exit.send(AppExit);
                }
            },

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
fn navigate_to_options() {
    println!("op");
}

fn navigate_to_about() {
    println!("ab");
}

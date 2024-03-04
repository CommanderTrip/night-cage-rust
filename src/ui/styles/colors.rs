use bevy::prelude::Color;

pub struct CustomColors(Color);

impl CustomColors {
    pub const BLACK: Color = Color::rgba(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0, 1.0);
    pub const TAN: Color = Color::rgba(190.0 / 255.0, 169.0 / 255.0, 125.0 / 255.0, 1.0);
}

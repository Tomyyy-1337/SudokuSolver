use nannou::{color, prelude::*};

#[derive(Clone, Copy, Default)]
pub enum ThemeType {
    Light,
    #[default]
    Dark,
    Discord,
}

impl ThemeType {
    pub fn next(&self) -> Self {
        match self {
            ThemeType::Dark => ThemeType::Discord,
            ThemeType::Discord => ThemeType::Light,
            ThemeType::Light => ThemeType::Dark,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            ThemeType::Light => "Light",
            ThemeType::Dark => "Dark",
            ThemeType::Discord => "Discord",
        }
    }
}

#[derive(Default)]
pub struct Theme {
    pub theme_type: ThemeType,
    pub primary_color: rgb::Rgb<color::encoding::Srgb, u8>,
    pub secondary_color: rgb::Rgb<color::encoding::Srgb, u8>,
    pub tile_color: rgb::Rgb<color::encoding::Srgb, u8>,
    pub background_color: rgb::Rgb<color::encoding::Srgb, u8>,
    pub theme_alpha: u8,
}

impl Theme {
    pub fn update(&mut self, theme: ThemeType) {
        self.theme_type = theme;
        match theme {
            ThemeType::Light => {
                self.primary_color = color::BLACK;
                self.secondary_color = color::Rgb8::from_components((120, 120, 120));
                self.tile_color = color::Rgb8::from_components((242, 243, 245));
                self.background_color = color::Rgb8::from_components((245, 245, 245));
                self.theme_alpha = 60;
            }
            ThemeType::Dark => {
                self.primary_color = color::WHITE;
                self.secondary_color = color::GREY;
                self.tile_color = color::BLACK;
                self.background_color = color::BLACK;
                self.theme_alpha = 15;
            }
            ThemeType::Discord => {
                self.primary_color = color::Rgb8::from_components((242, 243, 245));
                self.secondary_color = color::Rgb8::from_components((181, 186, 193));
                self.tile_color = color::Rgb8::from_components((43, 45, 49));
                self.background_color = color::Rgb8::from_components((30, 31, 34));
                self.theme_alpha = 20;
            }
        }
    }

    pub fn next(&mut self) {
        self.update(self.theme_type.next());
    }
}

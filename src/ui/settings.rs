use std::path::PathBuf;

use iced::alignment::Horizontal;
use iced::widget::text::Style;
use iced::widget::{image, center, column, horizontal_rule, vertical_rule, horizontal_space, radio, row, text, Radio};
use iced::{font, Length};
use iced_aw::iced_fonts;
use serde::de;

use crate::{error, user};
use crate::error::{error, function_message};

use super::Tab;

const TITLE: &str = "Settings"; 
const ICON:  char = '\u{E995}';

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ThemeChanged(crate::user::SettingTheme),
    TabPositionChanged(crate::user::SettingTabPosition)
}

impl Into<crate::Message> for SettingsMessage {
    fn into(self) -> crate::Message {
        crate::Message::UI(super::UIMessage::Settings(self))
    }
}

#[derive(Default)]
pub struct Settings {
    pub user_data:     crate::user::UserData,
    pub user_settings: crate::user::Settings,
}

impl Settings {

    // Zasad primamo iz `users` datoteke, baza podataka je za 2. sprint
    pub fn from_database(user_entry: &crate::user::UserEntry) -> Result<Self, error::Error> {

        let user_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(PathBuf::from("users"))
            .join(PathBuf::from(user_entry.username.clone()));

        let user_data_file = user_folder.clone().join("user.toml");
        let user_settings_file = user_folder.clone().join("settings.toml");

        if user_data_file.exists() == false {
            return error!(
                name: "Cannot find `user.toml`",
                message: format!("No `user.toml` was found for username `{}`", user_entry.username)
            );
        }

        let user_data_contents_raw = match std::fs::read_to_string(user_data_file) {
            Ok(raw) => raw,
            Err(err) => return error!(
                name: "Failed to read `user.toml`",
                message: function_message!("std::fs::read_to_string", err.to_string())
            )
        };

        let mut user_data_contents = match toml::from_str::<crate::user::UserData>(&user_data_contents_raw) {
            Ok(contents) => contents,
            Err(err) => return error!(
                name: "Failed to parse `user.toml`",
                message: function_message!("toml::from_str", err.message())
            )
        };

        let user_settings_contents = if user_settings_file.exists() == false {
            crate::user::Settings::default()
        } else {
            let user_settings_contents_raw = match std::fs::read_to_string(user_settings_file) {
                Ok(raw) => raw,
                Err(err) => return error!(
                    name: "Failed to read `user.toml`",
                    message: function_message!("std::fs::read_to_string", err.to_string())
                )
            };
            match toml::from_str::<crate::user::Settings>(&user_settings_contents_raw) {
                Ok(contents) => contents,
                Err(err) => return error!(
                    name: "Failed to parse `user.toml`",
                    message: function_message!("toml::from_str", err.message())
                )
            }
        };

        user_data_contents.username = user_entry.username.clone();

        Ok(Self {
            user_data: user_data_contents,
            user_settings: user_settings_contents,
        })

    }
 
}

impl Tab for Settings {

    type Message = crate::Message;

    fn title(&self) -> String {
        format!("{}: {} {}", TITLE, self.user_data.name, self.user_data.surname)
    }

    fn inner_title(&self) -> iced::Element<'_, Self::Message> {
        row!(
            center(text!("App settings...").size(26)).height(Length::Shrink),
            center(text!("User information...").size(26)).height(Length::Shrink)
        ).into()
    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        iced_aw::TabLabel::IconText(ICON, self.title())
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {

        let user_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(PathBuf::from("users"))
            .join(PathBuf::from(self.user_data.username.clone()));


        center(
            row!(
                center(
                    column!(
                        row!(
                            text!("Theme: ").font(
                                font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..font::Font::default()
                                }
                            ),
                            horizontal_space(),
                            column!(
                                row!(
                                    text!("Dark "),
                                    radio(
                                        "", 
                                        crate::user::SettingTheme::Dark, 
                                        Some(self.user_settings.theme.clone()), 
                                        |message| {SettingsMessage::ThemeChanged(message).into()}
                                    ),
                                ),
                                row!(
                                    text!("Light "),
                                    radio(
                                        "", 
                                        crate::user::SettingTheme::Light, 
                                        Some(self.user_settings.theme.clone()), 
                                        |message| {SettingsMessage::ThemeChanged(message).into()}
                                    )
                                ),
                            ).align_x(Horizontal::Right)
                        ),
                        horizontal_rule(2), text!(" "),

                        row!(
                            text!("Tab position: ").font(
                                font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..font::Font::default()
                                }
                            ),
                            horizontal_space(),
                            column!(
                                row!(
                                    text!("Top "),
                                    radio(
                                        "", 
                                        crate::user::SettingTabPosition::Top, 
                                        Some(self.user_settings.tab_bar_position.clone()), 
                                        |message| {SettingsMessage::TabPositionChanged(message).into()}
                                    ),
                                ),
                                row!(
                                    text!("Bottom "),
                                    radio(
                                        "", 
                                        crate::user::SettingTabPosition::Bottom, 
                                        Some(self.user_settings.tab_bar_position.clone()), 
                                        |message| {SettingsMessage::TabPositionChanged(message).into()}
                                    )
                                ),
                            ).align_x(Horizontal::Right)
                        ),
                        horizontal_rule(2), text!(" "),

                    ).width(Length::Fixed(256.0)),   
                ),
                vertical_rule(2),
                center(
                    column!(
                        if self.user_data.user_image.is_some() {
                            column!(
                                image(user_folder.clone().join(self.user_data.user_image.clone().unwrap())),
                                text!(" ")
                            )
                        } else {
                            column!()
                        },
                        horizontal_rule(2),
                        row!(
                            text!("Username: ").font(
                                font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Default::default()
                                }
                            ),
                            horizontal_space(),
                            text!("{}", self.user_data.username).font(
                                font::Font {
                                    family: font::Family::Monospace,
                                    ..Default::default()
                                }
                            )
                        ),
                        row!(
                            text!("Name: ").font(
                                font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Default::default()
                                }
                            ),
                            horizontal_space(),
                            text!("{}", self.user_data.name)
                        ),
                        row!(
                            text!("Surname: ").font(
                                font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Default::default()
                                }
                            ),
                            horizontal_space(),
                            text!("{}", self.user_data.surname)
                        ),
                        row!(
                            text!("Privilege: ").font(
                                font::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Default::default()
                                }
                            ),
                            horizontal_space(),
                            text!("{}", self.user_data.privilege)
                        ),
                        horizontal_rule(2)
                    ).width(Length::Fixed(256.0))
                )
            )
        ).into()
    }

}
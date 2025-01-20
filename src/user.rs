use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone)]
#[derive(Deserialize)]
pub struct UserEntry {

    #[serde(skip_deserializing)]
    pub username: String,

    pub md5:  String,
    pub salt: String

}

impl UserEntry {

    pub fn is_equal(&self, other: &Self) -> bool {
        if self.username == other.username
        && self.md5      == other.md5 {
            true
        } else {
            false
        }
    }

    pub fn calculate(&mut self, password: String, salt: String) {
        self.salt = salt.clone();
        self.md5 = format!("{:x}", md5::compute(format!("{password}{salt}")));
    }

}

#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum UserPrivilege {
    Basic,
    #[default]
    Admin
}

impl std::fmt::Display for UserPrivilege {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserPrivilege::Admin => write!(f, "Admin"),
            UserPrivilege::Basic => write!(f, "Basic")
        }
    }

}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct UserData {

    #[serde(skip_deserializing, skip_serializing)]
    pub username:   String,
    
    #[serde(skip_deserializing, skip_serializing)]
    pub privilege: UserPrivilege,

    pub name:       String,
    pub surname:    String,
    pub user_image: Option<PathBuf>,

}

impl Default for UserData {

    fn default() -> Self {
        Self { 
            username: "josko".into(), 
            privilege: UserPrivilege::Admin, 
            name: "Joško".into(), 
            surname: "Križanović".into(), 
            user_image: Some("image.qoi".into())
        }
    }

}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum SettingTheme {
    #[default]
    Light,
    Dark,
}

impl Into<iced::Theme> for SettingTheme {

    fn into(self) -> iced::Theme {
        match self {
            Self::Dark => iced::Theme::Dark,
            Self::Light => iced::Theme::Light,
        }
    }

}


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum SettingTabPosition {
    #[default]
    Top,
    Bottom,
}

impl Into<iced_aw::TabBarPosition> for SettingTabPosition {

    fn into(self) -> iced_aw::TabBarPosition {
        match self {
            Self::Top => iced_aw::TabBarPosition::Top,
            Self::Bottom => iced_aw::TabBarPosition::Bottom
        }
    }

}

#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub theme:        SettingTheme,
    pub tab_bar_position: SettingTabPosition
}
use std::{default, path::PathBuf};

use iced::{
    alignment::{Horizontal, Vertical}, theme, widget::{button, center, column, container, text, text_input}, window, Alignment::{self, Center}, Element, Length, Renderer, Task, Theme
};

use crate::user::UserEntry;
use crate::Message;

#[derive(Default, Debug, Clone)]
pub struct Login {

    state:    LoginState,
    entry:    UserEntry,
    password: String,
    issue:    String,

    pub found_entry: Option<UserEntry>

}

#[derive(Debug, Clone, Default)]
enum LoginState {
    #[default]
    AwaitingUser,   // Waiting for the user to finish writing pass and username.
    AwaitingServer, // Waiting for a response from the server.
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameFieldChanged(String),
    PasswordFieldChanged(String),
    LoginButtonPressed,
}

impl Into<crate::Message> for LoginMessage {
    fn into(self) -> crate::Message {
        crate::Message::Login(self)
    }
}

impl Login {
    pub fn set_username(&mut self, username: String) {
        self.entry.username = username;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn find_match_and_login(&mut self) -> Result<UserEntry, String> {

        let found_entry = match Login::fetch_username_from_database(&self.entry.username) {
            Ok(entry) => entry,
            Err(err) => return Err(err)
        };

        self.entry.calculate(self.password.clone(), found_entry.salt.clone());

        if found_entry.is_equal(&self.entry) {
            return Ok(self.entry.clone())
        } else {
            return Err(format!("Passwords do not match!"))
        }

    }

    pub fn fetch_username_from_database(username: &str) -> Result<UserEntry, String> {

        // for now just my users directory :p
        let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("users")
            .join(format!("{username}.toml"));

        match file.exists() {
            true => match std::fs::read_to_string(file) {
                Ok(data) => match toml::from_str::<UserEntry>(data.as_str()) {
                    Ok(found_entry) => Ok(UserEntry { 
                        username: username.to_owned(), 
                        md5: found_entry.md5, 
                        salt: found_entry.salt 
                    }),
                    Err(err) => Err(format!("User '{username}' contains invalid data!\n{}", err.message()))
                },
                Err(_) => Err(format!("Failed to read '{username}.toml'."))
            },
            false => Err(format!("User '{username}' does not exist."))
        } 
        


    }

    pub fn update(&mut self, message: LoginMessage) -> Task<Message> {
        match message {
            LoginMessage::UsernameFieldChanged(username) => self.set_username(username),
            LoginMessage::PasswordFieldChanged(password) => self.set_password(password),
            LoginMessage::LoginButtonPressed => {
                self.issue = String::new();
                self.state = LoginState::AwaitingServer;
                match self.find_match_and_login() {
                    Ok(found_entry) => self.found_entry = Some(found_entry),
                    Err(err) => {
                        self.issue = err;
                        self.state = LoginState::AwaitingUser;
                    }
                }
            },
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        
        let col: iced::widget::Column<'_, Message, Theme, Renderer> = column![
            text("Enter your credentials:")
                .align_x(Horizontal::Center)
                .width(Length::Fill),
            text_input("Username...", &self.entry.username)
                .size(20)
                .on_input_maybe(match self.state.clone() {
                    LoginState::AwaitingUser =>
                        Some(|s| { LoginMessage::UsernameFieldChanged(s).into() }),
                    LoginState::AwaitingServer => None,
                }),
            text_input("Password...", &self.password)
                .size(20)
                .secure(true)
                .on_input_maybe(match self.state.clone() {
                    LoginState::AwaitingUser =>
                        Some(|s| { LoginMessage::PasswordFieldChanged(s).into() }),
                    LoginState::AwaitingServer => None,
                }),
        ].push_maybe(match self.state.clone() {
            LoginState::AwaitingUser => Some(
                button("Login")
                    .on_press(LoginMessage::LoginButtonPressed.into())),
            _ => None
        })
        .push_maybe(match self.state.clone() {
            LoginState::AwaitingServer => Some(
                text("Please wait while we fetch your data...")),
            _ => None
        })
        .push_maybe(match self.state.clone() {
            LoginState::AwaitingUser => match self.issue.is_empty() {
                false => Some(text(format!("{}", self.issue))),
                _ => None
            },
            _ => None
        })
        .padding(20)
        .spacing(20)
        .align_x(Horizontal::Center);

        center(container(col)
            .height(Length::Shrink)
            .width(Length::Fixed(400.0))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(container::rounded_box)
        ).into()

    }
}

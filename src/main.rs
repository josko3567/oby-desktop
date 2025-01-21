mod login;
mod ui;
mod table;
mod user;
mod error;
mod shared;

use std::process::exit;

use clap::{command, arg};
use iced::{Element, Task, Theme};

const ICON_BYTES: &[u8] = include_bytes!("../fonts/icomoon.ttf");

const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

use crate::login::{
    LoginMessage, 
    Login
};
use crate::ui::{
    UI,
    UIMessage
};

use crate::ui::settings::Settings;

/// Messages from this file or other files.
/// 
/// These are sent back from UI (usually) when the user 
/// interacts with a element via. the various `view()` 
/// functions inside this and the rest of the files.
/// 
/// These are then received by a `update()` from this
/// file or another to decide what to do with the message.
#[derive(Debug, Clone)]
enum Message {
    Login(LoginMessage),
    UI(UIMessage)
}

/// What part of the UI are we currently on.
/// The main part is UI but you need to Login
/// first.
/// 
/// #[default] sets what part of the UI to load first.
#[derive(Default, PartialEq, Eq)]
enum Page {
    #[default]
    Login,
    UI
}

/// The data of the various parts of the UI.
#[derive(Default)]
struct Parts {
    pub login: Login,
    pub ui: UI
}

/// All the app data, including the current user,
/// current part of the UI we are on, theme and
/// probably more to come.
#[derive(Default)]
struct App {
    pub page:  Page,
    pub part:  Parts,
}

impl App {

    fn new() -> App {
        Self {
            part:  Parts::default(),
            page:   Page::default(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(login_message) => {
                let task = Login::update(&mut self.part.login, login_message);
                if self.part.login.found_entry.is_some() {
                    self.page = Page::UI;
                    let user_entry = self.part.login.found_entry.clone().unwrap();
                    self.part.ui.settings = match Settings::from_database(&user_entry) {
                        Ok(user_data_settings) => user_data_settings,
                        Err(err) => {
                            eprintln!("{}", err);
                            exit(EXIT_FAILURE);
                        }
                    };
                }
                return task;
            }

            Message::UI(ui_message) => return UI::update(&mut self.part.ui, ui_message),
        }
    }

    fn view(&self) -> Element<Message> {

        match self.page {
            Page::Login  => return Login::view(&self.part.login).into(),
            Page::UI     => return    UI::view(&self.part.ui).into()
        };

    }

    fn theme(&self) -> Theme {
        self.part.ui.settings.user_settings.theme.clone().into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        // Combine subscriptions if there are multiple tabs
        if self.page == Page::UI
        && self.part.ui.active_tab == crate::ui::UITabID::Orders {
            return self.part.ui.orders.subscription()
        } else {
            return iced::Subscription::none()
        }
    }

    fn title(&self) -> String {
        "OBY Desktop".to_string()
    }

    // fn title()

}

fn main() -> iced::Result {

    env_logger::init();

    if cfg!(debug_assertions) {

        let matches = command!()
            .arg(arg!(ui: --ui "Debug: Display the UI (skip login)"))
            .get_matches();
    
        let app: App = if matches.get_flag("ui") == true {
            let mut app = App::new();
            app.page = Page::UI;
            app
        } else {
            App::new()
        };

        iced::application(App::title, App::update, App::view)
            .theme(App::theme)
            .centered()
            .font(ICON_BYTES)
            .subscription(App::subscription)
            .run_with(|| (app, Task::none()))

    } else {

        iced::application(App::title, App::update, App::view)
            .theme(App::theme)
            .centered()
            .font(ICON_BYTES)
            .subscription(App::subscription)
            .run_with(|| (App::new(), Task::none()))

    }
}

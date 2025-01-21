use crate::shared::req_resp;
use crate::table::{self, VirtualTable};
use crate::Message;
use iced::{widget::{center, container, pane_grid::{self, Axis, Pane}, text, Column}, Alignment::Center, Color, Element, Font, Length::Fill, Task, Theme};
use iced_aw::{direction::{Horizontal, Vertical}, style::tab_bar::dark, tab_bar::{self, Style}, tabs::tab_bar_position, TabBarPosition, TabLabel, Tabs};

// use crate::server_old as server;


pub mod settings;
pub mod orders;
pub mod vtable;
// pub mod items;

use settings::{
    Settings,
    SettingsMessage
};


use orders::{
    OrderList,
    OrderListMessage
};

use vtable::{
    VirtualTableManager,
    VirtualTableManagerMessage
};

// use items::{
//     ItemManagerMessage
// }


const ICON: Font = Font::with_name("icomoon");

#[derive(Default)]
enum UIState {
    #[default]
    TabScreen
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum UITabID {
    #[default]  
    Settings,
    Orders,
    VirtualTableManager
}

#[derive(Debug, Clone)]
pub enum UIMessage {
    TabSelected(UITabID),
    Settings(SettingsMessage),
    Orders(OrderListMessage),
    VirtualTableManager(VirtualTableManagerMessage)
}

impl Into<crate::Message> for UIMessage {
    fn into(self) -> crate::Message {
        crate::Message::UI(self)
    }
}

#[derive(Default)]
pub struct UI {
    pub tables: Vec<VirtualTable>,
    pub active_tab: UITabID,
    pub vtable: VirtualTableManager,
    pub settings: Settings,
    pub orders: OrderList
}

pub fn tab_style(theme: &Theme, status: iced_aw::style::status::Status) -> Style {
    // _theme.palette().
    let mut base = Style {
        tab_label_background: iced::Background::Color(theme.palette().background.into()),
        tab_label_border_color: theme.palette().text,
        icon_color: theme.palette().text,
        text_color: theme.palette().text,
        ..Default::default()
    };

    match status {
        iced_aw::style::Status::Active | iced_aw::style::Status::Selected => {
            base.tab_label_background = iced::Background::Color(theme.extended_palette().primary.strong.color.scale_alpha(1.0));
            base.text_color = theme.extended_palette().primary.weak.text.scale_alpha(1.0);
            base.icon_color = theme.extended_palette().primary.weak.text.scale_alpha(1.0);
        },
        iced_aw::style::Status::Disabled => {
            base.tab_label_background = iced::Background::Color(theme.extended_palette().secondary.base.color);
            base.text_color = theme.extended_palette().secondary.base.text;
            base.icon_color = theme.extended_palette().secondary.base.text;
        },
        iced_aw::style::Status::Hovered => {
            base.tab_label_background = iced::Background::Color(theme.extended_palette().primary.base.color);
            base.text_color = theme.extended_palette().primary.base.text;
            base.icon_color = theme.extended_palette().primary.base.text;
        }
        _ => {}
    }

    // if status == iced_aw::style::status::Status::Disabled {
    //     base.tab_label_background = Background::Color([0.13, 0.13, 0.13].into());
    // }

    base
}


impl UI {

    pub fn update(&mut self, message: UIMessage) -> Task<Message> {
        match message {
            UIMessage::TabSelected(tab_id) => {
                self.active_tab = tab_id.clone();
                match tab_id {
                    UITabID::Orders => {
                        let mut request = req_resp::Request {
                            kind: req_resp::RequestKind::OffersTables,
                            payload: None
                        };
                        return Task::perform(
                            async move {request.send_request("".to_string()).await}, 
                            |value| {OrderListMessage::FetchedVirtualTablesAndItems(value).into()}
                        )
                    },
                    UITabID::VirtualTableManager => {
                        let mut request = req_resp::Request {
                            kind: req_resp::RequestKind::Tables,
                            payload: None
                        };
                        return Task::perform(
                            async move {request.send_request("".to_string()).await}, 
                            |value| {VirtualTableManagerMessage::FetchedVirtualTables(value).into()}
                        )
                    },
                    _ => {}
                }
            }
            UIMessage::Settings(setting_message) => {
                match setting_message {
                    SettingsMessage::ThemeChanged(theme) => {
                        self.settings.user_settings.theme = theme;
                    },
                    SettingsMessage::TabPositionChanged(tab_bar_position) => {
                        self.settings.user_settings.tab_bar_position = tab_bar_position
                    },
                }
            }
            UIMessage::Orders(orders_message) => {
                let task = OrderList::update(&mut self.orders, orders_message);
                return task;
            },
            UIMessage::VirtualTableManager(message) => {
                let task = VirtualTableManager::update(&mut self.vtable, message);
                return task;
            }
        }

        return Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {

        Tabs::new(|id| UIMessage::TabSelected(id).into())
            .tab_icon_position(iced_aw::tabs::Position::Bottom)
            .push(
                UITabID::Settings,
                self.settings.tab_label(),
                self.settings.view(),
            )
            .push(
                UITabID::Orders,
                self.orders.tab_label(),
                self.orders.view(),
            )
            .push(
                UITabID::VirtualTableManager,
                self.vtable.tab_label(),
                self.vtable.view(),
            )
            .set_active_tab(&self.active_tab)
            .icon_font(ICON)
            .tab_bar_style(Box::new(tab_style))
            .tab_bar_position(self.settings.user_settings.tab_bar_position.clone().into())
            .into()
            
    }

}


trait Tab {
    type Message;

    fn title(&self) -> String;

    fn inner_title(&self) -> Element<'_, Self::Message>;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(self.inner_title())
            .push(self.content())
            .align_x(iced::Alignment::Center);

        container(column)
            .width(Fill)
            .height(Fill)
            .align_x(Center)
            .align_y(Center)
            .padding(16)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
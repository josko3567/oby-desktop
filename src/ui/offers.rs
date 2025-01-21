use clap::builder::Str;
use iced::{advanced::widget::operation::text_input, border::Radius, widget::{button, center, container, row, scrollable, text, Column}, Border, Length, Task, Theme};
use serde_json::json;


use crate::shared::{dbt as dbt, req_resp};
use super::Tab;

const TITLE: &str = "Virtual Tables"; 
const ICON:  char = '\u{e9ba}';

#[derive(Debug, Clone)]
pub enum VirtualTableManagerMessage {
    FetchedOffers(Result<serde_json::Value, String>),
    DeleteOffers(dbt::VirtualTableID),
    DeleteOffersPost(Result<serde_json::Value, String>),
    AddOffers(dbt::VirtualTableID),
    AddOffersPost(Result<serde_json::Value, String>),
    TextInputedName(String),
    TextInputedDescription(String),
    TextInputedPrice(String),
}

impl Into<crate::Message> for VirtualTableManagerMessage {
    fn into(self) -> crate::Message {
        crate::Message::UI(super::UIMessage::VirtualTableManager(self))
    }
}


pub struct VirtualTableManager {
    pub fetch_vtables: Result<Vec<dbt::VirtualTable>, String>,
    pub table_name_text_input: String
}

impl Default for VirtualTableManager {
    fn default() -> Self {
        Self { 
            fetch_vtables: Err("Fetching data...".to_string()),
            table_name_text_input: String::new()
        }
    }
}



impl VirtualTableManager {

    pub fn update(&mut self, message: VirtualTableManagerMessage) -> Task<crate::Message> {

        match message {
            VirtualTableManagerMessage::FetchedVirtualTables(response) => {
                if response.is_ok() {
                    match serde_json::from_value::<req_resp::TablesResponseData>(response.unwrap()) {
                        Ok(payload) => self.fetch_vtables = Ok(payload.tables),
                        Err(err) => self.fetch_vtables = Err(err.to_string())
                    }
                } else {
                    self.fetch_vtables = Err(response.unwrap_err().to_string())
                }
            },
            VirtualTableManagerMessage::DeleteVirtualTable(table) => {
                let mut request = req_resp::Request {
                    kind: req_resp::RequestKind::TablesDelete,
                    payload: None
                };
                return Task::perform(
                    async move {request.send_request(table).await}, 
                    |value| {
                        VirtualTableManagerMessage::DeleteVirtualTablePost(value).into()}
                )
            },
            VirtualTableManagerMessage::DeleteVirtualTablePost(result) => {
                if result.is_ok() {
                    let mut request = req_resp::Request {
                        kind: req_resp::RequestKind::Tables,
                        payload: None
                    };
                    return Task::perform(
                        async move {request.send_request("".to_string()).await}, 
                        |value| {VirtualTableManagerMessage::FetchedVirtualTables(value).into()}
                    )
                }
            },
            VirtualTableManagerMessage::AddVirtualTable(name) => {
                if name.is_empty() {
                    return Task::none()
                }
                let mut request = req_resp::Request {
                    kind: req_resp::RequestKind::TablesInsert,
                    payload: Some(serde_json::to_value(req_resp::TablesInsertRequestData {
                        table: dbt::VirtualTable { name, order_count: 0 }
                    }).unwrap())
                };
                return Task::perform(
                    async move {request.send_request("".to_string()).await}, 
                    |value| {VirtualTableManagerMessage::AddVirtualTablePost(value).into()}
                )
            },
            VirtualTableManagerMessage::AddVirtualTablePost(result) => {
                if result.is_ok() {
                    self.table_name_text_input = String::new();
                    let mut request = req_resp::Request {
                        kind: req_resp::RequestKind::Tables,
                        payload: None
                    };
                    return Task::perform(
                        async move {request.send_request("".to_string()).await}, 
                        |value| {
                            VirtualTableManagerMessage::FetchedVirtualTables(value).into()
                        }
                    )
                }
            }
            VirtualTableManagerMessage::TextInputed(text) => {self.table_name_text_input = text}
        }
        Task::none()

    }

}

fn virtual_table_button_style(theme: &Theme, status: iced::widget::button::Status) -> button::Style {

    let mut table_button_style = iced::widget::button::Style { 
        text_color: theme.extended_palette().primary.weak.text, 
        background: Some(iced::Background::Color(theme.extended_palette().primary.weak.color)), 
        border: Border { 
            color: theme.extended_palette().primary.base.color, 
            width: 0.0, 
            radius: Radius::new(7)
        }, 
        shadow: iced::Shadow { 
            color: theme.extended_palette().secondary.weak.color,
            offset: iced::Vector::new(0.0, 0.0), 
            blur_radius: 0.0
        }
    };

    match status {
        iced::widget::button::Status::Active => {
            table_button_style.text_color = theme.extended_palette().primary.base.text; 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.base.color));
        },
        iced::widget::button::Status::Pressed =>{
            table_button_style.text_color = theme.extended_palette().primary.strong.text.inverse(); 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.strong.color.inverse()));
        },
        iced::widget::button::Status::Hovered => {
            table_button_style.text_color = theme.extended_palette().primary.strong.text; 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.strong.color));
        },
        _ => {}
    }


    table_button_style


}

fn virtual_table_button_style_add(theme: &Theme, status: iced::widget::button::Status) -> button::Style {

    let mut table_button_style = iced::widget::button::Style { 
        text_color: theme.extended_palette().primary.weak.text.inverse(), 
        background: Some(iced::Background::Color(theme.extended_palette().primary.weak.color.inverse())), 
        border: Border { 
            color: theme.extended_palette().primary.base.color.inverse(), 
            width: 0.0, 
            radius: Radius::new(7)
        }, 
        shadow: iced::Shadow { 
            color: theme.extended_palette().secondary.weak.color.inverse(),
            offset: iced::Vector::new(0.0, 0.0), 
            blur_radius: 0.0
        }
    };

    match status {
        iced::widget::button::Status::Active => {
            table_button_style.text_color = theme.extended_palette().primary.base.text.inverse(); 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.base.color.inverse()));
        },
        iced::widget::button::Status::Pressed =>{
            table_button_style.text_color = theme.extended_palette().primary.strong.text; 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.strong.color));
        },
        iced::widget::button::Status::Hovered => {
            table_button_style.text_color = theme.extended_palette().primary.strong.text.inverse(); 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.strong.color.inverse()));
        },
        _ => {}
    }


    table_button_style


}

fn vtable_container_style(theme: &Theme) -> iced::widget::container::Style {

    iced::widget::container::Style {
        text_color: Some(theme.extended_palette().secondary.base.text),
        background: Some(iced::Background::Color(theme.extended_palette().secondary.base.color)),
        border: Border { 
            color: theme.extended_palette().primary.base.color, 
            width: 0.0, 
            radius: Radius::new(7)
        }, 
        ..Default::default()
    }

}


impl Tab for VirtualTableManager {

    type Message = crate::Message;

    fn title(&self) -> String {
        format!("{}", TITLE)
    }

    fn inner_title(&self) -> iced::Element<'_, Self::Message> {
        text!("{}", self.title()).size(26).into()
    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        iced_aw::TabLabel::IconText(ICON, self.title())
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {
        
        if self.fetch_vtables.is_err() {
            text!("{}", self.fetch_vtables.clone().expect_err("").to_string()).into()
        } else {

            let mut col: Column<'_, crate::Message> = Column::new().spacing(20).padding(30);

            let vtables = self.fetch_vtables.clone().unwrap();
            for vtable in vtables.iter() {
                col = col.push(
                    container(
                        row![
                            text!("`{}`", vtable.name.clone()),
                            iced::widget::horizontal_space(),
                            button("X")
                                .on_press(VirtualTableManagerMessage::DeleteVirtualTable(vtable.name.clone()).into())
                                .style(virtual_table_button_style)
                        ]
                    )
                    .padding(10)
                    .style(vtable_container_style).width(Length::Fill)
                );
            }
            col = col.push(
                container(
                    row![
                        iced::widget::text_input("Table name", &self.table_name_text_input)
                            .on_input(|s| VirtualTableManagerMessage::TextInputed(s).into()),
                        iced::widget::horizontal_space(),
                        button("+")
                            .on_press(VirtualTableManagerMessage::AddVirtualTable(self.table_name_text_input.clone()).into())
                            .style(virtual_table_button_style_add)
                    ]
                )
                .padding(10)
                .style(vtable_container_style).width(Length::Fill)
            );
        // let client = Client::new();

            center(scrollable(col).width(Length::Fill)).into()

        }

    }

}
use clap::builder::Str;
use iced::{advanced::widget::operation::text_input, border::Radius, widget::{button, center, container, row, column, scrollable, text, Column}, Border, Length, Task, Theme};
use serde_json::json;


use crate::shared::{dbt::{self as dbt, Offer}, req_resp};
use super::Tab;

const TITLE: &str = "Offers"; 
const ICON:  char = '\u{e9ba}';

#[derive(Debug, Clone)]
pub enum OfferManagerMessage {
    FetchedOffers(Result<serde_json::Value, String>),
    DeleteOffers(dbt::VirtualTableID),
    DeleteOffersPost(Result<serde_json::Value, String>),
    AddOffers,
    AddOffersPost(Result<serde_json::Value, String>),
    TextInputedName(String),
    TextInputedDescription(String),
    TextInputedPrice(String),
}

impl Into<crate::Message> for OfferManagerMessage {
    fn into(self) -> crate::Message {
        crate::Message::UI(super::UIMessage::OfferManager(self))
    }
}


pub struct OfferManager {
    pub fetch_offers: Result<Vec<dbt::Offer>, String>,
    pub offer_name_text_input: String,
    pub offer_description_text_input: String,
    pub offer_price_text_input: String,
}

impl Default for OfferManager {
    fn default() -> Self {
        Self { 
            fetch_offers: Err("Fetching data...".to_string()),
            offer_name_text_input: String::new(),
            offer_description_text_input: String::new(),
            offer_price_text_input: String::new(),
        }
    }
}



impl OfferManager {

    pub fn update(&mut self, message: OfferManagerMessage) -> Task<crate::Message> {

        match message {
            OfferManagerMessage::FetchedOffers(response) => {
                if response.is_ok() {
                    match serde_json::from_value::<req_resp::OffersResponseData>(response.unwrap()) {
                        Ok(payload) => self.fetch_offers = Ok(payload.offers),
                        Err(err) => self.fetch_offers = Err(err.to_string())
                    }
                } else {
                    self.fetch_offers = Err(response.unwrap_err().to_string())
                }
            },
            OfferManagerMessage::DeleteOffers(table) => {
                let mut request = req_resp::Request {
                    kind: req_resp::RequestKind::TablesDelete,
                    payload: None
                };
                return Task::perform(
                    async move {request.send_request(table).await}, 
                    |value| {
                        OfferManagerMessage::DeleteOffersPost(value).into()}
                )
            },
            OfferManagerMessage::DeleteOffersPost(result) => {
                if result.is_ok() {
                    let mut request = req_resp::Request {
                        kind: req_resp::RequestKind::Offers,
                        payload: None
                    };
                    return Task::perform(
                        async move {request.send_request("".to_string()).await}, 
                        |value| {OfferManagerMessage::FetchedOffers(value).into()}
                    )
                }
            },
            OfferManagerMessage::AddOffers => {
                if self.offer_name_text_input.is_empty()
                ||  self.offer_price_text_input.is_empty()
                ||  self.offer_description_text_input.is_empty() {
                    return Task::none()
                }

                let (integer, fraction) = {
                    let price = self.offer_price_text_input.clone();
                    let Some((left, right)) = price.split_once(",") else {
                        log::error!("Invalid price!");
                        return Task::none()
                    };
                    let Ok(integer) = left.parse::<u32>() else {
                        log::error!("Invalid integer!");
                        return Task::none()
                    };
                    let Ok(fraction) = right.parse::<u32>() else {
                        log::error!("Invalid fraction!");
                        return Task::none()
                    };
                    if fraction >= 100 {
                        log::error!("Invalid fraction!");
                    }
                    (integer, fraction)
                };

                let mut request = req_resp::Request {
                    kind: req_resp::RequestKind::OffersInsert,
                    payload: Some(serde_json::to_value(req_resp::OffersInsertRequestData {
                        offer: dbt::Offer {
                            name: self.offer_description_text_input.clone(),
                            description: self.offer_description_text_input.clone(), 
                            price_integer: integer,
                            price_fraction: fraction
                        }
                    }).unwrap())
                };
                return Task::perform(
                    async move {request.send_request("".to_string()).await}, 
                    |value| {OfferManagerMessage::AddOffersPost(value).into()}
                )
            },
            OfferManagerMessage::AddOffersPost(result) => {
                if result.is_ok() {
                    self.offer_name_text_input = String::new();
                    self.offer_description_text_input = String::new();
                    self.offer_price_text_input = String::new();
                    let mut request = req_resp::Request {
                        kind: req_resp::RequestKind::Offers,
                        payload: None
                    };
                    return Task::perform(
                        async move {request.send_request("".to_string()).await}, 
                        |value| {
                            OfferManagerMessage::FetchedOffers(value).into()
                        }
                    )
                }
            }
            OfferManagerMessage::TextInputedName(text) => {self.offer_name_text_input = text}
            OfferManagerMessage::TextInputedDescription(text) => {self.offer_description_text_input = text}
            OfferManagerMessage::TextInputedPrice(text) => {self.offer_price_text_input = text}
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


impl Tab for OfferManager {

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
        
        if self.fetch_offers.is_err() {
            text!("{}", self.fetch_offers.clone().expect_err("").to_string()).into()
        } else {

            let mut col: Column<'_, crate::Message> = Column::new().spacing(20).padding(30);

            let offers = self.fetch_offers.clone().unwrap();
            for offer in offers.iter() {
                col = col.push(
                    container(
                        row![
                            column![
                                text!("`{}`", offer.name.clone()),
                                text!("{}", offer.description.clone()),
                                text!("Price: {},{}", offer.price_integer, offer.price_fraction)
                            ],
                            iced::widget::horizontal_space(),
                            button("X")
                                .on_press(OfferManagerMessage::DeleteOffers(offer.name.clone()).into())
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
                        column![
                            
                            iced::widget::text_input("Name", &self.offer_name_text_input)
                                .on_input(|s| OfferManagerMessage::TextInputedName(s).into()),
                            iced::widget::text_input("Description", &self.offer_description_text_input)
                                .on_input(|s| OfferManagerMessage::TextInputedDescription(s).into()),
                            iced::widget::text_input("Price", &self.offer_price_text_input)
                                .on_input(|s| OfferManagerMessage::TextInputedPrice(s).into()),
                            
                        ],
                        iced::widget::horizontal_space(),
                        button("+")
                            .on_press(OfferManagerMessage::AddOffers.into())
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
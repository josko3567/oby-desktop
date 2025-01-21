
use std::{collections::{HashMap, HashSet}, thread::current};

use iced::{
    alignment::Horizontal, border::Radius, widget::{
        button, center, column, container, horizontal_rule, horizontal_space, row, scrollable, text, Column
    }, Border, Length, Task, Theme
};

use crate::{shared::req_resp, Message};
use crate::shared::dbt;
use super::Tab;


const TITLE: &str = "Orders"; 
const ICON:  char = '\u{e9ba}';

#[derive(Debug, Clone)]
pub enum OrderListMessage {
    FetchedVirtualTablesAndItems(Result<serde_json::Value, String>),
    UpdateOrders(Result<serde_json::Value, String>),
    TablePressed(dbt::VirtualTableID),
    FetchedOrders(Result<serde_json::Value, String>),
    FinishOrder(dbt::OrderID),
    PollOrders,
    FetchedOrderAndUnfinished(Result<serde_json::Value, String>),
    PollFetchedTablesWithUnfinishedOrders,
    PollFetchedTablesWithUnfinishedOrdersPost(Result<serde_json::Value, String>),
}


impl OrderList {
    pub fn subscription(&self) -> iced::Subscription<Message> {
        // Trigger `PollOrders` every 3 seconds
        iced::time::every(iced::time::Duration::from_secs(3)).map(|_| {
            OrderListMessage::PollOrders.into() // Map the timer to `PollOrders` message
        })
    }
}

impl Into<crate::Message> for OrderListMessage {
    fn into(self) -> crate::Message {
        crate::Message::UI(super::UIMessage::Orders(self))
    }
}

#[derive(Debug)]
pub struct OrderList {

    pub fetch_vtables: Result<Vec<dbt::VirtualTable>, String>,
    pub current_vtable: Option<dbt::VirtualTableID>,
    pub fetch_orders: Result<Vec<dbt::Order>, String>,
    pub fetch_items: Result<Vec<dbt::Offer>, String>,
    pub unfinished_tables: Result<Vec<dbt::VirtualTableID>, String>

}

impl Default for OrderList {
    fn default() -> Self {
        Self { 
            current_vtable: None,
            fetch_vtables:  Err("Fetching data...".to_string()),
            fetch_orders:   Err("Click on a table first :)".to_string()),
            fetch_items:    Err("Couldn't fetch Item's".to_string()),
            unfinished_tables: Err("Couldn't fetch orders.".to_string())
        }
    }
}


impl OrderList {


    pub fn update(&mut self, message: OrderListMessage) -> Task<Message> {

        match message {
            OrderListMessage::PollFetchedTablesWithUnfinishedOrders => {
                let mut request = req_resp::Request {
                    kind: req_resp::RequestKind::Orders,
                    payload: Some(serde_json::to_value(req_resp::OrdersRequestData {
                        new: true,
                        table: None
                    }).unwrap())                
                };
                return Task::perform(
                    async move {request.send_request("".to_string()).await}, 
                    |result| OrderListMessage::PollFetchedTablesWithUnfinishedOrdersPost(result).into(),
                );
            }
            OrderListMessage::PollFetchedTablesWithUnfinishedOrdersPost(fetch) => {
                let value = match fetch {
                    Ok(value) => value,
                    Err(err) => {
                        self.unfinished_tables = Err(err.to_string());
                        return Task::none();
                    }
                };
                let response = match serde_json::from_value::<req_resp::OrdersResponseData>(value) {
                    Ok(payload) => payload,
                    Err(err) => {
                        self.unfinished_tables = Err(err.to_string());
                        return Task::none()
                    }
                };
                let mut unique = HashSet::new();
                for order in response.orders {
                    unique.insert(order.id.table);
                }
                self.unfinished_tables = Ok(unique.into_iter().collect::<Vec<String>>());
                Task::none()
            }
            OrderListMessage::FetchedVirtualTablesAndItems(fetch) => {
                let value = match fetch {
                    Ok(value) => value,
                    Err(err) => {
                        log::error!("err: {}", err);
                        self.fetch_items = Err("Failed to fetch offers and tables. 1".to_string());
                        self.fetch_vtables = Err("Failed to fetch offers and tables. 1".to_string());
                        return Task::none();
                    }
                };
                let response = match serde_json::from_value::<req_resp::OffersTablesResponseData>(value) {
                    Ok(payload) => payload,
                    Err(err) => {
                        self.fetch_items = Err("Failed to fetch offers and tables. 2".to_string());
                        self.fetch_vtables = Err("Failed to fetch offers and tables. 2".to_string());
                        return Task::none();
                    }
                };
                self.fetch_vtables = Ok(response.tables);
                self.fetch_items = Ok(response.offers);
                // eprintln!("Fetched data: {:#?}", self);
                return Task::perform(
                    async move {}, 
                    |result| OrderListMessage::PollFetchedTablesWithUnfinishedOrders.into(),
                );    
            },
            OrderListMessage::TablePressed(table) => {
                self.current_vtable = Some(table.clone());
                let mut request = req_resp::Request {
                    kind: req_resp::RequestKind::Orders,
                    payload: Some(serde_json::to_value(req_resp::OrdersRequestData {
                        new: true,
                        table: Some(table)
                    }).unwrap())
                };
                return Task::perform(
                    async move {request.send_request("".to_string()).await}, 
                    |result| OrderListMessage::FetchedOrderAndUnfinished(result).into(),
                );
            }
            OrderListMessage::PollOrders => {
                if let Some(current_table) = &self.current_vtable {
                    // Fetch new orders for the currently selected table
                    let table = current_table.clone();
                    let mut request = req_resp::Request {
                        kind: req_resp::RequestKind::Orders,
                        payload: Some(serde_json::to_value(req_resp::OrdersRequestData {
                            new: true,
                            table: Some(table)
                        }).unwrap())
                    };
                    return Task::perform(
                        async move {request.send_request("".to_string()).await}, 
                        |result| OrderListMessage::FetchedOrderAndUnfinished(result).into(),
                    );
                }
                Task::none() // Do nothing if no table is selected
            }
            OrderListMessage::FetchedOrders(orders) => {
                let response = match orders {
                    Ok(value) => value,
                    Err(err) => {
                        self.fetch_orders = Err(err.to_string());
                        return Task::none();
                    }
                };
                match serde_json::from_value::<req_resp::OrdersResponseData>(response) {
                    Ok(payload) => self.fetch_orders = Ok(payload.orders),
                    Err(err) => self.fetch_orders = Err(err.to_string())
                }
                eprintln!("Fetched orders: {:?}", self.fetch_orders);
                return Task::none()
            }
            OrderListMessage::FetchedOrderAndUnfinished(orders) => {
                let response = match orders {
                    Ok(value) => value,
                    Err(err) => {
                        self.fetch_orders = Err(err.to_string());
                        return Task::none();
                    }
                };
                match serde_json::from_value::<req_resp::OrdersResponseData>(response) {
                    Ok(payload) => self.fetch_orders = Ok(payload.orders),
                    Err(err) => self.fetch_orders = Err(err.to_string())
                }
                eprintln!("Fetched orders: {:?}", self.fetch_orders);
                return Task::perform(
                    async move {}, 
                    |result| OrderListMessage::PollFetchedTablesWithUnfinishedOrders.into(),
                );            
            }
            OrderListMessage::FinishOrder(order_id) => {
                let mut request = req_resp::Request {
                    kind: req_resp::RequestKind::OrdersFinish,
                    payload: Some(serde_json::to_value(req_resp::OrdersFinishRequestData {
                        order: dbt::Order {
                            id: order_id.clone(),
                            ..Default::default()
                        } 
                    }).unwrap())
                };
                self.fetch_orders = Err("Refetching orders...".to_string());
                return Task::perform(
                    async move {request.send_request("".to_string()).await}, 
                    move |value| {
                        OrderListMessage::UpdateOrders(value).into()
                    }
                )
            }
            OrderListMessage::UpdateOrders(result) => {
                match result {
                    Ok(value) => {
                        let response = match serde_json::from_value::<req_resp::OrdersFinishResponseData>(value) {
                            Ok(payload) => payload,
                            Err(err) => {
                                self.fetch_orders = Err(format!("Something went wrong when deleting: {}", err));
                                return Task::none();
                            }
                        };

                        let current_table = if self.current_vtable.is_some() {
                            self.current_vtable.clone().unwrap()
                        } else {
                            self.current_vtable = Some(response.table.clone());
                            response.table
                        };

                        let mut request = req_resp::Request {
                            kind: req_resp::RequestKind::Orders,
                            payload: Some(serde_json::to_value(req_resp::OrdersRequestData {
                                new: true,
                                table: Some(current_table)
                            }).unwrap())
                        };
                        return Task::perform(
                            async move {request.send_request("".to_string()).await}, 
                            |result| OrderListMessage::FetchedOrderAndUnfinished(result).into(),
                        );
                    },
                    Err(err) => {
                        self.fetch_orders = Err(format!("Something went wrong when deleting: {}", err));
                        return Task::none();
                    }
                }
            }
        }

    }

}

fn virtual_table_button_style_new_order(theme: &Theme, status: iced::widget::button::Status) -> button::Style {

    let mut table_button_style = button::Style { 
        text_color: theme.extended_palette().primary.weak.text, 
        background: Some(iced::Background::Color(theme.extended_palette().primary.weak.color)), 
        border: Border { 
            color: theme.extended_palette().primary.base.color, 
            width: 0.0, 
            radius: Radius::new(7)
        }, 
        ..Default::default()
        // shadow: iced::Shadow { 
        //     color: theme.extended_palette().secondary.weak.color,
        //     offset: iced::Vector::new(0.0, 0.0), 
        //     blur_radius: 0.0
        // }
    };

    match status {
        button::Status::Active => {
            table_button_style.text_color = theme.extended_palette().primary.base.text; 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.base.color));
        },
        button::Status::Pressed =>{
            table_button_style.text_color = theme.extended_palette().primary.strong.text.inverse(); 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.strong.color.inverse()));
        },
        button::Status::Hovered => {
            table_button_style.text_color = theme.extended_palette().primary.strong.text; 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().primary.strong.color));
        },
        _ => {}
    }


    table_button_style


}

fn virtual_table_button_style(theme: &Theme, status: iced::widget::button::Status) -> button::Style {

    let mut table_button_style = button::Style { 
        text_color: theme.extended_palette().secondary.weak.text, 
        background: Some(iced::Background::Color(theme.extended_palette().secondary.weak.color)), 
        border: Border { 
            color: theme.extended_palette().secondary.base.color, 
            width: 0.0, 
            radius: Radius::new(7)
        }, 
        ..Default::default()
        // shadow: iced::Shadow { 
        //     color: theme.extended_palette().primary.strong.color,
        //     offset: iced::Vector::new(0.0, 0.0), 
        //     blur_radius: 0.0
        // }
    };

    match status {
        button::Status::Active => {
            table_button_style.text_color = theme.extended_palette().secondary.base.text; 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().secondary.base.color));
        },
        button::Status::Pressed =>{
            table_button_style.text_color = theme.extended_palette().secondary.strong.text.inverse(); 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().secondary.strong.color.inverse()));
        },
        button::Status::Hovered => {
            table_button_style.text_color = theme.extended_palette().secondary.strong.text; 
            table_button_style.background = Some(iced::Background::Color(theme.extended_palette().secondary.strong.color));
        },
        _ => {}
    }


    table_button_style


}

fn order_item_container_style(theme: &Theme) -> iced::widget::container::Style {

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


/// Veličina u pikselima
const VIRTUAL_TABLE_LIST_WIDTH: f32 = 300.0;
const ORDER_LIST_WIDTH: f32 = 600.0;


impl Tab for OrderList {

    type Message = crate::Message;

    fn title(&self) -> String {TITLE.into()}

    fn inner_title(&self) -> iced::Element<'_, Self::Message> {

        let order_label = if self.current_vtable.is_some() {
            format!("Orders from `{}`", self.current_vtable.clone().unwrap())
        } else {
            format!("Orders")
        };

        let virt_table_label: iced::widget::Container<'_, Message> = 
            container(text!("Virtual Tables")
                .size(26))
                .height(Length::Shrink)
                .width(VIRTUAL_TABLE_LIST_WIDTH)
                .align_x(iced::alignment::Horizontal::Center);

        row!(
            center(text!("{}", order_label).size(26)).height(Length::Shrink),
            virt_table_label,
        ).into()    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        iced_aw::TabLabel::IconText(ICON, self.title())
    }

    fn content(&self) -> iced::Element<'_, Self::Message> {

        if self.fetch_vtables.is_err() {
            return center(text(format!("{:?}", self.fetch_vtables))).into()
        }
        
        // TABLE PANE

        let mut table_column: Column<'_, Message> = Column::new()
            .spacing(10)
            .padding(20);

        let vtables = self.fetch_vtables.clone().unwrap();

        for vtable in vtables {

            table_column = table_column.push(
                button(
                    row![
                        text(format!("Virtual Table:")).font(iced::font::Font {
                            weight: iced::font::Weight::Semibold,
                            ..Default::default()
                        }),
                        horizontal_space(),
                        text(format!("`{}`", vtable.name)).font(iced::font::Font {
                            weight: iced::font::Weight::Semibold,
                            ..Default::default()
                        }),
                    ]
                )
                .style(
                    if self.unfinished_tables.clone().is_ok_and(|x| x.contains(&vtable.name.clone())) {
                        virtual_table_button_style_new_order
                    } else {
                        virtual_table_button_style
                    }
                )
                .on_press(OrderListMessage::TablePressed(vtable.name).into())
                .width(Length::Fill)
            )

        }

        // ORDER PANE
        let mut order_pane: iced::Element<'_, Self::Message> = text!("").into();

        if self.fetch_orders.is_err() {
            order_pane = center(text!("fetch_orders: {}", self.fetch_orders.clone().unwrap_err())).into();
        } else if self.fetch_items.is_err() {
            order_pane = center(text!("fetch_items: {}", self.fetch_items.clone().unwrap_err())).into();
        } else {

            let fetched_orders = self.fetch_orders.clone().unwrap();
            if fetched_orders.is_empty() {
                order_pane = center(text!("No active orders for this table :(")).into()
            } else {
                let mut item_map: HashMap<String, dbt::Offer> = HashMap::new();
                for offer in self.fetch_items.clone().unwrap() {
                    item_map.insert(offer.clone().name, offer);
                }
                let mut order_list: Column<'_, Message> = Column::new();
                for order in fetched_orders {
                    let mut total: (u32, u32) = (0,0); 

                    let mut item_list: Column<'_, Message> = Column::new()
                        .spacing(10);

                    for item in order.items {
                        let actual_item = match item_map.get(&item.id) {
                            Some(result) => result,
                            None => {
                                item_list = item_list.push(
                                    container(text!("Unknown item: `{}`", item.id))
                                        .style(order_item_container_style)
                                );
                                continue;
                            }
                        };
                        item_list = item_list.push(
                            container(
                                column![
                                    row![text!("{}", actual_item.name), horizontal_space(), text!("Price: {}.{}", 
                                        actual_item.price_integer, 
                                        actual_item.price_fraction).height(Length::Shrink)],
                                    row![horizontal_space(), text!("Quantity: {}", item.count)].height(Length::Shrink)
                                ]
                            )
                            .height(Length::Shrink)
                            .width(Length::Fill)
                            .style(order_item_container_style)
                            .padding(10)
                        );
                        total.0 += actual_item.price_integer * item.count;
                        total.1 += actual_item.price_fraction * item.count;
                    }

                    

                    order_list = order_list.push(
                        column![
                            column![
                                row![text!("Order {}", order.id.count).size(24), horizontal_space()],
                                item_list,
                                row![
                                    text!("Total: {}.{}", total.0 + total.1/100, total.1%100),
                                    horizontal_space(),
                                    button(
                                        text!("Finish").font(iced::font::Font {
                                            weight: iced::font::Weight::Bold,
                                            ..Default::default()
                                        })
                                        .height(Length::Shrink)
                                    )
                                    .style(virtual_table_button_style)
                                    .on_press(OrderListMessage::FinishOrder(order.id.clone()).into())
                                ]
                                .height(Length::Shrink),
                                ].spacing(10),
                                horizontal_rule(2)
                        ]
                        .height(Length::Shrink)
                        .spacing(105)
                        .padding(10)

                    )
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .padding(10);
                }
                order_pane = scrollable(order_list)
                    .height(Length::Shrink)
                    .width(Length::Shrink)
                    .into();
            }

        }


        let panes = row![
            center(order_pane)
                .width(Length::Fill) // Make it take the full width of its column
                .height(Length::Fill) // Make it take the full height
                .align_x(Horizontal::Center),
            iced::widget::vertical_rule(2),
            container(scrollable(table_column)
                .direction(iced::widget::scrollable::Direction::Vertical(
                    iced::widget::scrollable::Scrollbar::new()
                        .anchor(scrollable::Anchor::Start)
                    )
                )
                .height(Length::Fill).width(VIRTUAL_TABLE_LIST_WIDTH))
                .style(|theme: &Theme| {
                    iced::widget::container::Style {
                        // background: Some(iced::Background::Color(theme.extended_palette().secondary.base.color)),
                        ..Default::default()
                    }
                }
            ),
        ];
        
        return center(panes).into()

    }

}


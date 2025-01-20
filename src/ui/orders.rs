
use iced::{
    alignment::Horizontal, border::Radius, widget::{
        button, center, column, container, horizontal_rule, horizontal_space, row, scrollable, text, Column
    }, Border, Length, Task, Theme
};
use crate::{database_types::VirtualTableID, Message};
use crate::database_types;
use super::Tab;

use crate::server_old as server;

const TITLE: &str = "Orders"; 
const ICON:  char = '\u{e9ba}';

#[derive(Debug, Clone)]
pub enum OrderListMessage {
    FetchedVirtualTablesAndItems(server::FetchVirtualTablesAndItems),
    UpdateOrders(Result<VirtualTableID, String>),
    TablePressed(database_types::VirtualTableID),
    FetchedOrders(Result<server::FetchVirtualTableNewOrders, String>),
    FinishOrder(database_types::OrderID),
    PollOrders
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

    pub fetch_vtables: Result<server::FetchVirtualTables, String>,
    pub current_vtable: Option<database_types::VirtualTableID>,
    pub fetch_orders: Result<server::FetchVirtualTableNewOrders, String>,
    pub fetch_items: Result<server::FetchItems, String>

}

impl Default for OrderList {
    fn default() -> Self {
        Self { 
            current_vtable: None,
            fetch_vtables:  Err("Fetching data...".to_string()),
            fetch_orders:   Err("Click on a table first :)".to_string()),
            fetch_items:    Err("Couldn't fetch Item's".to_string()) 
        }
    }
}


impl OrderList {


    pub fn update(&mut self, message: OrderListMessage) -> Task<Message> {

        match message {
            OrderListMessage::FetchedVirtualTablesAndItems(fetch) => {
                self.fetch_vtables = fetch.fetched_vtables;
                self.fetch_items = fetch.fetched_items;
                eprintln!("Fetched data: {:#?}", self);
                return Task::none()
            },
            OrderListMessage::TablePressed(vtable) => {
                self.current_vtable = Some(vtable.clone());
                return  Task::perform(
                    async move {server::FetchVirtualTableNewOrders::from_db(vtable)}, 
                    |value| {
                        OrderListMessage::FetchedOrders(value).into()
                    }
                )
            }
            OrderListMessage::PollOrders => {
                if let Some(current_table) = &self.current_vtable {
                    // Fetch new orders for the currently selected table
                    let table = current_table.clone();
                    return Task::perform(
                        async move { server::FetchVirtualTableNewOrders::from_db(table) },
                        |result| OrderListMessage::FetchedOrders(result).into(),
                    );
                }
                Task::none() // Do nothing if no table is selected
            }
            OrderListMessage::FetchedOrders(orders) => {
                self.fetch_orders = orders;
                eprintln!("Fetched orders: {:?}", self.fetch_orders);
                return Task::none()
            }
            OrderListMessage::FinishOrder(order_id) => {
                let table = order_id.table.clone();
                let request = server::RequestFinishOrder {
                    order_id: order_id
                };
                self.fetch_orders = Err("Refetching orders...".to_string());
                return Task::perform(
                    async move {server::RequestFinishOrder::send_request(request)}, 
                    move |value| {
                        OrderListMessage::UpdateOrders(value).into()
                    }
                )
            }
            OrderListMessage::UpdateOrders(result) => {
                match result {
                    Ok(table) => {
                        let current_table = if self.current_vtable.is_some() {
                            self.current_vtable.clone().unwrap()
                        } else {
                            self.current_vtable = Some(table.clone());
                            table
                        };
                        return Task::perform(
                            async move {server::FetchVirtualTableNewOrders::from_db(current_table)}, 
                            |value| {
                                OrderListMessage::FetchedOrders(value).into()
                            }
                        )
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

fn virtual_table_button_style(theme: &Theme, status: iced::widget::button::Status) -> button::Style {

    let mut table_button_style = button::Style { 
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

        let vtables = self.fetch_vtables.clone().unwrap().vtables;

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
                .style(virtual_table_button_style)
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

            let fetched = self.fetch_orders.clone().unwrap();
            if fetched.orders.is_empty() {
                order_pane = center(text!("No active orders for this table :(")).into()
            } else {
                let item_map = self.fetch_items.clone().unwrap().items;
                let mut order_list: Column<'_, Message> = Column::new();
                for order in fetched.orders {
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


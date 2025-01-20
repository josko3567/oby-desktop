use chrono::DateTime;
use chrono::Local;



enum OrderState {
    Pending,
    Accepted,
    Denied,
    AwaitingPayment,
    Payed((i32, i32))
}

pub struct VirtualTable {

    name:   String,
    id:     u32,
    orders: Vec<Order>

}

pub struct Order {

    items: Vec<OrderedItem>,
    time: DateTime<Local>

}

pub struct Item {

    name: String,
    price: (i32, i32),
    extras: Vec<Extra>

}

pub struct Extra {

    name: String,
    price: (i32, i32)

}

pub struct OrderedItem {

    item: Item,
    amount: i32,
    extras: Vec<OrderedExtra> 

}

pub struct OrderedExtra {

    extra: Extra,
    amount: i32,

}
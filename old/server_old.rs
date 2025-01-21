use std::{collections::HashMap, io::{Read, Write}, net::TcpStream, str::FromStr};
use serde::{Deserialize, Serialize};
use crate::shared::dbt as dbt;

const IP_PORT: &'static str = "127.0.0.1:8000";

#[derive(Default, Debug, strum::EnumString, strum::Display, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Message {

    #[default]
    unknown,
    print_current_database,

    get_virtual_tables,
    remove_virtual_table,
    insert_virtual_table,
    update_virtual_table,

    get_items,
    insert_items,
    remove_items,

    order_items,
    get_order_number,
    get_virtual_table_new_orders,
    finish_order

}


fn fetch_format(kind: String, payload: String) -> Vec<u8> {
    format!("REQUEST!{}:{}", kind, payload).into_bytes()
}

fn extract_parts(payload: String) -> Result<(Message, String), String> {

    let (_request, rest) = match payload.split_once('!') {
        Some((f, s)) => (f, s),
        None => return Err("Invalid request...".to_string())
    };

    let (kind, payload) = match rest.split_once(":") {
        Some((f, payload)) => {
            let msg = match Message::from_str(f) {
                Ok(result) => result,
                Err(err) => return Err(err.to_string())
            };
            (msg, payload.to_string())
        }
        None => return Err("Invalid request...".to_string())
    };

    Ok((kind, payload))

}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct FetchVirtualTables {
    pub vtables: Vec<dbt::VirtualTable>
} 

impl FetchVirtualTables {

    pub fn from_db() -> Result<Self, String> {

        let mut stream = match  TcpStream::connect(IP_PORT) {
    
            Ok(stream) => stream,
            Err(err) => return Err(err.to_string())
    
        };
    
        match stream.write(&fetch_format(
            Message::get_virtual_tables.to_string(), 
            "".to_string())
        ) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        }
        
        let mut s = String::new();
        match stream.read_to_string(&mut s) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };
    
        let (msg, payload) = match extract_parts(s) {
            Ok((msg, payload)) => (msg, payload),
            Err(err) => return Err(err.to_string())
        };
    
        if msg != Message::get_virtual_tables {
            return Err("Invalid message kind...".to_string());
        }
    
        let fetched = match serde_json::from_str::<FetchVirtualTables>(payload.as_str()) {
            Ok(fetched) => fetched,
            Err(err) => return Err(err.to_string())
        };
    
        Ok(fetched)
    
    }

}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct FetchItems {
    pub items: HashMap<String, dbt::Offer>,
} 

impl FetchItems {

    pub fn from_db() -> Result<Self, String> {

        let mut stream = match  TcpStream::connect(IP_PORT) {

            Ok(stream) => stream,
            Err(err) => return Err(err.to_string())

        };

        match stream.write(&fetch_format(
            Message::get_items.to_string(), 
            "".to_string())
        ) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        }
        
        let mut s = String::new();
        match stream.read_to_string(&mut s) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };

        let (msg, payload) = match extract_parts(s) {
            Ok((msg, payload)) => (msg, payload),
            Err(err) => return Err(err.to_string())
        };

        if msg != Message::get_items {
            return Err("Invalid message kind...".to_string());
        }

        let fetched = match serde_json::from_str::<FetchItems>(payload.as_str()) {
            Ok(fetched) => fetched,
            Err(err) => return Err(err.to_string())
        };

        Ok(fetched)

    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FetchVirtualTablesAndItems {
    pub fetched_vtables: Result<FetchVirtualTables, String>,
    pub fetched_items: Result<FetchItems, String>
}

impl FetchVirtualTablesAndItems {

    pub fn from_db() -> Self {
        Self {
            fetched_vtables: FetchVirtualTables::from_db(),
            fetched_items: FetchItems::from_db(),
        }
    }

}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct FetchVirtualTableNewOrders {
    pub orders: Vec<dbt::Order>
} 

impl FetchVirtualTableNewOrders {

    pub fn from_db(vtable: dbt::VirtualTableID) -> Result<Self, String> {

        let mut stream = match  TcpStream::connect(IP_PORT) {

            Ok(stream) => stream,
            Err(err) => return Err(err.to_string())

        };

        match stream.write(&fetch_format(
            Message::get_virtual_table_new_orders.to_string(), 
            vtable)
        ) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        }
        
        let mut s = String::new();
        match stream.read_to_string(&mut s) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };

        let (msg, payload) = match extract_parts(s) {
            Ok((msg, payload)) => (msg, payload),
            Err(err) => return Err(err.to_string())
        };

        if msg != Message::get_virtual_table_new_orders {
            return Err("Invalid message kind...".to_string());
        }

        let fetched = match serde_json::from_str::<FetchVirtualTableNewOrders>(payload.as_str()) {
            Ok(fetched) => fetched,
            Err(err) => return Err(err.to_string())
        };


        Ok(fetched)

    }

}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RequestFinishOrder {
    pub order_id: dbt::OrderID
}

impl RequestFinishOrder {

    pub fn send_request(self) -> Result<dbt::VirtualTableID, String> {

        let mut stream = match  TcpStream::connect(IP_PORT) {

            Ok(stream) => stream,
            Err(err) => return Err(err.to_string())

        };

        let payload = match serde_json::to_string(&self.order_id) {
            Ok(payload) => payload,
            Err(err) => return Err(err.to_string())
        };

        match stream.write(&fetch_format(
            Message::finish_order.to_string(), 
            payload)
        ) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        }
        
        let mut s = String::new();
        match stream.read_to_string(&mut s) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };

        let (msg, payload) = match extract_parts(s) {
            Ok((msg, payload)) => (msg, payload),
            Err(err) => return Err(err.to_string())
        };

        if msg != Message::finish_order {
            return Err("Invalid message kind...".to_string());
        }

        if payload.as_str() == "REQUEST_SUCCEEDED" {
            Ok(self.order_id.table)
        } else {
            Err(payload)
        }

    }

}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RequestDeleteVirtualTable {
    pub table_id: dbt::VirtualTableID
}

impl RequestDeleteVirtualTable {

    pub fn send_request(self) -> Result<(), String> {

        let mut stream = match  TcpStream::connect(IP_PORT) {

            Ok(stream) => stream,
            Err(err) => return Err(err.to_string())

        };

        match stream.write(&fetch_format(
            Message::remove_virtual_table.to_string(), 
            self.table_id)
        ) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        }
        
        let mut s = String::new();
        match stream.read_to_string(&mut s) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };

        let (msg, payload) = match extract_parts(s) {
            Ok((msg, payload)) => (msg, payload),
            Err(err) => return Err(err.to_string())
        };

        if msg != Message::remove_virtual_table {
            return Err("Invalid message kind...".to_string());
        }

        if payload.as_str() == "REQUEST_SUCCEEDED" {
            Ok(())
        } else {
            Err(payload)
        }

    }

}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RequestInsertVirtualTable {
    pub table_id: dbt::VirtualTableID
}

impl RequestInsertVirtualTable {

    pub fn send_request(self) -> Result<(), String> {

        let mut stream = match  TcpStream::connect(IP_PORT) {

            Ok(stream) => stream,
            Err(err) => return Err(err.to_string())

        };

        match stream.write(&fetch_format(
            Message::insert_virtual_table.to_string(), 
            self.table_id)
        ) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        }
        
        let mut s = String::new();
        match stream.read_to_string(&mut s) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };

        let (msg, payload) = match extract_parts(s) {
            Ok((msg, payload)) => (msg, payload),
            Err(err) => return Err(err.to_string())
        };

        if msg != Message::insert_virtual_table {
            return Err("Invalid message kind...".to_string());
        }

        if payload.as_str() == "REQUEST_SUCCEEDED" {
            Ok(())
        } else {
            Err(payload)
        }

    }

}
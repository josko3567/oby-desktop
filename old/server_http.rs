use std::collections::HashMap;

use crate::shared::dbt as dbt;

#[derive(Debug)]
#[derive(strum::EnumString, strum::EnumProperty)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum Request {
    virtual_table(RequestVirtualTable),
    offer(RequestOffer),
    order {
        order: dbt::Order
    },
}

#[derive(Debug)]
#[derive(strum::EnumString, strum::EnumProperty)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum RequestVirtualTable {

    /// ADMIN, GET
    fetch {
        only_unfinished_offers: bool,
    },
    /// ADMIN, POST
    add {
        tables: Vec<dbt::VirtualTable>
    },
    /// ADMIN, DELETE
    remove {
        tables: Vec<dbt::VirtualTableID>
    },
    /// ADMIN, PATCH
    update {
        tables: HashMap<dbt::VirtualTableID, dbt::VirtualTable>
    },

}

#[derive(Debug)]
#[derive(strum::EnumString, strum::EnumProperty)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum RequestOffer {

    /// ADMIN, GET
    fetch {
        only_unfinished_offers: bool,
        table: dbt::VirtualTableID,
        /// Optionally get specific offers, otherwise all. 
        offers: Option<Vec<dbt::OfferID>>
    },
    /// ADMIN, POST
    add {
        table: dbt::VirtualTableID,
        offer: Vec<dbt::Offer>
    },
    /// ADMIN, DELETE
    remove {
        table: dbt::VirtualTableID,
        offer: Vec<dbt::OfferID>
    },
    /// ADMIN, PATCH
    update {
        table: dbt::VirtualTableID,
        offer: HashMap<dbt::OfferID, dbt::Offer>
    },

}

impl Default for RequestOffer {
    fn default() -> Self {
        return Self::fetch {
            offers: None,
            table: dbt::VirtualTableID::default(),
            only_unfinished_offers: false,
        };
    }
}

impl Default for RequestVirtualTable {
    fn default() -> Self {
        return Self::fetch {
            only_unfinished_offers: false,
        };
    }
}


// pub enum Request {

//     /// ADMIN, GET
//     fetch_virtual_tables {
//         new_offers: bool
//     },
//     /// ADMIN, POST
//     post_virtual_tables {
//         tables: Vec<dbt::VirtualTable>
//     },
//     /// ADMIN, DELETE
//     remove_virtual_tables {
//         tables: Vec<dbt::VirtualTableID>
//     },
//     /// ADMIN, PATCH
//     update_virtual_tables {
//         tables: HashMap<dbt::VirtualTableID, dbt::VirtualTable>
//     },

//     /// USER, GET
//     fetch_offers,
//     /// ADMIN, POST
//     post_offers {
//         offers: Vec<dbt::Offer>
//     },
//     /// ADMIN, DELETE
//     remove_offers {
//         offers: Vec<dbt::OfferID>
//     },
//     // ADMIN, PATCH
//     update_offers {
//         offers: HashMap<dbt::OfferID, dbt::Offer>
//     },

//     /// USER, POST
//     order {

//     }

// }



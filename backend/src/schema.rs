// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "parcelsize"))]
    pub struct Parcelsize;
}

diesel::table! {
    addresses (id) {
        id -> Int4,
        street -> Varchar,
        city -> Varchar,
        postal_code -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Parcelsize;

    parcels (id) {
        id -> Int4,
        recipient_name -> Varchar,
        recipient_email -> Varchar,
        recipient_phone -> Varchar,
        recipient_address_id -> Int4,
        warehouse_id -> Int4,
        pickup_date -> Date,
        size -> Parcelsize,
    }
}

diesel::table! {
    roles (role_name, user_id) {
        role_name -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    trade_partners (id) {
        id -> Int4,
        name -> Varchar,
        price_list -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        pass_hash -> Varchar,
    }
}

diesel::table! {
    warehouses (id) {
        id -> Int4,
        name -> Varchar,
        trade_partner_id -> Int4,
        address_id -> Int4,
    }
}

diesel::joinable!(parcels -> addresses (recipient_address_id));
diesel::joinable!(parcels -> warehouses (warehouse_id));
diesel::joinable!(roles -> users (user_id));
diesel::joinable!(warehouses -> addresses (address_id));
diesel::joinable!(warehouses -> trade_partners (trade_partner_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    parcels,
    roles,
    trade_partners,
    users,
    warehouses,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        id -> Int4,
        street -> Varchar,
        city -> Varchar,
        postal_code -> Varchar,
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

diesel::joinable!(roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    roles,
    trade_partners,
    users,
);

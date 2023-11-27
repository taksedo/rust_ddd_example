// @generated automatically by Diesel CLI.

pub mod shop {
    diesel::table! {
        shop.meal (id) {
            id -> Int8,
            name -> Varchar,
            description -> Nullable<Varchar>,
            removed -> Bool,
            price -> Numeric,
            version -> Int8,
        }
    }
}

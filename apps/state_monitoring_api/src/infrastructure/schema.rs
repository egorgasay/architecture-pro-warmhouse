// @generated automatically by Diesel CLI.

diesel::table! {
    sensor_data (id) {
        id -> Int4,
        value -> Varchar,
        status -> Varchar,
        ts -> Timestamp,
    }
}

diesel::table! {
    service_contexts (id) {
        id -> Int4,
        maintenance -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(sensor_data, service_contexts,);

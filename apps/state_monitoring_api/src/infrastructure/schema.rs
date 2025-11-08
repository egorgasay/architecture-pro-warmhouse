// @generated automatically by Diesel CLI.

diesel::table! {
    sensor_data (id) {
        id -> Nullable<Int4>,
        sensor_id -> Int4,
        value -> Double,
        unit -> Varchar,
        status -> Varchar,
        created_at -> Timestamptz,
    }
}

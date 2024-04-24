// @generated automatically by Diesel CLI.

diesel::table! {
    calculations (id) {
        id -> Int4,
        temperature -> Float8,
        wind_speed -> Float8,
        relative_humidity -> Float8,
        dew_point -> Float8,
    }
}

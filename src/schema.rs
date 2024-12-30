// @generated automatically by Diesel CLI.

diesel::table! {
    prompt_formats (id) {
        id -> Integer,
        format_name -> Text,
        prompt -> Text,
    }
}

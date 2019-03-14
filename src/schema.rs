table! {
    achievements (name, date) {
        name -> Text,
        date -> Text,
    }
}

table! {
    palm_log (log_time) {
        log_time -> Text,
        moisture -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    achievements,
    palm_log,
);

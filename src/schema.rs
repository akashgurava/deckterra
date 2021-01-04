table! {
    cards (id) {
        id -> Integer,
        deck_code -> Text,
        code -> Text,
        set -> Integer,
        faction -> Integer,
        number -> Integer,
        count -> Integer,
    }
}

table! {
    decks (id) {
        id -> Integer,
        uid -> Text,
        deck_code -> Text,
        title -> Text,
        description -> Text,
        rating -> Integer,
        mode -> Text,
        playstyle -> Text,
        changed_at -> Timestamp,
        created_at -> Timestamp,
        is_draft -> Bool,
        is_private -> Bool,
        is_riot -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    cards,
    decks,
);

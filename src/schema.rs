diesel::table! {
    fish_users (id) {
        id -> BigInt,
        external_id -> BigInt,
        created_at -> Timestamptz,
        updated_at -> Timestamptz
    }
}

diesel::table! {
    fish_fishes (id) {
        id -> BigInt,
        user_id -> BigInt,
        data_id -> Integer,
        created_at -> Timestamptz,
        updated_at -> Timestamptz
    }
}

diesel::joinable!(fish_fishes -> fish_users (user_id));
diesel::allow_tables_to_appear_in_same_query!(fish_fishes, fish_users);

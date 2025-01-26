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
        updated_at -> Timestamptz,
        size_baby_mm -> Float,
        size_adult_mm -> Float,
        lifespan_days -> Float
    }
}

diesel::table! {
    fish_ponds (id) {
        id -> BigInt,
        user_id -> BigInt,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        capacity -> Integer
    }
}

diesel::joinable!(fish_fishes -> fish_users (user_id));
diesel::allow_tables_to_appear_in_same_query!(fish_fishes, fish_users);

diesel::joinable!(fish_ponds -> fish_users (user_id));
diesel::allow_tables_to_appear_in_same_query!(fish_ponds, fish_users);

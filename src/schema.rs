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
        species_id -> Integer,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        size_baby_mm -> Float,
        size_adult_mm -> Float,
        lifespan_days -> Float,
        catch_age -> Float
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

diesel::table! {
    fish_fishing_history_entries (id) {
        id -> BigInt,
        user_id -> BigInt,
        species_id -> Integer,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        caught_count -> Integer,
        sold_count -> Integer,
        smallest_catch_mm -> Float,
        largest_catch_mm -> Float,
        last_catch -> Timestamptz,
        first_sell -> Nullable<Timestamptz>,
        last_sell -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(fish_fishes -> fish_users (user_id));
diesel::allow_tables_to_appear_in_same_query!(fish_fishes, fish_users);

diesel::joinable!(fish_ponds -> fish_users (user_id));
diesel::allow_tables_to_appear_in_same_query!(fish_ponds, fish_users);

diesel::joinable!(fish_fishing_history_entries -> fish_users (user_id));
diesel::allow_tables_to_appear_in_same_query!(fish_fishing_history_entries, fish_users);

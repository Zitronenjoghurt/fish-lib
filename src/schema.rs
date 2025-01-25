diesel::table! {
    users (id) {
        id -> BigInt,
        external_id -> BigInt,
    }
}

diesel::table! {
    fishes (id) {
        id -> BigInt,
        user_id -> BigInt,
    }
}

diesel::joinable!(fishes -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(fishes, users);

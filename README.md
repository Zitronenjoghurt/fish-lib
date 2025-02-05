[![](https://img.shields.io/crates/v/fish-lib)](https://crates.io/crates/fish-lib)
[![Library](https://github.com/Zitronenjoghurt/fish-lib/actions/workflows/library.yml/badge.svg)](https://github.com/Zitronenjoghurt/fish-lib/actions/workflows/library.yml)
[![Example](https://github.com/Zitronenjoghurt/fish-lib/actions/workflows/example.yml/badge.svg)](https://github.com/Zitronenjoghurt/fish-lib/actions/workflows/example.yml)
[![codecov](https://codecov.io/gh/Zitronenjoghurt/fish-lib/graph/badge.svg?token=UM6T22YO17)](https://codecov.io/gh/Zitronenjoghurt/fish-lib)
![](https://tokei.rs/b1/github/Zitronenjoghurt/fish-lib?category=code&type=Rust&logo=https://simpleicons.org/icons/rust.svg)

# fish-lib

**THIS LIBRARY IS STILL IN DEVELOPMENT, ALL FEATURES ARE WORK-IN-PROGRESS AND MAY BE SUBJECT TO CHANGE**

The game/storage logic for a highly customizable fishing game. This library is tailored to MMO-style fishing games for
discord bots, etc.

## Features

- Full Database handling
    - You will only have to provide a Postgres Database and its URL
- Customization
    - You can customize the gameplay and content in detail
- Clean public API
    - The Game interface is clean and documented comprehensively with doc-tested examples
- Gameplay features
    - Random generated fish data, size and age for your customized fish species
    - Customizable locations with randomly generated weather
    - Customizable fish encounter data, they will be able to be encountered at different rarities and at specific times
      and weather
    - and much more...

# Examples

These examples are not doc-tested. If you notice inaccuracies, please create an issue.\
Doc-tested examples can be found in the Game struct's documentation.

## Basic Functionality

This will show you how to interact with this library on a basic level.

```rust
use fish_lib::config::{Config, ConfigBuilderInterface};
use fish_lib::game::prelude::*;
use std::path::Path;

const POSTGRES_URL: &str = "...";

// For simplification all errors will be unwrapped, you can choose how you want to handle errors.
// Examples of error handling can be found in the documentation of the Game functions.
fn main() {
    let locations_file_path: &Path = "...";
    let settings_file_path: &Path = "...";
    let species_file_path: &Path = "...";
    let config = Config::builder()
        .locations_file(locations_file_path).unwrap()
        .settings_file(settings_file_path).unwrap()
        .species_file(species_file_path).unwrap()
        .build();

    // Will create a new game interface
    // Fails if it's unable to connect to the database
    let game = Game::new(POSTGRES_URL, Some(config)).unwrap();

    // Register a user
    let external_id: i64 = 1337; // That's the ID your system identifies this user with
    let mut user = game.user_register(external_id).unwrap();

    // Manipulate and save the user
    user.credits = 10;
    let updated_user = game.user_save(user).unwrap();
    assert_eq!(updated_user.credits, 10);
}
```
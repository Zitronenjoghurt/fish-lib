use crate::config::Config;
use crate::data::species_data::SpeciesData;
use crate::models::specimen::Specimen;
use crate::{set_config, setup_test};
use chrono::{Duration, Utc};
use std::collections::HashMap;

fn mock_config() {
    let species_data = SpeciesData {
        name: "salmon".to_string(),
        min_size_baby_mm: 10,
        max_size_baby_mm: 30,
        min_size_adult_mm: 20,
        max_size_adult_mm: 60,
        min_weight_baby_g: 20,
        max_weight_baby_g: 60,
        min_weight_adult_g: 40,
        max_weight_adult_g: 120,
        min_lifespan_days: 1,
        max_lifespan_days: 4,
        lifespan_adult_ratio: 0.35,
        encounters: Default::default(),
    };
    let mut species_data_map = HashMap::new();
    species_data_map.insert(0, species_data);

    let config = Config::builder().species(species_data_map).build();
    set_config(config);
}

#[test]
fn test_age_calculation() {
    setup_test();
    mock_config();

    let now = Utc::now();
    let yesterday = now - Duration::days(1);

    let specimen = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: now,
        updated_at: now,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 0.0,
        catch_age: 0.5,
    };

    let specimen2 = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 0.0,
        catch_age: 0.5,
    };

    let specimen3 = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 1.0,
        catch_age: 0.0,
    };

    let age = specimen.get_age(1.0);
    assert!(
        (age - 0.5).abs() < 0.001,
        "Expected age to be approximately 0.5, got {}",
        age
    );

    let age2 = specimen2.get_age(1.0);
    assert!(
        (age2 - 1.0).abs() < 0.001,
        "Expected age2 to be approximately 1.0, got {}",
        age2
    );

    let age3 = specimen3.get_age(1.0);
    assert!(
        (age3 - 0.25).abs() < 0.001,
        "Expected age3 to be approximately 0.25, got {}",
        age3
    );

    let age3_accelerated = specimen3.get_age(2.0);
    assert!(
        (age3_accelerated - 0.5).abs() < 0.001,
        "Expected age3_accelerated to be approximately 0.5, got {}",
        age3_accelerated
    );
}

#[test]
fn test_size_calculation() {
    setup_test();
    mock_config();

    let now = Utc::now();
    let yesterday = now - Duration::days(1);

    let specimen = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: now,
        updated_at: now,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 0.0,
        catch_age: 0.5,
    };

    let specimen2 = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 0.0,
        catch_age: 0.5,
    };

    let specimen3 = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 1.0,
        catch_age: 0.0,
    };

    let size = specimen.get_size_mm(1.0);
    assert!(
        (size - 30.0).abs() < 0.001,
        "Expected size to be approximately 30.0, got {}",
        size
    );

    let size2 = specimen2.get_size_mm(1.0);
    assert!(
        (size2 - 40.0).abs() < 0.001,
        "Expected size2 to be approximately 40.0, got {}",
        size2
    );

    let size3 = specimen3.get_size_mm(1.0);
    assert!(
        (size3 - 25.0).abs() < 0.001,
        "Expected size3 to be approximately 25.0, got {}",
        size3
    );

    let size3_accelerated = specimen3.get_size_mm(3.0);
    assert!(
        (size3_accelerated - 35.0).abs() < 0.001,
        "Expected size3_accelerated to be approximately 35.0, got {}",
        size3_accelerated
    );
}

#[test]
fn test_weight_calculation() {
    setup_test();
    mock_config();

    let now = Utc::now();
    let yesterday = now - Duration::days(1);

    let specimen = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: now,
        updated_at: now,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 0.0,
        catch_age: 0.5,
    };

    let specimen2 = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 0.0,
        catch_age: 0.5,
    };

    let specimen3 = Specimen {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 1.0,
        catch_age: 0.0,
    };

    let weight = specimen.get_weight_g(1.0);
    assert!(
        (weight - 60.0).abs() < 0.001,
        "Expected weight to be approximately 60.0, got {}",
        weight
    );

    let weight2 = specimen2.get_weight_g(1.0);
    assert!(
        (weight2 - 80.0).abs() < 0.001,
        "Expected weight2 to be approximately 80.0, got {}",
        weight2
    );

    let weight3 = specimen3.get_weight_g(1.0);
    assert!(
        (weight3 - 50.0).abs() < 0.001,
        "Expected weight3 to be approximately 50.0, got {}",
        weight3
    );

    let weight3_accelerated = specimen3.get_weight_g(3.0);
    assert!(
        (weight3_accelerated - 70.0).abs() < 0.001,
        "Expected weight3_accelerated to be approximately 70.0, got {}",
        weight3_accelerated
    );
}

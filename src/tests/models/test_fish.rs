use crate::models::fish::Fish;
use chrono::{Duration, Utc};

#[test]
fn test_age_calculation() {
    let now = Utc::now();
    let yesterday = now - Duration::days(1);

    let fish = Fish {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: now,
        updated_at: now,
        size_baby_mm: 20.0,
        size_adult_mm: 40.0,
        lifespan_days: 1.0,
        catch_age: 0.5,
    };

    let fish2 = Fish {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_mm: 20.0,
        size_adult_mm: 40.0,
        lifespan_days: 1.0,
        catch_age: 0.5,
    };

    let fish3 = Fish {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_mm: 20.0,
        size_adult_mm: 40.0,
        lifespan_days: 4.0,
        catch_age: 0.0,
    };

    let age = fish.get_age(1.0);
    assert!(
        (age - 0.5).abs() < 0.001,
        "Expected age to be approximately 0.5, got {}",
        age
    );

    let age2 = fish2.get_age(1.0);
    assert!(
        (age2 - 1.0).abs() < 0.001,
        "Expected age2 to be approximately 1.0, got {}",
        age2
    );

    let age3 = fish3.get_age(1.0);
    assert!(
        (age3 - 0.25).abs() < 0.001,
        "Expected age3 to be approximately 0.25, got {}",
        age3
    );

    let age3_accelerated = fish3.get_age(2.0);
    assert!(
        (age3_accelerated - 0.5).abs() < 0.001,
        "Expected age3_accelerated to be approximately 0.5, got {}",
        age3_accelerated
    );
}

#[test]
fn test_size_calculation() {
    let now = Utc::now();
    let yesterday = now - Duration::days(1);

    let fish = Fish {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: now,
        updated_at: now,
        size_baby_mm: 20.0,
        size_adult_mm: 40.0,
        lifespan_days: 1.0,
        catch_age: 0.5,
    };

    let fish2 = Fish {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_mm: 20.0,
        size_adult_mm: 40.0,
        lifespan_days: 1.0,
        catch_age: 0.5,
    };

    let fish3 = Fish {
        id: 0,
        user_id: 0,
        species_id: 0,
        created_at: yesterday,
        updated_at: yesterday,
        size_baby_mm: 20.0,
        size_adult_mm: 40.0,
        lifespan_days: 4.0,
        catch_age: 0.0,
    };

    let size = fish.get_size_mm(1.0);
    assert!(
        (size - 30.0).abs() < 0.001,
        "Expected size to be approximately 30.0, got {}",
        size
    );

    let size2 = fish2.get_size_mm(1.0);
    assert!(
        (size2 - 40.0).abs() < 0.001,
        "Expected size2 to be approximately 40.0, got {}",
        size2
    );

    let size3 = fish3.get_size_mm(1.0);
    assert!(
        (size3 - 25.0).abs() < 0.001,
        "Expected size3 to be approximately 25.0, got {}",
        size3
    );

    let size3_accelerated = fish3.get_size_mm(3.0);
    assert!(
        (size3_accelerated - 35.0).abs() < 0.001,
        "Expected size3 to be approximately 35.0, got {}",
        size3
    );
}

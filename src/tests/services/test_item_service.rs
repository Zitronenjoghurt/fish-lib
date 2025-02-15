use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::item_data::ItemData;
use crate::enums::item_category::ItemCategory;
use crate::game::service_provider::ServiceProviderInterface;
use crate::models::item::attributes_container::ItemAttributesContainer;
use crate::models::item::properties_container::{
    ItemPropertiesContainer, ItemPropertiesContainerInterface,
};
use crate::models::item::{ItemInterface, NewItem};
use crate::tests::mock::mock_service_provider;
use std::collections::HashMap;
use std::sync::Arc;

const BAIT_ID: i32 = 1;
const UNIQUE_ROD_ID: i32 = 2;
const NON_UNIQUE_ROD_ID: i32 = 3;

fn mock_config() -> Arc<dyn ConfigInterface> {
    let bait = ItemData {
        name: "Bait".to_string(),
        attributes: ItemAttributesContainer::new()
            .with_bait(1)
            .with_purchasable(20),
        default_properties: ItemPropertiesContainer::new().with_stackable(1),
        ..Default::default()
    };

    let unique_rod = ItemData {
        name: "Unique Rod".to_string(),
        attributes: ItemAttributesContainer::new().with_rod(1),
        default_properties: ItemPropertiesContainer::new().with_usage(0),
        ..Default::default()
    };

    let non_unique_rod = ItemData {
        name: "Stackable Rod".to_string(),
        max_count: 2,
        attributes: ItemAttributesContainer::new().with_rod(1),
        default_properties: ItemPropertiesContainer::new().with_usage(0),
        ..Default::default()
    };

    let item_data_map = HashMap::from([
        (BAIT_ID, bait),
        (UNIQUE_ROD_ID, unique_rod),
        (NON_UNIQUE_ROD_ID, non_unique_rod),
    ]);

    Config::builder().items(item_data_map).build().unwrap()
}

fn create_new_item(sp: &Arc<dyn ServiceProviderInterface>, user_id: i64, item_id: i32) -> NewItem {
    let item = sp.config().get_item_data(item_id).unwrap();
    NewItem {
        user_id,
        type_id: item_id,
        properties: item.default_properties.clone(),
    }
}

#[test]
fn test_add_new_item() {
    let config = mock_config();
    let sp = mock_service_provider(config.clone());

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    // Stackable
    let no_bait = sp
        .item_repository()
        .find_by_type_and_user(BAIT_ID, user.id)
        .unwrap();
    assert!(no_bait.is_empty());

    let new_bait1 = create_new_item(&sp, user.id, BAIT_ID);
    sp.item_service().add_new_item(new_bait1, &user).unwrap();

    let found_baits1 = sp
        .item_repository()
        .find_by_type_and_user(BAIT_ID, user.id)
        .unwrap();
    assert_eq!(found_baits1.len(), 1);

    let fond_bait1 = found_baits1.first().unwrap();
    assert!(fond_bait1.attributes(config.clone()).unwrap().is_bait());
    assert_eq!(fond_bait1.get_count(), Some(1));

    let mut new_bait2 = create_new_item(&sp, user.id, BAIT_ID);
    new_bait2.properties.on_add(4);
    sp.item_service().add_new_item(new_bait2, &user).unwrap();

    let found_baits2 = sp
        .item_repository()
        .find_by_type_and_user(BAIT_ID, user.id)
        .unwrap();
    assert_eq!(found_baits2.len(), 1);

    let found_bait2 = found_baits2.first().unwrap();
    assert!(found_bait2.attributes(config.clone()).unwrap().is_bait());
    assert_eq!(found_bait2.get_count(), Some(6));

    // Unstackable & not unique
    let no_non_unique = sp
        .item_repository()
        .find_by_type_and_user(NON_UNIQUE_ROD_ID, user.id)
        .unwrap();
    assert!(no_non_unique.is_empty());

    let new_non_unique1 = create_new_item(&sp, user.id, NON_UNIQUE_ROD_ID);
    sp.item_service()
        .add_new_item(new_non_unique1, &user)
        .unwrap();

    let found_non_uniques1 = sp
        .item_repository()
        .find_by_type_and_user(NON_UNIQUE_ROD_ID, user.id)
        .unwrap();
    assert_eq!(found_non_uniques1.len(), 1);

    let found_non_unique = found_non_uniques1.first().unwrap();
    assert!(found_non_unique
        .attributes(config.clone())
        .unwrap()
        .is_rod());
    assert_eq!(found_non_unique.get_times_used(), Some(0));

    let new_non_unique_2 = create_new_item(&sp, user.id, NON_UNIQUE_ROD_ID);
    sp.item_service()
        .add_new_item(new_non_unique_2, &user)
        .unwrap();

    let found_non_uniques2 = sp
        .item_repository()
        .find_by_type_and_user(NON_UNIQUE_ROD_ID, user.id)
        .unwrap();
    assert_eq!(found_non_uniques2.len(), 2);

    let new_non_unique_3 = create_new_item(&sp, user.id, NON_UNIQUE_ROD_ID);
    let error = sp
        .item_service()
        .add_new_item(new_non_unique_3, &user)
        .unwrap_err();
    if let Some(resource_error) = error.as_resource_error() {
        assert!(resource_error.is_item_max_count_exceeded());
        assert_eq!(resource_error.get_item_type_id(), Some(NON_UNIQUE_ROD_ID));
        assert_eq!(resource_error.get_external_id(), Some(user.external_id));
    } else {
        panic!("Expected ItemMaxCountExceeded error")
    }

    // Unstackable & unique
    let no_unique = sp
        .item_repository()
        .find_by_type_and_user(UNIQUE_ROD_ID, user.id)
        .unwrap();
    assert!(no_unique.is_empty());

    let new_unique1 = create_new_item(&sp, user.id, UNIQUE_ROD_ID);
    sp.item_service().add_new_item(new_unique1, &user).unwrap();

    let found_uniques = sp
        .item_repository()
        .find_by_type_and_user(UNIQUE_ROD_ID, user.id)
        .unwrap();
    assert_eq!(found_uniques.len(), 1);

    let found_unique = found_uniques.first().unwrap();
    assert!(found_unique.attributes(config.clone()).unwrap().is_rod());
    assert_eq!(found_unique.get_times_used(), Some(0));

    let new_unique2 = create_new_item(&sp, user.id, UNIQUE_ROD_ID);
    let error = sp
        .item_service()
        .add_new_item(new_unique2, &user)
        .unwrap_err();
    if let Some(resource_error) = error.as_resource_error() {
        assert!(resource_error.is_item_max_count_exceeded());
        assert_eq!(resource_error.get_item_type_id(), Some(UNIQUE_ROD_ID));
        assert_eq!(resource_error.get_external_id(), Some(user.external_id));
    } else {
        panic!("Expected ItemMaxCountExceeded error")
    }
}

#[test]
fn test_create_and_save_item() {
    let config = mock_config();
    let sp = mock_service_provider(config.clone());

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let rod_data = config.get_item_data(NON_UNIQUE_ROD_ID).unwrap();
    let rod_item = sp
        .item_service()
        .create_and_save_item(rod_data.clone(), &user)
        .unwrap();
    assert!(rod_item.attributes(config.clone()).unwrap().is_rod());

    let bait_data = config.get_item_data(BAIT_ID).unwrap();
    let bait_item = sp
        .item_service()
        .create_and_save_item_with_count(bait_data, &user, 5)
        .unwrap();
    assert!(bait_item.attributes(config.clone()).unwrap().is_bait());
    assert_eq!(bait_item.get_count(), Some(5));

    let error = sp
        .item_service()
        .create_and_save_item_with_count(rod_data, &user, 5)
        .unwrap_err();
    if let Some(resource_error) = error.as_resource_error() {
        assert!(resource_error.is_item_unstackable());
        assert_eq!(resource_error.get_item_type_id(), Some(NON_UNIQUE_ROD_ID));
    }
}

#[test]
fn test_get_inventory() {
    let sp = mock_service_provider(mock_config());
    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let item_data = sp.item_service().get_item_data(UNIQUE_ROD_ID).unwrap();
    let rod = sp
        .item_service()
        .create_and_save_item(item_data, &user)
        .unwrap();

    let inventory = sp.item_service().get_inventory(&user).unwrap();
    let items = inventory.get_items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0], rod);

    let rod_items = inventory.get_items_by_category(ItemCategory::Rod);
    assert_eq!(rod_items.len(), 1);
    assert_eq!(rod_items[0], &rod);

    let no_items = inventory.get_items_by_category(ItemCategory::Bait);
    assert_eq!(no_items.len(), 0);
}

use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::item_data::ItemData;
use crate::game::service_provider::ServiceProviderInterface;
use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface, RodProperties};
use crate::models::item::{Item, NewItem};
use crate::tests::mock::mock_service_provider;
use std::collections::HashMap;
use std::sync::Arc;

fn mock_config() -> Arc<dyn ConfigInterface> {
    let item = ItemData {
        name: "Roddie the Rod".to_string(),
        default_properties: ItemProperties::Rod(RodProperties { times_used: 0 }),
        ..Default::default()
    };

    let item_data_map = HashMap::from([(1, item)]);
    Config::builder().items(item_data_map).build().unwrap()
}

fn create_and_save_item(
    sp: &Arc<dyn ServiceProviderInterface>,
    user_id: i64,
    item_id: i32,
) -> Item {
    let item = sp.config().get_item_data(item_id).unwrap();
    let new_item = NewItem {
        user_id,
        type_id: item_id,
        properties: item.default_properties.clone(),
    };
    sp.item_repository().create(new_item).unwrap()
}

#[test]
fn test_find() {
    let sp = mock_service_provider(mock_config());

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let item = create_and_save_item(&sp, user.id, 1);

    let found_item = sp.item_repository().find(item.id).unwrap().unwrap();
    assert_eq!(item, found_item);
}

#[test]
fn test_save() {
    let sp = mock_service_provider(mock_config());

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let mut item = create_and_save_item(&sp, user.id, 1);
    item.increment_times_used();

    sp.item_repository().save(item.clone()).unwrap();
    let found_item = sp.item_repository().find(item.id).unwrap().unwrap();
    assert_eq!(found_item.get_times_used(), Some(1));
    assert_eq!(item.id, found_item.id);
}

#[test]
fn test_delete() {
    let sp = mock_service_provider(mock_config());

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let item = create_and_save_item(&sp, user.id, 1);

    let found_item = sp.item_repository().find(item.id).unwrap().unwrap();
    assert_eq!(item, found_item);

    let found_item_id = found_item.id;
    sp.item_repository().delete(found_item).unwrap();
    assert_eq!(sp.item_repository().find(found_item_id).unwrap(), None);
}

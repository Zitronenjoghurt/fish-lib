use crate::config::ConfigInterface;
use crate::data::item_data::ItemData;
use crate::dto::inventory::Inventory;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::repositories::item_repository::ItemRepositoryInterface;
use crate::models::item::properties_container::ItemPropertiesContainerInterface;
use crate::models::item::{Item, ItemEventResult, ItemEventSuccess, NewItem};
use crate::models::user::User;
use std::sync::Arc;

pub trait ItemServiceInterface: Send + Sync {
    fn get_item_data(&self, type_id: i32) -> GameResult<Arc<ItemData>>;
    fn add_new_item(&self, item: NewItem, user: &User) -> GameResult<Item>;
    fn create_and_save_item(&self, item_data: Arc<ItemData>, user: &User) -> GameResult<Item>;
    fn create_and_save_item_with_count(
        &self,
        item_data: Arc<ItemData>,
        user: &User,
        count: u64,
    ) -> GameResult<Item>;
    fn manipulate(
        &self,
        item: Item,
        function: Box<dyn Fn(&mut Item) -> ItemEventResult>,
    ) -> GameResult<ItemEventSuccess>;
    fn get_inventory(&self, user: &User) -> GameResult<Inventory>;
}

pub struct ItemService {
    config: Arc<dyn ConfigInterface>,
    item_repository: Arc<dyn ItemRepositoryInterface>,
}

impl ItemService {
    pub fn new(
        config: Arc<dyn ConfigInterface>,
        item_repository: Arc<dyn ItemRepositoryInterface>,
    ) -> ItemService {
        ItemService {
            config,
            item_repository,
        }
    }
}

impl ItemServiceInterface for ItemService {
    fn get_item_data(&self, type_id: i32) -> GameResult<Arc<ItemData>> {
        self.config
            .get_item_data(type_id)
            .ok_or(GameResourceError::item_not_found(type_id).into())
    }

    fn add_new_item(&self, new_item: NewItem, user: &User) -> GameResult<Item> {
        let item_data = self.get_item_data(new_item.type_id)?;
        let existing_items = self
            .item_repository
            .find_by_type_and_user(new_item.type_id, user.id)?;

        let max_count = item_data.max_count;
        let has_count = existing_items.len() as u32;

        let count_max_exceeded = max_count > 1 && has_count >= max_count;
        let count_unique_exceeded = max_count == 1 && has_count > 0 && !item_data.is_stackable();
        if count_max_exceeded || count_unique_exceeded {
            return Err(GameResourceError::item_max_count_exceeded(
                new_item.type_id,
                user.external_id,
            )
            .into());
        };

        let item = if max_count > 1 || has_count == 0 || !item_data.is_stackable() {
            self.item_repository.create(new_item)?
        } else {
            let amount =
                new_item
                    .properties
                    .get_count()
                    .ok_or(GameResourceError::item_unstackable(
                        new_item.type_id,
                        "New item has no count property",
                    ))?;

            let mut item_to_edit =
                existing_items
                    .first()
                    .cloned()
                    .ok_or(GameResourceError::item_unstackable(
                        new_item.type_id,
                        "Did not find any item to add the new item's amount to",
                    ))?;

            item_to_edit.add(amount)?;
            self.item_repository.save(item_to_edit)?
        };

        Ok(item)
    }

    fn create_and_save_item(&self, item_data: Arc<ItemData>, user: &User) -> GameResult<Item> {
        let new_item = NewItem::new(user.id, item_data.clone());
        self.add_new_item(new_item, user)
    }

    fn create_and_save_item_with_count(
        &self,
        item_data: Arc<ItemData>,
        user: &User,
        count: u64,
    ) -> GameResult<Item> {
        let mut new_item = NewItem::new(user.id, item_data.clone());
        if !item_data.is_stackable() {
            return Err(GameResourceError::item_unstackable(
                item_data.id,
                "count provided on item creation, but item is unstackable",
            )
            .into());
        } else {
            new_item.properties.set_count(count);
        }
        self.add_new_item(new_item, user)
    }

    fn manipulate(
        &self,
        mut item: Item,
        function: Box<dyn Fn(&mut Item) -> ItemEventResult>,
    ) -> GameResult<ItemEventSuccess> {
        let success = function(&mut item)?;

        if success.consume {
            self.item_repository.delete(item)?;
        } else {
            self.item_repository.save(item)?;
        }

        Ok(success)
    }

    fn get_inventory(&self, user: &User) -> GameResult<Inventory> {
        let items = self.item_repository.find_by_user(user.id)?;
        Ok(Inventory::new(items))
    }
}

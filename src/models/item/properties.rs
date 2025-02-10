use crate::enums::item_type::ItemType;
use crate::models::item::components::usage_count::UsageComponent;
use crate::models::item::components::{ItemComponent, ItemComponentType};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Jsonb;
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct ItemProperties {
    item_type: ItemType,
    components: HashMap<ItemComponentType, ItemComponent>,
}

impl ItemProperties {
    pub fn new(item_type: ItemType) -> Self {
        Self {
            item_type,
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: ItemComponent) {
        match component {
            ItemComponent::Usage(_) => self.components.insert(ItemComponentType::Usage, component),
        };
    }

    pub fn with_usage_count(mut self, count: u64) -> Self {
        let component = ItemComponent::usage(count);
        self.add_component(component);
        self
    }
}

impl ItemPropertiesInterface for ItemProperties {
    fn get_item_type(&self) -> ItemType {
        self.item_type
    }

    fn get_usage_component(&self) -> Option<&UsageComponent> {
        match self.components.get(&ItemComponentType::Usage) {
            Some(ItemComponent::Usage(count)) => Some(count),
            None => None,
        }
    }

    fn get_usage_component_mut(&mut self) -> Option<&mut UsageComponent> {
        match self.components.get_mut(&ItemComponentType::Usage) {
            Some(ItemComponent::Usage(count)) => Some(count),
            None => None,
        }
    }
}

pub trait ItemPropertiesInterface {
    fn get_item_type(&self) -> ItemType;
    fn get_usage_component(&self) -> Option<&UsageComponent>;
    fn get_usage_component_mut(&mut self) -> Option<&mut UsageComponent>;

    fn is_none(&self) -> bool {
        self.get_item_type() == ItemType::None
    }

    fn is_rod(&self) -> bool {
        self.get_item_type() == ItemType::Rod
    }

    // Component-specific variable access
    fn get_times_used(&self) -> Option<u64> {
        self.get_usage_component()
            .map(|count| count.get_times_used())
    }

    // Orchestration between different components
    fn is_usable(&self) -> bool {
        self.get_usage_component().is_some()
    }

    fn do_use(&mut self) -> bool {
        if !self.is_usable() {
            return false;
        }

        if let Some(usage) = self.get_usage_component_mut() {
            usage.do_use()
        };

        true
    }
}

impl ToSql<Jsonb, Pg> for ItemProperties {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        ToSql::<Jsonb, Pg>::to_sql(&value, &mut out.reborrow())
    }
}

impl FromSql<Jsonb, Pg> for ItemProperties {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}

use crate::models::item::properties::stackable::StackableComponent;
use crate::models::item::properties::usage_count::UsageComponent;
use crate::models::item::properties::{ItemProperties, ItemPropertiesType};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Jsonb;
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone, PartialEq, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct ItemPropertiesContainer {
    components: HashMap<ItemPropertiesType, ItemProperties>,
}

impl ItemPropertiesContainer {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn get_properties_types(&self) -> HashSet<ItemPropertiesType> {
        self.components.keys().copied().collect()
    }

    pub fn add_properties(&mut self, properties: ItemProperties) {
        match properties {
            ItemProperties::Stackable(_) => self
                .components
                .insert(ItemPropertiesType::Stackable, properties),
            ItemProperties::Usage(_) => self
                .components
                .insert(ItemPropertiesType::Usage, properties),
        };
    }

    pub fn remove_properties(&mut self, properties_type: ItemPropertiesType) {
        self.components.remove(&properties_type);
    }

    pub fn with_stackable(mut self, count: u64) -> Self {
        let component = ItemProperties::stackable(count);
        self.add_properties(component);
        self
    }

    pub fn with_usage(mut self, count: u64) -> Self {
        let component = ItemProperties::usage(count);
        self.add_properties(component);
        self
    }
}

impl ItemPropertiesContainerInterface for ItemPropertiesContainer {
    fn get_properties(&self) -> &HashMap<ItemPropertiesType, ItemProperties> {
        &self.components
    }

    fn get_properties_mut(&mut self) -> &mut HashMap<ItemPropertiesType, ItemProperties> {
        &mut self.components
    }
}

pub trait ItemPropertiesContainerInterface {
    fn get_properties(&self) -> &HashMap<ItemPropertiesType, ItemProperties>;
    fn get_properties_mut(&mut self) -> &mut HashMap<ItemPropertiesType, ItemProperties>;

    fn get_stackable_properties(&self) -> Option<&StackableComponent> {
        match self.get_properties().get(&ItemPropertiesType::Stackable) {
            Some(ItemProperties::Stackable(stackable)) => Some(stackable),
            Some(_) | None => None,
        }
    }

    fn get_stackable_properties_mut(&mut self) -> Option<&mut StackableComponent> {
        match self
            .get_properties_mut()
            .get_mut(&ItemPropertiesType::Stackable)
        {
            Some(ItemProperties::Stackable(stackable)) => Some(stackable),
            Some(_) | None => None,
        }
    }

    fn get_usage_properties(&self) -> Option<&UsageComponent> {
        match self.get_properties().get(&ItemPropertiesType::Usage) {
            Some(ItemProperties::Usage(usage)) => Some(usage),
            Some(_) | None => None,
        }
    }
    fn get_usage_properties_mut(&mut self) -> Option<&mut UsageComponent> {
        match self
            .get_properties_mut()
            .get_mut(&ItemPropertiesType::Usage)
        {
            Some(ItemProperties::Usage(usage)) => Some(usage),
            Some(_) | None => None,
        }
    }

    // Properties-existence functions
    fn is_stackable(&self) -> bool {
        self.get_stackable_properties().is_some()
    }

    fn has_usage(&self) -> bool {
        self.get_usage_properties().is_some()
    }

    // Properties-specific variable access
    fn get_count(&self) -> Option<u64> {
        self.get_stackable_properties()
            .map(|stackable| stackable.get_count())
    }

    fn get_times_used(&self) -> Option<u64> {
        self.get_usage_properties()
            .map(|usage| usage.get_times_used())
    }

    // Properties events
    fn on_use(&mut self, times: u64) {
        self.get_properties_mut()
            .values_mut()
            .for_each(|component| component.on_use(times))
    }

    fn on_add(&mut self, amount: u64) {
        self.get_properties_mut()
            .values_mut()
            .for_each(|component| component.on_add(amount));
    }

    fn on_remove(&mut self, amount: u64) {
        self.get_properties_mut()
            .values_mut()
            .for_each(|component| component.on_remove(amount));
    }

    fn should_consume(&self) -> bool {
        self.get_properties()
            .values()
            .any(|component| component.should_consume())
    }
}

impl ToSql<Jsonb, Pg> for ItemPropertiesContainer {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        ToSql::<Jsonb, Pg>::to_sql(&value, &mut out.reborrow())
    }
}

impl FromSql<Jsonb, Pg> for ItemPropertiesContainer {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}

// Serde
impl<'de> Deserialize<'de> for ItemPropertiesContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            components: HashMap<String, serde_json::Value>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let mut container = ItemPropertiesContainer::new();

        for (type_str, component_value) in helper.components {
            let (component_type, component) = match type_str.as_str() {
                "Usage" => {
                    let usage = serde_json::from_value(component_value)
                        .map_err(serde::de::Error::custom)?;
                    (ItemPropertiesType::Usage, ItemProperties::Usage(usage))
                }
                "Stackable" => {
                    let stackable = serde_json::from_value(component_value)
                        .map_err(serde::de::Error::custom)?;
                    (
                        ItemPropertiesType::Stackable,
                        ItemProperties::Stackable(stackable),
                    )
                }
                _ => continue,
            };

            container.components.insert(component_type, component);
        }

        Ok(container)
    }
}

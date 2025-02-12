use crate::models::item::components::stackable::StackableComponent;
use crate::models::item::components::usage_count::UsageComponent;
use crate::models::item::components::{ItemComponent, ItemComponentType};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Jsonb;
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone, PartialEq, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct ItemProperties {
    components: HashMap<ItemComponentType, ItemComponent>,
}

impl ItemProperties {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn get_component_types(&self) -> HashSet<ItemComponentType> {
        self.components.keys().copied().collect()
    }

    pub fn add_component(&mut self, component: ItemComponent) {
        match component {
            ItemComponent::Stackable(_) => self
                .components
                .insert(ItemComponentType::Stackable, component),
            ItemComponent::Usage(_) => self.components.insert(ItemComponentType::Usage, component),
        };
    }

    pub fn remove_component(&mut self, component_type: ItemComponentType) {
        self.components.remove(&component_type);
    }

    pub fn with_stackable(mut self, count: u64) -> Self {
        let component = ItemComponent::stackable(count);
        self.add_component(component);
        self
    }

    pub fn with_usage(mut self, count: u64) -> Self {
        let component = ItemComponent::usage(count);
        self.add_component(component);
        self
    }
}

impl ItemPropertiesInterface for ItemProperties {
    fn get_components(&self) -> &HashMap<ItemComponentType, ItemComponent> {
        &self.components
    }

    fn get_components_mut(&mut self) -> &mut HashMap<ItemComponentType, ItemComponent> {
        &mut self.components
    }
}

pub trait ItemPropertiesInterface {
    fn get_components(&self) -> &HashMap<ItemComponentType, ItemComponent>;
    fn get_components_mut(&mut self) -> &mut HashMap<ItemComponentType, ItemComponent>;

    fn get_stackable_component(&self) -> Option<&StackableComponent> {
        match self.get_components().get(&ItemComponentType::Stackable) {
            Some(ItemComponent::Stackable(stackable)) => Some(stackable),
            Some(_) | None => None,
        }
    }
    fn get_stackable_component_mut(&mut self) -> Option<&mut StackableComponent> {
        match self
            .get_components_mut()
            .get_mut(&ItemComponentType::Stackable)
        {
            Some(ItemComponent::Stackable(stackable)) => Some(stackable),
            Some(_) | None => None,
        }
    }

    fn get_usage_component(&self) -> Option<&UsageComponent> {
        match self.get_components().get(&ItemComponentType::Usage) {
            Some(ItemComponent::Usage(usage)) => Some(usage),
            Some(_) | None => None,
        }
    }
    fn get_usage_component_mut(&mut self) -> Option<&mut UsageComponent> {
        match self.get_components_mut().get_mut(&ItemComponentType::Usage) {
            Some(ItemComponent::Usage(usage)) => Some(usage),
            Some(_) | None => None,
        }
    }

    // Component-existence functions
    fn is_stackable(&self) -> bool {
        self.get_stackable_component().is_some()
    }

    fn has_usage(&self) -> bool {
        self.get_stackable_component().is_some()
    }

    // Component-specific variable access
    fn get_count(&self) -> Option<u64> {
        self.get_stackable_component()
            .map(|stackable| stackable.get_count())
    }

    fn get_times_used(&self) -> Option<u64> {
        self.get_usage_component()
            .map(|usage| usage.get_times_used())
    }

    // Component events
    fn on_use(&mut self, times: u64) {
        self.get_components_mut()
            .values_mut()
            .for_each(|component| component.on_use(times))
    }

    fn on_add(&mut self, amount: u64) {
        self.get_components_mut()
            .values_mut()
            .for_each(|component| component.on_add(amount));
    }

    fn on_remove(&mut self, amount: u64) {
        self.get_components_mut()
            .values_mut()
            .for_each(|component| component.on_remove(amount));
    }

    fn should_consume(&self) -> bool {
        self.get_components()
            .values()
            .any(|component| component.should_consume())
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

// Serde
impl<'de> Deserialize<'de> for ItemProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            components: HashMap<String, serde_json::Value>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let mut properties = ItemProperties::new();

        for (type_str, component_value) in helper.components {
            let (component_type, component) = match type_str.as_str() {
                "Usage" => {
                    let usage = serde_json::from_value(component_value)
                        .map_err(serde::de::Error::custom)?;
                    (ItemComponentType::Usage, ItemComponent::Usage(usage))
                }
                "Stackable" => {
                    let stackable = serde_json::from_value(component_value)
                        .map_err(serde::de::Error::custom)?;
                    (
                        ItemComponentType::Stackable,
                        ItemComponent::Stackable(stackable),
                    )
                }
                _ => continue,
            };

            properties.components.insert(component_type, component);
        }

        Ok(properties)
    }
}

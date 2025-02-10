use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Jsonb;
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct RodProperties {
    pub times_used: i64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
#[serde(tag = "type")]
pub enum ItemProperties {
    #[default]
    None,
    Rod(RodProperties),
}

impl ItemPropertiesInterface for ItemProperties {
    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    fn is_rod(&self) -> bool {
        matches!(self, Self::Rod(_))
    }

    fn as_rod(&self) -> Option<&RodProperties> {
        match self {
            Self::Rod(rod) => Some(rod),
            _ => None,
        }
    }

    fn get_times_used(&self) -> Option<i64> {
        match self {
            Self::Rod(rod) => Some(rod.times_used),
            _ => None,
        }
    }

    fn increment_times_used(&mut self) {
        if let Self::Rod(rod) = self {
            rod.times_used += 1
        }
    }
}

pub trait ItemPropertiesInterface {
    fn is_none(&self) -> bool;
    fn is_rod(&self) -> bool;
    fn as_rod(&self) -> Option<&RodProperties>;
    fn get_times_used(&self) -> Option<i64>;
    fn increment_times_used(&mut self);
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

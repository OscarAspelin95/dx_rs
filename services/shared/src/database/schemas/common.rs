use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::sql::Thing;

use crate::database::DatabaseError;

/// The thing is that surrealdb when returning an id from a query,
/// what we get is a RecordId, which contains a bunch of stuff we don't
/// necessarily want. Instead, we implement a custom deserializer that
/// extracts what we want, which is the record_id and table_name only.
#[derive(Serialize, Debug)]
pub struct SimpleRecordId {
    record_id: String,
    table_name: String,
}

impl SimpleRecordId {
    pub fn formatted_id(&self) -> String {
        format!("{}:{}", self.table_name, self.record_id)
    }

    pub fn surrealdb_id(&self) -> Result<Thing, DatabaseError> {
        let surrealdb_id = self
            .formatted_id()
            .parse::<Thing>()
            .map_err(|_| DatabaseError::RecordIdConversionError)?;

        Ok(surrealdb_id)
    }
}

impl<'de> Deserialize<'de> for SimpleRecordId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Let serde do the heavy work of deserializing data into RecordId.
        let surreal_id = RecordId::deserialize(deserializer)?;

        let record_id = surreal_id.key().to_string();
        let table_name = surreal_id.table().to_string();

        Ok(Self {
            record_id: record_id,
            table_name: table_name,
        })
    }
}

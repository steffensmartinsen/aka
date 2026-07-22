use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// BTreeMap keeps aliases sorted alphabetically in the JSON
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AliasStore {
    pub aliases: BTreeMap<String, Alias>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alias {
    pub command: String,
    // skip_serializing_if omits the field from Json when None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
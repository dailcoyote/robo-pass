use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Hash, Serialize, Deserialize, Clone)]
pub struct Keeper {
    pub url: String,
    pub username: String,
    pub password: String,
}

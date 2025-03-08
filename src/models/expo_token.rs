use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ExpoToken {
    pub token: String,
}

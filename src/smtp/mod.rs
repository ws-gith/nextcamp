use serde::{Deserialize, Serialize};

#[derive(new, Debug, Deserialize, Serialize)]
pub struct Smtp {
    name: String,
    host: String,
    port: i64,
    username: String,
    password: String,
}

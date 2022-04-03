use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct User {
        username: String,
        email: String,
        hash: String,
        salt: String,
    }
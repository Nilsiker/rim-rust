use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct User {
        id: String,
        username: String,
        email: String,
        hash: String,
        salt: String,
    }
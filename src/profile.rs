pub struct Profile {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub ssl: bool
}

impl Profile {
    pub fn new(user: String, password: String, host: String, port: u32, ssl: bool) -> Self {
        Self { user, password, host, port, ssl }
    }
}
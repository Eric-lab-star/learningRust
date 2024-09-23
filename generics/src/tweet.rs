pub struct Tweet {
    pub username: String,
}

impl Tweet {
    pub fn new(username: String) -> Self {
        Self {
            username,
        }
    }
}

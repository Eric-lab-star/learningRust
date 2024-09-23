
pub struct Tweet {
    pub username: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
      String::from(&self.username)  
    }
}

impl Tweet {
    pub fn new(username: String) -> Self {
        Self {
            username,
        }
    }
}


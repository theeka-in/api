use std::sync::Arc;

#[derive(Debug)]
pub struct UsersService {}

impl UsersService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub fn hello(&self, name: &str) -> String {
        format!("hello {}", name)
    }
}

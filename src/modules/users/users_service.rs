#[derive(Clone, Debug)]
pub struct UsersService {}

impl UsersService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hello(&self, name: &str) -> String {
        format!("hello {}", name)
    }
}

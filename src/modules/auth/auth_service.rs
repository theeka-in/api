use crate::modules::users::UsersService;

#[derive(Clone, Debug)]
pub struct AuthService {
    users_service: UsersService,
}

impl AuthService {
    pub fn new(users_service: UsersService) -> Self {
        Self { users_service }
    }

    pub fn hello_from_auth(&self, name: &str) -> String {
        format!("{} from auth", self.users_service.hello(name))
    }
}

use fast_common::base::base_service::BaseService;
use fast_common::models::user::User;

pub struct UserService {}

impl BaseService for UserService {
    type Model = User;
    type Mapper = UserMapper;
}

use fast_common::base::base_mapper::BaseMapper;
use fast_common::models::user::User;

pub struct UserMapper {}

impl BaseMapper for UserMapper {
    type M = User;
}

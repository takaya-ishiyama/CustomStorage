mod entity;
mod value_object;

use entity::UserEntity;
use value_object::UserValueObject;
pub struct User {
    entity: UserEntity,
    value_object: UserValueObject,
}

impl User {
    pub fn new(name: String, password: String) -> Self {
        match UserValueObject::new(name, password) {
            Ok(value_object) => User {
                entity: UserEntity::new(),
                value_object,
            },
            Err(e) => panic!("入力値に誤りがある: {}", e),
        }
    }
}

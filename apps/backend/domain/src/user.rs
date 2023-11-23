mod entity;
mod value_object;

use entity::UserEntity;
use value_object::UserValueObject;
pub struct User {
    entity: UserEntity,
    value_object: UserValueObject,
}

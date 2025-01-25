use crate::traits::model::Model;

pub trait Repository<T: Model> {
    fn create(new_entity: T::InsertType) -> Result<T, Box<dyn std::error::Error>>;

    fn find(id: T::PrimaryKeyType) -> Result<Option<T>, Box<dyn std::error::Error>>;
    fn save(entity: T) -> Result<T, Box<dyn std::error::Error>>;
    fn delete(entity: &T) -> Result<bool, Box<dyn std::error::Error>>;
}
